use super::*;

#[derive(Clone)]
pub struct Options {
    pub pixel_size: f32,
    pub max_distance: f32,
    // TODO: specify a set of glyphs
}

impl Default for Options {
    fn default() -> Self {
        Self {
            pixel_size: 64.0,
            max_distance: 0.25,
        }
    }
}

#[derive(Debug, Clone, ugli::Vertex)]
pub struct GlyphInstance {
    pub i_pos: vec2<f32>,
    pub i_size: vec2<f32>,
    pub i_uv_pos: vec2<f32>,
    pub i_uv_size: vec2<f32>,
}

#[derive(Debug)]
struct GlyphMetrics {
    uv: Aabb2<f32>,
    pos: Aabb2<f32>,
}

#[derive(Debug)]
struct Glyph {
    metrics: Option<GlyphMetrics>,
    advance_x: f32,
}

pub struct Ttf {
    ugli: Ugli,
    sdf_program: Rc<ugli::Program>,
    program: Rc<ugli::Program>,
    glyphs: HashMap<char, Glyph>,
    atlas: ugli::Texture,
    max_distance: f32,
    ascender: f32,
    descender: f32,
    line_gap: f32,
}

impl Ttf {
    pub fn new(geng: &Geng, data: &[u8], options: Options) -> anyhow::Result<Self> {
        Self::new_with(geng.ugli(), geng.shader_lib(), data, options)
    }
    pub(crate) fn new_with(
        ugli: &Ugli,
        shader_lib: &ShaderLib,
        data: &[u8],
        options: Options,
    ) -> anyhow::Result<Self> {
        let face = ttf_parser::Face::from_slice(data, 0)?;
        struct RawGlyph {
            id: ttf_parser::GlyphId,
            code_point: char,
            bounding_box: Option<Aabb2<f32>>,
        }
        let unit_scale = 1.0 / (face.ascender() - face.descender()) as f32;
        let scale = options.pixel_size * unit_scale;
        let mut raw_glyphs = Vec::new();
        let mut found = HashSet::new();
        for subtable in face.tables().cmap.unwrap().subtables {
            if !subtable.is_unicode() {
                continue;
            }
            subtable.codepoints(|code_point| {
                let id = match subtable.glyph_index(code_point) {
                    Some(id) => id,
                    None => return,
                };
                let code_point = match char::from_u32(code_point) {
                    Some(code_point) => code_point,
                    None => return,
                };
                if found.contains(&code_point) {
                    return;
                }
                found.insert(code_point);
                let bounding_box = face.glyph_bounding_box(id).map(|rect| {
                    Aabb2 {
                        min: vec2(rect.x_min, rect.y_min),
                        max: vec2(rect.x_max, rect.y_max),
                    }
                    .map(|x| x as f32 * scale)
                });
                raw_glyphs.push(RawGlyph {
                    id,
                    code_point,
                    bounding_box,
                })
            });
        }
        raw_glyphs.sort_unstable_by_key(|glyph| {
            glyph
                .bounding_box
                .map_or(0, |bb| -bb.height().ceil() as i32)
        });
        let mut glyphs: HashMap<char, Glyph> = HashMap::with_capacity(raw_glyphs.len());
        let mut width = 0;
        let mut x = 0;
        let mut y = 0;
        let mut row_height = 0;
        let renderable_glyphs: Vec<&RawGlyph> = raw_glyphs
            .iter()
            .filter(|g| g.bounding_box.is_some())
            .collect();
        for glyph in &raw_glyphs {
            if glyph.bounding_box.is_none() {
                glyphs.insert(
                    glyph.code_point,
                    Glyph {
                        metrics: None,
                        advance_x: face.glyph_hor_advance(glyph.id).unwrap_or(0) as f32
                            * unit_scale,
                    },
                );
            }
        }
        for (i, glyph) in renderable_glyphs.iter().enumerate() {
            let glyph_pos = glyph
                .bounding_box
                .unwrap()
                .extend_uniform(options.max_distance * options.pixel_size);
            let glyph_size = glyph_pos.size().map(|x| x.ceil() as usize);
            if (y == 0 && i * i >= renderable_glyphs.len())
                || (y > 0 && x > 0 && x + glyph_size.x > width)
            {
                x = 0;
                y += row_height;
                row_height = 0;
            }
            let uv = Aabb2::point(vec2(x, y)).extend_positive(glyph_size);
            x = uv.max.x;
            row_height = row_height.max(uv.height());
            width = width.max(x);
            glyphs.insert(
                glyph.code_point,
                Glyph {
                    metrics: Some(GlyphMetrics {
                        uv: uv.map(|x| x as f32),
                        pos: glyph_pos.map(|x| x / options.pixel_size),
                    }),
                    advance_x: face.glyph_hor_advance(glyph.id).unwrap_or(0) as f32 * unit_scale,
                },
            );
        }
        let height = y + row_height;
        let atlas_size = vec2(width, height);
        for glyph in glyphs.values_mut() {
            if let Some(metrics) = &mut glyph.metrics {
                metrics.uv = metrics.uv.map_bounds(|b| b / atlas_size.map(|x| x as f32));
            }
        }
        let mut atlas = ugli::Texture::new_uninitialized(ugli, atlas_size);
        {
            let mut depth_buffer = ugli::Renderbuffer::new(ugli, atlas_size);
            let mut framebuffer = ugli::Framebuffer::new(
                ugli,
                ugli::ColorAttachment::Texture(&mut atlas),
                ugli::DepthAttachment::RenderbufferWithStencil(&mut depth_buffer),
            );
            let framebuffer = &mut framebuffer;
            ugli::clear(
                framebuffer,
                Some(Rgba::TRANSPARENT_BLACK),
                Some(1.0),
                Some(0),
            );

            #[derive(ugli::Vertex, Copy, Clone)]
            struct Vertex {
                a_pos: vec2<f32>,
                a_dist_pos: vec2<f32>,
            }
            struct Builder {
                distance_mesh: Vec<Vertex>,
                stencil_mesh: Vec<Vertex>,
                pos: vec2<f32>,
                scale: f32,
                offset: vec2<f32>,
                options: Options,
            }
            impl Builder {
                fn new_glyph_at(&mut self, offset: vec2<f32>) {
                    self.offset = offset;
                }
                fn add_triangle_fan(&mut self, mid: Vertex, vs: impl IntoIterator<Item = Vertex>) {
                    use itertools::Itertools;
                    for (a, b) in vs.into_iter().tuple_windows() {
                        self.distance_mesh.push(mid);
                        self.distance_mesh.push(a);
                        self.distance_mesh.push(b);
                    }
                }
                fn add_triangle_fan2(&mut self, vs: impl IntoIterator<Item = Vertex>) {
                    let mut vs = vs.into_iter();
                    let first = vs.next().unwrap();
                    self.add_triangle_fan(first, vs);
                }
                fn add_triangle_fan_loop(
                    &mut self,
                    mid: Vertex,
                    vs: impl IntoIterator<Item = Vertex>,
                ) {
                    let mut vs = vs.into_iter();
                    let v0 = vs.next();
                    self.add_triangle_fan(mid, itertools::chain![v0, vs, v0]);
                }
                fn add_line(&mut self, a: vec2<f32>, b: vec2<f32>) {
                    let radius = self.options.max_distance * self.options.pixel_size;
                    self.stencil_mesh.push(Vertex {
                        a_pos: self.offset,
                        a_dist_pos: vec2::ZERO,
                    });
                    self.stencil_mesh.push(Vertex {
                        a_pos: a,
                        a_dist_pos: vec2::ZERO,
                    });
                    self.stencil_mesh.push(Vertex {
                        a_pos: b,
                        a_dist_pos: vec2::ZERO,
                    });
                    let unit_quad = Aabb2::point(vec2::ZERO).extend_uniform(1.0);
                    let a_quad = Aabb2::point(a).extend_uniform(radius);
                    let b_quad = Aabb2::point(b).extend_uniform(radius);
                    self.add_triangle_fan2(
                        itertools::izip![a_quad.corners(), unit_quad.corners()]
                            .map(|(a_pos, a_dist_pos)| Vertex { a_pos, a_dist_pos }),
                    );
                    self.add_triangle_fan2(
                        itertools::izip![b_quad.corners(), unit_quad.corners()]
                            .map(|(a_pos, a_dist_pos)| Vertex { a_pos, a_dist_pos }),
                    );
                    let n = (b - a).rotate_90().normalize_or_zero() * radius;
                    self.add_triangle_fan2([
                        Vertex {
                            a_pos: a + n,
                            a_dist_pos: vec2(0.0, 1.0),
                        },
                        Vertex {
                            a_pos: b + n,
                            a_dist_pos: vec2(0.0, 1.0),
                        },
                        Vertex {
                            a_pos: b - n,
                            a_dist_pos: vec2(0.0, -1.0),
                        },
                        Vertex {
                            a_pos: a - n,
                            a_dist_pos: vec2(0.0, -1.0),
                        },
                    ]);
                }
            }
            fn quad_bezier(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, t: f32) -> vec2<f32> {
                (1.0 - t).sqr() * p0 + 2.0 * (1.0 - t) * t * p1 + t.sqr() * p2
            }
            fn cubic_bezier(
                p0: vec2<f32>,
                p1: vec2<f32>,
                p2: vec2<f32>,
                p3: vec2<f32>,
                t: f32,
            ) -> vec2<f32> {
                (1.0 - t) * quad_bezier(p0, p1, p2, t) + t * quad_bezier(p1, p2, p3, t)
            }
            const N: usize = 10;
            impl ttf_parser::OutlineBuilder for Builder {
                fn move_to(&mut self, x: f32, y: f32) {
                    self.pos = vec2(x, y) * self.scale + self.offset;
                }
                fn line_to(&mut self, x: f32, y: f32) {
                    let a = self.pos;
                    self.move_to(x, y);
                    let b = self.pos;
                    self.add_line(a, b);
                }
                fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                    // TODO proper math stuff
                    let p0 = self.pos;
                    let p1 = vec2(x1, y1) * self.scale + self.offset;
                    let p2 = vec2(x, y) * self.scale + self.offset;
                    for i in 1..=N {
                        let t = i as f32 / N as f32;
                        let p = quad_bezier(p0, p1, p2, t);
                        self.add_line(self.pos, p);
                        self.pos = p;
                    }
                }
                fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                    // TODO proper math stuff
                    let p0 = self.pos;
                    let p1 = vec2(x1, y1) * self.scale + self.offset;
                    let p2 = vec2(x2, y2) * self.scale + self.offset;
                    let p3 = vec2(x, y) * self.scale + self.offset;
                    for i in 1..=N {
                        let t = i as f32 / N as f32;
                        let p = cubic_bezier(p0, p1, p2, p3, t);
                        self.add_line(self.pos, p);
                        self.pos = p;
                    }
                }
                fn close(&mut self) {
                    // TODO: hm?
                }
            }
            let mut builder = Builder {
                distance_mesh: vec![],
                stencil_mesh: vec![],
                pos: vec2::ZERO,
                scale,
                offset: vec2::ZERO,
                options: options.clone(),
            };
            for glyph in &raw_glyphs {
                if glyph.bounding_box.is_none() {
                    continue;
                }
                builder.new_glyph_at(
                    (glyphs[&glyph.code_point]
                        .metrics
                        .as_ref()
                        .unwrap()
                        .uv
                        .bottom_left()
                        * atlas_size.map(|x| x as f32))
                    .map(|x| x + options.max_distance * options.pixel_size)
                        - glyph.bounding_box.unwrap().bottom_left(),
                );
                face.outline_glyph(glyph.id, &mut builder);
            }
            let line_shader = shader_lib.compile(include_str!("ttf_line.glsl")).unwrap();
            let white_shader = shader_lib.compile(include_str!("white.glsl")).unwrap();
            ugli::draw(
                framebuffer,
                &line_shader,
                ugli::DrawMode::Triangles,
                &ugli::VertexBuffer::new_static(ugli, builder.stencil_mesh),
                ugli::uniforms! {
                    u_framebuffer_size: framebuffer.size(),
                },
                ugli::DrawParameters {
                    stencil_mode: Some(ugli::StencilMode {
                        back_face: ugli::FaceStencilMode {
                            test: ugli::StencilTest {
                                condition: ugli::Condition::Always,
                                reference: 0,
                                mask: 0,
                            },
                            op: ugli::StencilOp::always(ugli::StencilOpFunc::IncrementWrap),
                        },
                        front_face: ugli::FaceStencilMode {
                            test: ugli::StencilTest {
                                condition: ugli::Condition::Always,
                                reference: 0,
                                mask: 0,
                            },
                            op: ugli::StencilOp::always(ugli::StencilOpFunc::DecrementWrap),
                        },
                    }),
                    write_color: false,
                    ..default()
                },
            );
            ugli::draw(
                framebuffer,
                &line_shader,
                ugli::DrawMode::Triangles,
                &ugli::VertexBuffer::new_static(ugli, builder.distance_mesh),
                ugli::uniforms! {
                    u_framebuffer_size: framebuffer.size(),
                },
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::combined(ugli::ChannelBlendMode {
                        src_factor: ugli::BlendFactor::One,
                        dst_factor: ugli::BlendFactor::One,
                        equation: ugli::BlendEquation::Max,
                    })),
                    // depth_func: Some(ugli::DepthFunc::Less),
                    ..default()
                },
            );
            ugli::draw(
                framebuffer,
                &white_shader,
                ugli::DrawMode::TriangleFan,
                &ugli::VertexBuffer::new_static(
                    ugli,
                    Aabb2::point(vec2(0, 0))
                        .extend_positive(framebuffer.size())
                        .corners()
                        .into_iter()
                        .map(|p| Vertex {
                            a_pos: p.map(|x| x as f32),
                            a_dist_pos: vec2::ZERO,
                        })
                        .collect(),
                ),
                ugli::uniforms! {
                    u_framebuffer_size: framebuffer.size(),
                },
                ugli::DrawParameters {
                    stencil_mode: Some(ugli::StencilMode::always(ugli::FaceStencilMode {
                        test: ugli::StencilTest {
                            condition: ugli::Condition::NotEqual,
                            reference: 0,
                            mask: 0xff,
                        },
                        op: ugli::StencilOp::always(ugli::StencilOpFunc::Keep),
                    })),
                    blend_mode: Some(ugli::BlendMode::combined(ugli::ChannelBlendMode {
                        src_factor: ugli::BlendFactor::OneMinusDstColor,
                        dst_factor: ugli::BlendFactor::Zero,
                        equation: ugli::BlendEquation::Add,
                    })),
                    ..default()
                },
            );
        }
        thread_local! { pub static SHADERS: RefCell<Option<[Rc<ugli::Program>; 2]>> = default(); };
        let [program, sdf_program] = SHADERS.with(|shaders| {
            fn map<T, R>(a: &[T; 2], f: impl Fn(&T) -> R) -> [R; 2] {
                let [a, b] = a;
                [f(a), f(b)]
            }
            map(
                shaders.borrow_mut().get_or_insert_with(|| {
                    [
                        shader_lib.compile(include_str!("shader.glsl")).unwrap(),
                        shader_lib.compile(include_str!("sdf.glsl")).unwrap(),
                    ]
                    .map(Rc::new)
                }),
                |shader| Rc::clone(shader),
            )
        });
        Ok(Self {
            ugli: ugli.clone(),
            program,
            sdf_program,
            glyphs,
            atlas,
            max_distance: options.max_distance,
            ascender: face.ascender() as f32 * unit_scale,
            descender: face.descender() as f32 * unit_scale,
            line_gap: face.line_gap() as f32 * unit_scale,
        })
    }

    pub fn max_distance(&self) -> f32 {
        self.max_distance
    }

    pub fn ascender(&self) -> f32 {
        self.ascender
    }

    pub fn descender(&self) -> f32 {
        self.descender
    }

    pub fn line_gap(&self) -> f32 {
        self.line_gap
    }

    pub fn measure_bounding_box(&self, text: &str, align: vec2<TextAlign>) -> Option<Aabb2<f32>> {
        self.draw_with(text, align, |glyphs, _| {
            if glyphs.is_empty() {
                return None;
            }
            Some(Aabb2::points_bounding_box(glyphs.iter().flat_map(
                |glyph| {
                    [
                        glyph.i_pos + vec2(self.max_distance, self.max_distance),
                        glyph.i_pos + glyph.i_size - vec2(self.max_distance, self.max_distance),
                    ]
                },
            )))
        })
    }

    pub fn advance(&self, text: &str) -> f32 {
        let mut x = 0.0;
        for glyph in text.chars().filter_map(move |c| self.glyphs.get(&c)) {
            // TODO: kerning
            x += glyph.advance_x;
        }
        x
    }

    pub fn draw_with<R>(
        &self,
        text: &str,
        align: vec2<TextAlign>,
        f: impl FnOnce(&[GlyphInstance], &ugli::Texture) -> R,
    ) -> R {
        let mut vs = Vec::<GlyphInstance>::new();
        let mut pos = vec2::ZERO;
        let mut size_x: f32 = 0.0;
        let mut line_width: f32 = 0.0;
        let mut line_start = 0;
        for c in text.chars() {
            if c == '\n' {
                for v in &mut vs[line_start..] {
                    v.i_pos.x -= line_width * align.x.0;
                }
                pos.x = 0.0;
                pos.y -= 1.0;
                line_width = 0.0;
                line_start = vs.len();
                continue;
            }
            let Some(glyph) = self.glyphs.get(&c) else { continue };
            // TODO: kerning
            if let Some(metrics) = &glyph.metrics {
                let instance = GlyphInstance {
                    i_pos: pos + metrics.pos.bottom_left(),
                    i_size: metrics.pos.size(),
                    i_uv_pos: metrics.uv.bottom_left(),
                    i_uv_size: metrics.uv.size(),
                };
                line_width =
                    line_width.max(instance.i_pos.x + instance.i_size.x - self.max_distance);
                size_x = size_x.max(line_width);
                vs.push(instance);
            }
            pos.x += glyph.advance_x;
        }
        for v in &mut vs[line_start..] {
            v.i_pos.x -= line_width * align.x.0;
        }
        for v in &mut vs {
            v.i_pos.y += pos.y * align.y.0;
        }
        f(&vs, &self.atlas)
    }

    #[deprecated]
    pub fn measure(&self, text: &str, size: f32) -> Option<Aabb2<f32>> {
        self.measure_bounding_box(text, vec2(TextAlign::LEFT, TextAlign::LEFT))
            .map(|aabb| aabb.map(|x| x * size))
    }

    #[deprecated]
    pub(crate) fn draw_impl(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &(impl AbstractCamera2d + ?Sized),
        transform: mat3<f32>,
        text: &str,
        pos: vec2<f32>,
        size: f32,
        color: Rgba<f32>,
        outline_size: f32,
        outline_color: Rgba<f32>,
    ) {
        let transform = transform * mat3::translate(pos) * mat3::scale_uniform(size);
        self.draw_with(
            text,
            vec2(TextAlign::LEFT, TextAlign::LEFT),
            |glyphs, texture| {
                let framebuffer_size = framebuffer.size();
                ugli::draw(
                    framebuffer,
                    &self.program,
                    ugli::DrawMode::TriangleFan,
                    // TODO: don't create VBs each time
                    ugli::instanced(
                        &ugli::VertexBuffer::new_dynamic(
                            &self.ugli,
                            Aabb2::point(vec2::ZERO)
                                .extend_positive(vec2(1.0, 1.0))
                                .corners()
                                .into_iter()
                                .map(|v| draw_2d::Vertex { a_pos: v })
                                .collect(),
                        ),
                        &ugli::VertexBuffer::new_dynamic(&self.ugli, glyphs.to_vec()),
                    ),
                    (
                        ugli::uniforms! {
                            u_texture: texture,
                            u_model_matrix: transform,
                            u_color: color,
                            u_outline_dist: outline_size / size / self.max_distance,
                            u_outline_color: outline_color,
                        },
                        camera2d_uniforms(camera, framebuffer_size.map(|x| x as f32)),
                    ),
                    ugli::DrawParameters {
                        depth_func: None,
                        blend_mode: Some(ugli::BlendMode::straight_alpha()),
                        ..default()
                    },
                );
            },
        );
    }

    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        text: &str,
        pos: vec2<f32>,
        align: TextAlign,
        size: f32,
        color: Rgba<f32>,
    ) {
        let mut pos = pos;
        for line in text.lines().rev() {
            if let Some(aabb) = self.measure(line, size) {
                self.draw_impl(
                    framebuffer,
                    camera,
                    mat3::identity(),
                    line,
                    vec2(pos.x - aabb.width() * align.0, pos.y),
                    size,
                    color,
                    0.0,
                    Rgba { a: 0.0, ..color },
                );
            }
            pos.y += size;
        }
    }

    pub fn draw_with_outline(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        text: &str,
        pos: vec2<f32>,
        align: TextAlign,
        size: f32,
        color: Rgba<f32>,
        outline_size: f32,
        outline_color: Rgba<f32>,
    ) {
        let mut pos = pos;
        for line in text.lines().rev() {
            if let Some(aabb) = self.measure(line, size) {
                self.draw_impl(
                    framebuffer,
                    camera,
                    mat3::identity(),
                    line,
                    vec2(pos.x - aabb.width() * align.0, pos.y),
                    size,
                    color,
                    outline_size,
                    outline_color,
                );
            }
            pos.y += size;
        }
    }

    pub fn create_text_sdf(
        &self,
        text: &str,
        line_align: TextAlign,
        pixel_size: f32,
    ) -> Option<ugli::Texture> {
        let align = vec2(line_align, TextAlign::BOTTOM);
        let aabb = self.measure_bounding_box(text, align)?;
        let texture_size = (vec2(
            aabb.width() + 2.0 * self.max_distance(),
            text.chars().filter(|c| *c == '\n').count() as f32 + 1.0 + 2.0 * self.max_distance(),
        ) * pixel_size)
            .map(|x| x.ceil() as usize);
        let mut texture = ugli::Texture::new_uninitialized(&self.ugli, texture_size);
        let framebuffer = &mut ugli::Framebuffer::new_color(
            &self.ugli,
            ugli::ColorAttachment::Texture(&mut texture),
        );
        ugli::clear(framebuffer, Some(Rgba::TRANSPARENT_BLACK), None, None);
        self.draw_with(text, align, |glyphs, atlas| {
            ugli::draw(
                framebuffer,
                &self.sdf_program,
                ugli::DrawMode::TriangleFan,
                ugli::instanced(
                    &ugli::VertexBuffer::new_dynamic(
                        &self.ugli,
                        Aabb2::point(vec2::ZERO)
                            .extend_positive(vec2(1.0, 1.0))
                            .corners()
                            .into_iter()
                            .map(|v| draw_2d::Vertex { a_pos: v })
                            .collect(),
                    ),
                    &ugli::VertexBuffer::new_dynamic(&self.ugli, glyphs.to_vec()),
                ),
                ugli::uniforms! {
                    u_texture: atlas,
                    u_matrix: mat3::ortho(aabb.extend_uniform(self.max_distance())),
                },
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::combined(ugli::ChannelBlendMode {
                        src_factor: ugli::BlendFactor::One,
                        dst_factor: ugli::BlendFactor::One,
                        equation: ugli::BlendEquation::Max,
                    })),
                    ..default()
                },
            );
        });
        Some(texture)
    }
}
