use geng::prelude::*;

struct State {
    geng: Geng,
    texture: ugli::Texture,
    rendered_texture: ugli::Texture,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            texture: ugli::Texture::new_with(geng.ugli(), vec2(2, 2), |pos| match (pos.x, pos.y) {
                (0, 0) => Color::BLACK,
                (1, 0) => Color::RED,
                (1, 1) => Color::GREEN,
                (0, 1) => Color::BLUE,
                _ => unreachable!(),
            }),
            rendered_texture: {
                const SIZE: usize = 128;
                let mut texture = ugli::Texture::new_uninitialized(geng.ugli(), vec2(SIZE, SIZE));
                let mut framebuffer = ugli::Framebuffer::new_color(
                    geng.ugli(),
                    ugli::ColorAttachment::Texture(&mut texture),
                );
                geng.draw_2d(
                    &mut framebuffer,
                    &geng::PixelPerfectCamera,
                    &draw_2d::Polygon::new_gradient(vec![
                        draw_2d::ColoredVertex {
                            a_pos: vec2(0.0, 0.0),
                            a_color: Color::BLACK,
                        },
                        draw_2d::ColoredVertex {
                            a_pos: vec2(SIZE as f32, 0.0),
                            a_color: Color::RED,
                        },
                        draw_2d::ColoredVertex {
                            a_pos: vec2(SIZE as f32, SIZE as f32),
                            a_color: Color::GREEN,
                        },
                        draw_2d::ColoredVertex {
                            a_pos: vec2(0.0, SIZE as f32),
                            a_color: Color::BLUE,
                        },
                    ]),
                );
                texture
            },
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        let camera = geng::Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 10.0,
        };
        let mut objects = Vec::<Box<dyn geng::draw_2d::Draw2d>>::new();
        objects.push(Box::new(
            draw_2d::Quad::unit(Color::WHITE)
                .transform(Mat3::rotate(0.5) * Mat3::scale_uniform(0.5)),
        ));
        objects.push(Box::new(draw_2d::TexturedQuad::unit(&self.texture)));
        objects.push(Box::new(draw_2d::TexturedQuad::unit(
            &self.rendered_texture,
        )));
        objects.push(Box::new(draw_2d::Ellipse::unit(Color::RED)));
        objects.push(Box::new(
            draw_2d::Ellipse::unit_with_cut(0.5, Color::RED)
                .transform(Mat3::rotate(f32::PI / 4.0) * Mat3::scale(vec2(1.0, 0.5))),
        ));
        objects.push(Box::new(draw_2d::Polygon::new_gradient(vec![
            draw_2d::ColoredVertex {
                a_pos: vec2(-1.0, -1.0),
                a_color: Color::RED,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(1.0, -1.0),
                a_color: Color::GREEN,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(0.0, 1.0),
                a_color: Color::BLUE,
            },
        ])));
        objects.push(Box::new(draw_2d::Polygon::strip(
            vec![
                vec2(-1.0, -1.0),
                vec2(0.0, -1.0),
                vec2(-0.5, 0.0),
                vec2(0.0, 0.0),
                vec2(0.5, 1.0),
                vec2(1.0, 0.5),
            ],
            Color::GRAY,
        )));

        let mut x = -5.0;
        for drawable in objects {
            self.geng.draw_2d(
                framebuffer,
                &camera,
                &drawable.transform(Mat3::translate(vec2(x, 0.0))),
            );
            x += 2.0;
        }
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Let's draw!");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
