use geng::prelude::*;

const SDF_SHADER_SOURCE: &str = "
varying vec2 v_uv;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec2 i_pos;
attribute vec2 i_size;
attribute vec2 i_uv_pos;
attribute vec2 i_uv_size;

uniform mat3 u_matrix;
void main() {
    v_uv = i_uv_pos + a_pos * i_uv_size;
    vec3 pos = u_matrix * vec3(i_pos + a_pos * i_size, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;

void main() {
    gl_FragColor = texture2D(u_texture, v_uv);
}
#endif
";

const SHADER_SOURCE: &str = "
varying vec2 v_uv;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_model_matrix;
void main() {
    v_uv = a_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;

float aa(float edge, float x) {
    float w = length(vec2(dFdx(x), dFdy(x)));
    return smoothstep(edge - w, edge + w, x);
}

void main() {
    //gl_FragColor = texture2D(u_texture, v_uv);
    //gl_FragColor.a = 1.0;
    //return;
    float dist = (texture2D(u_texture, v_uv).x - 0.5) * 2.0;
    float inside = aa(0.0, dist);
    float inside_border = aa(-0.15, dist);
    vec4 color = vec4(1.0, 0.0, 0.0, 1.0);
    vec4 border_color = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 outside_color = vec4(border_color.xyz, 0.0);
    //vec4 outside_color = vec4(0.0, 1.0, 0.0, 1.0);
    gl_FragColor = color * inside + (1.0 - inside) * (border_color * inside_border + outside_color * (1.0 - inside_border));
    //gl_FragColor = color * inside + (1.0 - inside) * vec4(border_color.rgb, -dist);
}
#endif
";

struct State {
    geng: Geng,
    sdf_program: ugli::Program,
    program: ugli::Program,
    font: geng::font::Ttf,
}

impl State {
    fn new(geng: &Geng) -> Self {
        let font = geng::font::Ttf::new(
            geng,
            include_bytes!("../src/font/default.ttf"),
            geng::font::ttf::Options {
                pixel_size: 64.0,
                max_distance: 0.25,
            },
        )
        .unwrap();
        Self {
            geng: geng.clone(),
            font,
            sdf_program: geng.shader_lib().compile(SDF_SHADER_SOURCE).unwrap(),
            program: geng.shader_lib().compile(SHADER_SOURCE).unwrap(),
        }
    }

    fn create_text_sdf(
        &self,
        text: &str,
        font: &geng::Font,
        pixel_size: f32,
    ) -> Option<ugli::Texture> {
        let aabb = font.measure_bounding_box(text)?;
        let texture_size = (vec2(
            aabb.width() + 2.0 * font.max_distance(),
            1.0 + 2.0 * font.max_distance(),
        ) * pixel_size)
            .map(|x| x.ceil() as usize);
        let mut texture = ugli::Texture::new_uninitialized(self.geng.ugli(), texture_size);
        let framebuffer = &mut ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut texture),
        );
        ugli::clear(framebuffer, Some(Rgba::TRANSPARENT_BLACK), None, None);
        font.draw_with(text, |glyphs, atlas| {
            ugli::draw(
                framebuffer,
                &self.sdf_program,
                ugli::DrawMode::TriangleFan,
                ugli::instanced(
                    &ugli::VertexBuffer::new_dynamic(
                        self.geng.ugli(),
                        Aabb2::point(vec2::ZERO)
                            .extend_positive(vec2(1.0, 1.0))
                            .corners()
                            .into_iter()
                            .map(|v| draw_2d::Vertex { a_pos: v })
                            .collect(),
                    ),
                    &ugli::VertexBuffer::new_dynamic(self.geng.ugli(), glyphs.to_vec()),
                ),
                ugli::uniforms! {
                    u_texture: atlas,
                    u_matrix: mat3::ortho(aabb.extend_uniform(font.max_distance())),
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

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        let text = "� О, аутлайн!";
        if let Some(texture) = self.create_text_sdf(text, &self.font, 256.0) {
            ugli::draw(
                framebuffer,
                &self.program,
                ugli::DrawMode::TriangleFan,
                &ugli::VertexBuffer::new_dynamic(
                    self.geng.ugli(),
                    vec![
                        draw_2d::Vertex {
                            a_pos: vec2(0.0, 0.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(1.0, 0.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(1.0, 1.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(0.0, 1.0),
                        },
                    ],
                ),
                (
                    ugli::uniforms! {
                        u_model_matrix: mat3::scale(texture.size().map(|x| x as f32)),
                        u_texture: texture,
                    },
                    geng::camera2d_uniforms(
                        &geng::PixelPerfectCamera,
                        framebuffer.size().map(|x| x as f32),
                    ),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..default()
                },
            );
        }
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Font");
    let state = State::new(&geng);
    geng::run(&geng, state);
}
