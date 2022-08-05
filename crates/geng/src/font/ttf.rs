use super::*;

#[derive(Clone)]
pub struct Options {
    pub size: f32,
    pub max_distance: f32,
    // TODO: not all glyphs
}

impl Default for Options {
    fn default() -> Self {
        Self {
            size: 32.0,
            max_distance: 8.0,
        }
    }
}

#[derive(Debug)]
struct Glyph {
    bounding_box: Option<AABB<usize>>,
}

pub struct Ttf {
    glyphs: HashMap<char, Glyph>,
    pub atlas: ugli::Texture,
}

impl Ttf {
    pub fn new(geng: &Geng, data: &[u8], options: Options) -> anyhow::Result<Self> {
        let face = ttf_parser::Face::from_slice(data, 0)?;
        struct RawGlyph {
            id: ttf_parser::GlyphId,
            code_point: char,
            bounding_box: Option<AABB<f32>>,
        }
        let scale = options.size / (face.ascender() - face.descender()) as f32;
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
                    AABB {
                        x_min: rect.x_min,
                        x_max: rect.x_max,
                        y_min: rect.y_min,
                        y_max: rect.y_max,
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
        for (i, glyph) in renderable_glyphs.iter().enumerate() {
            let glyph_size = glyph
                .bounding_box
                .unwrap()
                .size()
                .map(|x| (x + 2.0 * options.max_distance).ceil() as usize);
            if (y == 0 && i * i >= renderable_glyphs.len())
                || (y > 0 && x > 0 && x + glyph_size.x > width)
            {
                x = 0;
                y += row_height;
                row_height = 0;
            }
            let bounding_box = AABB::point(vec2(x, y)).extend_positive(glyph_size);
            x = bounding_box.x_max;
            row_height = row_height.max(bounding_box.height());
            width = width.max(x);
            glyphs.insert(
                glyph.code_point,
                Glyph {
                    bounding_box: Some(bounding_box),
                },
            );
        }
        let height = y + row_height;
        let mut atlas = ugli::Texture::new_uninitialized(geng.ugli(), vec2(width, height));
        {
            let mut depth_buffer = ugli::Renderbuffer::new(geng.ugli(), atlas.size());
            let mut framebuffer = ugli::Framebuffer::new(
                geng.ugli(),
                ugli::ColorAttachment::Texture(&mut atlas),
                ugli::DepthAttachment::RenderbufferWithStencil(&mut depth_buffer),
            );
            let framebuffer = &mut framebuffer;
            ugli::clear(framebuffer, Some(Rgba::TRANSPARENT_BLACK), Some(1.0));

            #[derive(ugli::Vertex, Copy, Clone)]
            struct Vertex {
                a_pos: Vec3<f32>,
            }
            fn v(a_pos: Vec3<f32>) -> Vertex {
                Vertex { a_pos }
            }
            struct Builder {
                distance_mesh: Vec<Vertex>,
                stencil_mesh: Vec<Vertex>,
                pos: Vec2<f32>,
                scale: f32,
                offset: Vec2<f32>,
                options: Options,
            }
            impl Builder {
                fn new_glyph_at(&mut self, offset: Vec2<f32>) {
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
                fn add_triangle_fan_loop(
                    &mut self,
                    mid: Vertex,
                    vs: impl IntoIterator<Item = Vertex>,
                ) {
                    let mut vs = vs.into_iter();
                    let v0 = vs.next();
                    self.add_triangle_fan(mid, itertools::chain![v0, vs, v0]);
                }
                fn add_line(&mut self, a: Vec2<f32>, b: Vec2<f32>) {
                    self.stencil_mesh.push(v(self.offset.extend(0.0)));
                    self.stencil_mesh.push(v(a.extend(0.0)));
                    self.stencil_mesh.push(v(b.extend(0.0)));
                    let a_quad = AABB::point(a).extend_uniform(self.options.max_distance);
                    let b_quad = AABB::point(b).extend_uniform(self.options.max_distance);
                    self.add_triangle_fan_loop(
                        v(a.extend(0.0)),
                        a_quad.corners().map(|p| v(p.extend(1.0))),
                    );
                    self.add_triangle_fan_loop(
                        v(b.extend(0.0)),
                        b_quad.corners().map(|p| v(p.extend(1.0))),
                    );
                    for (a_corner, b_corner) in itertools::izip![a_quad.corners(), b_quad.corners()]
                    {
                        self.add_triangle_fan(
                            v(a.extend(0.0)),
                            [
                                v(a_corner.extend(1.0)),
                                v(b_corner.extend(1.0)),
                                v(b.extend(0.0)),
                            ],
                        );
                    }
                }
            }
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
                    self.line_to(x1, y1);
                    self.line_to(x, y)
                }
                fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                    self.line_to(x1, y1);
                    self.line_to(x2, y2);
                    self.line_to(x, y);
                }
                fn close(&mut self) {}
            }
            let mut builder = Builder {
                distance_mesh: vec![],
                stencil_mesh: vec![],
                pos: Vec2::ZERO,
                scale,
                offset: Vec2::ZERO,
                options: options.clone(),
            };
            for glyph in &raw_glyphs {
                if glyph.bounding_box.is_none() {
                    continue;
                }
                builder.new_glyph_at(
                    glyphs[&glyph.code_point]
                        .bounding_box
                        .unwrap()
                        .bottom_left()
                        .map(|x| x as f32 + options.max_distance)
                        - glyph.bounding_box.unwrap().bottom_left(),
                );
                face.outline_glyph(glyph.id, &mut builder);
            }
            let line_shader = geng
                .shader_lib()
                .compile(include_str!("ttf_line.glsl"))
                .unwrap();
            ugli::draw(
                framebuffer,
                &line_shader,
                ugli::DrawMode::Triangles,
                &ugli::VertexBuffer::new_static(geng.ugli(), builder.stencil_mesh),
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
                    write_depth: false,
                    ..default()
                },
            );
            ugli::draw(
                framebuffer,
                &line_shader,
                ugli::DrawMode::Triangles,
                &ugli::VertexBuffer::new_static(geng.ugli(), builder.distance_mesh),
                ugli::uniforms! {
                    u_framebuffer_size: framebuffer.size(),
                },
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    ..default()
                },
            );
            ugli::draw(
                framebuffer,
                &line_shader,
                ugli::DrawMode::TriangleFan,
                &ugli::VertexBuffer::new_static(
                    geng.ugli(),
                    AABB::point(vec2(0, 0))
                        .extend_positive(framebuffer.size())
                        .corners()
                        .into_iter()
                        .map(|p| v(p.map(|x| x as f32).extend(-1.0)))
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
                    })),
                    ..default()
                },
            );
        }
        Ok(Self { glyphs, atlas })
    }
}
