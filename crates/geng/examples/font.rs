use geng::prelude::*;

const SHADER_SOURCE: &str = "
varying vec2 v_uv;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec2 i_pos;
attribute vec2 i_size;
attribute vec2 i_uv_pos;
attribute vec2 i_uv_size;

uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
void main() {
    v_uv = i_uv_pos + a_pos * i_uv_size;
    vec3 pos = u_projection_matrix * u_view_matrix * vec3(i_pos + a_pos * i_size, 1.0);
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
    float dist = (texture2D(u_texture, v_uv).x - 0.5) * 2.0;
    float w = length(vec2(dFdx(dist), dFdy(dist)));
    float inside = aa(0.0, dist);
    float inside_border = aa(-0.15, dist);
    vec4 color = vec4(1.0, 0.0, 0.0, 1.0);
    vec4 border_color = vec4(1.0, 1.0, 1.0, 1.0);
    gl_FragColor = color * inside + (1.0 - inside) * (border_color * inside_border + vec4(border_color.xyz, 0.0) * (1.0 - inside_border));
}
#endif
";

struct State {
    geng: Geng,
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
            program: geng.shader_lib().compile(SHADER_SOURCE).unwrap(),
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        let text = "� О, аутлайн!";
        let bb = self.font.measure_bounding_box(text).unwrap();
        let camera = geng::Camera2d {
            center: bb.center(),
            rotation: 0.0,
            fov: 4.0,
        };
        self.font.draw_with(text, |glyphs, atlas| {
            ugli::draw(
                framebuffer,
                &self.program,
                ugli::DrawMode::TriangleFan,
                ugli::instanced(
                    &ugli::VertexBuffer::new_dynamic(
                        self.geng.ugli(),
                        Aabb2::point(Vec2::ZERO)
                            .extend_positive(vec2(1.0, 1.0))
                            .corners()
                            .into_iter()
                            .map(|v| draw_2d::Vertex { a_pos: v })
                            .collect(),
                    ),
                    &ugli::VertexBuffer::new_dynamic(self.geng.ugli(), glyphs.to_vec()),
                ),
                (
                    ugli::uniforms! {
                        u_texture: atlas,
                    },
                    geng::camera2d_uniforms(&camera, framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::default()),
                    ..default()
                },
            );
        });

        let start = bb.min.x;
        let end = bb.max.x;
        let line_width = 0.02;
        self.geng.draw_2d(
            framebuffer,
            &camera,
            &draw_2d::Segment::new(
                Segment(vec2(start, 0.0), vec2(end, 0.0)),
                line_width,
                Rgba::new(1.0, 0.0, 0.0, 0.5),
            ),
        );
        self.geng.draw_2d(
            framebuffer,
            &camera,
            &draw_2d::Segment::new(
                Segment(
                    vec2(start, self.font.ascender()),
                    vec2(end, self.font.ascender()),
                ),
                line_width,
                Rgba::new(0.0, 1.0, 0.0, 0.5),
            ),
        );
        self.geng.draw_2d(
            framebuffer,
            &camera,
            &draw_2d::Segment::new(
                Segment(
                    vec2(start, self.font.descender()),
                    vec2(end, self.font.descender()),
                ),
                line_width,
                Rgba::new(0.0, 0.0, 1.0, 0.5),
            ),
        );
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Font");
    let state = State::new(&geng);
    geng::run(&geng, state);
}
