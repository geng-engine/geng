use geng::prelude::*;

struct State {
    geng: Geng,
    position: vec2<f32>, // Current position
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            position: vec2::ZERO,
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

        // Move depending on the keys currently being pressed
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
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.geng.default_font().draw(
            framebuffer,
            &geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov: 15.0,
            },
            "Use arrow keys to move around\nPress Space to reset",
            self.position,
            geng::TextAlign::CENTER,
            1.0,
            Rgba::WHITE,
        );
    }
    // We can handle events like KeyDown by implementing this method
    fn handle_event(&mut self, event: geng::Event) {
        if matches!(
            event,
            geng::Event::KeyDown {
                key: geng::Key::Space
            }
        ) {
            self.position = vec2::ZERO;
        }
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Moving");
    let state = State::new(&geng);
    geng.run(state);
}
