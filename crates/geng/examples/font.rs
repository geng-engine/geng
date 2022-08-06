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
                pixel_size: 64.0,
                max_distance: 0.25,
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
        let text = "� О, аутлайн!";
        let bb = self.font.measure(text).unwrap();
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
                    geng::camera2d_uniforms(&camera, framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::default()),
                    ..default()
                },
            );
        });

        let start = bb.x_min;
        let end = bb.x_max;
        let line_width = 0.02;
        self.geng.draw_2d(
            framebuffer,
            &camera,
            &draw_2d::Segment::new(
                Segment::new(vec2(start, 0.0), vec2(end, 0.0)),
                line_width,
                Rgba::new(1.0, 0.0, 0.0, 0.5),
            ),
        );
        self.geng.draw_2d(
            framebuffer,
            &camera,
            &draw_2d::Segment::new(
                Segment::new(
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
                Segment::new(
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
