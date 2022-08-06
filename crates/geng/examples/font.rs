use geng::prelude::*;

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
                size: 64.0,
                max_distance: 16.0,
            },
        )
        .unwrap();
        Self {
            geng: geng.clone(),
            font,
            program: geng
                .shader_lib()
                .compile(include_str!("font.glsl"))
                .unwrap(),
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None);
        self.font.draw_with("О, аутлайн!", |glyphs, atlas| {
            ugli::draw(
                framebuffer,
                &self.program,
                ugli::DrawMode::TriangleFan,
                ugli::instanced(
                    &ugli::VertexBuffer::new_dynamic(
                        self.geng.ugli(),
                        AABB::point(Vec2::ZERO)
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
                    geng::camera2d_uniforms(
                        &geng::Camera2d {
                            center: vec2(2.0, 0.0),
                            rotation: 0.0,
                            fov: 3.0,
                        },
                        framebuffer.size().map(|x| x as f32),
                    ),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::default()),
                    ..default()
                },
            );
        });
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Font");
    let state = State::new(&geng);
    geng::run(&geng, state);
}
