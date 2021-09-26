use geng::prelude::*;

struct State {
    geng: Geng,
    position: Vec2<f32>, // Current character's position
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            position: vec2(0.0, 0.0),
        }
    }
}

impl geng::State for State {
    // Specify how to update game state over time
    fn update(
        &mut self,
        delta_time: f64, // Time in seconds since last update
    ) {
        let delta_time = delta_time as f32;

        if self.geng.window().is_key_pressed(geng::Key::Left) {
            self.position.x -= delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::Right) {
            self.position.x += delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::Up) {
            self.position.y += delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::Down) {
            self.position.y -= delta_time;
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.geng.draw_2d().circle(
            framebuffer,
            &geng::Camera2d {
                center: vec2(0.0, 0.0),
                fov: 10.0,
            },
            self.position,
            1.0,
            Color::WHITE,
        );
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Moving");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
