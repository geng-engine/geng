use geng::prelude::*;

struct State {
    geng: Geng,
    texture: ugli::Texture,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            texture: ugli::Texture::new_with(geng.ugli(), vec2(2, 2), |pos| {
                Color::rgb(pos.x as f32, pos.y as f32, 0.0)
            }),
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
        self.geng.draw_with(
            framebuffer,
            &camera as &dyn geng::AbstractCamera2d,
            AABB::point(Vec2::ZERO).extend_uniform(1.0),
            Color::WHITE,
        );
        self.geng.draw_with(
            framebuffer,
            &camera as &dyn geng::AbstractCamera2d,
            AABB::point(Vec2::ZERO)
                .extend_uniform(1.0)
                .translate(vec2(2.0, 0.0)),
            &self.texture,
        );
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Let's draw!");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
