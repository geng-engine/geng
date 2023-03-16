use geng::prelude::*;

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
uniform vec4 u_color;
uniform vec4 u_border_color;
uniform float u_outline_distance;

float aa(float x) {
    float w = length(vec2(dFdx(x), dFdy(x)));
    return 1.0 - smoothstep(-w, w, x);
}

float read_sdf(sampler2D texture, vec2 uv) {
    return 1.0 - 2.0 * texture2D(texture, uv).x;
}

void main() {
    float dist = read_sdf(u_texture, v_uv);
    float inside = aa(dist);
    float inside_border = aa(dist - u_outline_distance);
    vec4 outside_color = vec4(u_border_color.xyz, 0.0);
    gl_FragColor = u_color * inside +
        (1.0 - inside) * (
            u_border_color * inside_border +
            outside_color * (1.0 - inside_border)
        );
}
#endif
";

struct State {
    geng: Geng,
    program: ugli::Program,
    texture: ugli::Texture,
    font: geng::Font,
}

impl State {
    fn new(geng: &Geng) -> Self {
        let font = geng::Font::new(
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
            program: geng.shader_lib().compile(SHADER_SOURCE).unwrap(),
            texture: font
                .create_text_sdf("Hello, Crabs", geng::TextAlign::CENTER, 64.0)
                .unwrap(),
            font,
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        let size = vec2(self.texture.size().map(|x| x as f32).aspect(), 1.0);
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
                    u_model_matrix: mat3::scale(size),
                    u_texture: &self.texture,
                    u_color: Rgba::WHITE,
                    u_border_color: Rgba::GRAY,
                    u_outline_distance: self.font.max_distance() / 2.0,
                },
                geng::camera2d_uniforms(
                    &geng::Camera2d {
                        center: size / 2.0,
                        rotation: 0.0,
                        fov: 3.0,
                    },
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

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new("Font");
    let state = State::new(&geng);
    geng.run(state);
}
