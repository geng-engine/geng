use geng::prelude::*;

struct State {
    geng: Geng,
    camera: geng::Camera2d,        // Store camera in the game state
    framebuffer_size: Vec2<f32>,   // Save framebuffer size to access it outside of draw call
    drag_start: Option<Vec2<f32>>, // Store location that needs to stay under cursor
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            camera: geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov: 15.0,
            },
            framebuffer_size: vec2(1.0, 1.0),
            drag_start: None,
        }
    }
}

impl geng::State for State {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        // Rotating camera
        if self.geng.window().is_key_pressed(geng::Key::Q) {
            self.camera.rotation += delta_time;
        }
        if self.geng.window().is_key_pressed(geng::Key::E) {
            self.camera.rotation -= delta_time;
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32); // Save framebuffer size
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            "Scroll to zoom\nDrag LMB to move\nQ/E to rotate",
            Vec2::ZERO,
            geng::TextAlign::CENTER,
            1.0,
            Color::WHITE,
        );
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown {
                key: geng::Key::Space,
            } => {
                *self = Self::new(&self.geng);
            }
            // Scrolling to zoom
            geng::Event::Wheel { delta } => {
                self.camera.fov = clamp(self.camera.fov * 1.01f32.powf(-delta as f32), 1.0..=30.0);
            }
            // Drag start
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Left,
            } => {
                self.drag_start = Some(
                    self.camera
                        .screen_to_world(self.framebuffer_size, position.map(|x| x as f32)),
                );
            }
            // Drag move
            geng::Event::MouseMove { position, .. } => {
                if let Some(start) = self.drag_start {
                    // Find current world position under cursor
                    let current_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, position.map(|x| x as f32));
                    // Move camera so that start position is now under cursor
                    self.camera.center += start - current_pos;
                }
            }
            // Drag end
            geng::Event::MouseUp {
                button: geng::MouseButton::Left,
                ..
            } => self.drag_start = None,
            _ => {}
        }
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Moving");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
