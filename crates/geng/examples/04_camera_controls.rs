use geng::prelude::*;

struct State {
    geng: Geng,
    camera: geng::Camera2d,        // Store camera in the game state
    framebuffer_size: vec2<f32>,   // Save framebuffer size to access it outside of draw call
    drag_start: Option<vec2<f32>>, // Store location that needs to stay under cursor
    prev_touch_distance: f32,
    prev_touch_angle: f32,
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
            prev_touch_distance: 0.0,
            prev_touch_angle: 0.0,
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
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            "Scroll to zoom\nDrag LMB to move\nQ/E to rotate",
            vec2::splat(geng::TextAlign::CENTER),
            mat3::identity(),
            Rgba::WHITE,
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
                self.camera.fov = (self.camera.fov * 1.01f32.powf(-delta as f32)).clamp(1.0, 30.0);
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
            // TODO OMEGALUL
            // geng::Event::TouchStart { touches } => {
            //     if touches.len() == 1 {
            //         self.drag_start = Some(self.camera.screen_to_world(
            //             self.framebuffer_size,
            //             touches[0].position.map(|x| x as f32),
            //         ));
            //     }
            //     if touches.len() == 2 {
            //         let diff = touches[0].position - touches[1].position;
            //         self.prev_touch_distance = diff.len() as f32;
            //         self.prev_touch_angle = f64::atan2(diff.x, diff.y) as f32;
            //         self.drag_start = Some(self.camera.screen_to_world(
            //             self.framebuffer_size,
            //             (touches[0].position + touches[1].position).map(|x| x as f32) / 2.0,
            //         ));
            //     }
            // }
            // geng::Event::TouchMove { touches } => {
            //     if touches.len() == 1 {
            //         if let Some(start) = self.drag_start {
            //             let current_pos = self.camera.screen_to_world(
            //                 self.framebuffer_size,
            //                 touches[0].position.map(|x| x as f32),
            //             );
            //             self.camera.center += start - current_pos;
            //         }
            //     } else if touches.len() == 2 {
            //         let diff = touches[0].position - touches[1].position;
            //         let now_dist = diff.len() as f32;
            //         self.camera.fov /= now_dist / self.prev_touch_distance;
            //         self.prev_touch_distance = now_dist;
            //         let now_angle = f64::atan2(diff.x, diff.y) as f32;
            //         let mut angle_diff = now_angle - self.prev_touch_angle;
            //         while angle_diff > std::f32::consts::PI {
            //             angle_diff -= 2.0 * std::f32::consts::PI;
            //         }
            //         while angle_diff < -std::f32::consts::PI {
            //             angle_diff += 2.0 * std::f32::consts::PI;
            //         }
            //         self.camera.rotation -= angle_diff;
            //         self.prev_touch_angle = now_angle;
            //         if let Some(start) = self.drag_start {
            //             let current_pos = self.camera.screen_to_world(
            //                 self.framebuffer_size,
            //                 (touches[0].position + touches[1].position).map(|x| x as f32) / 2.0,
            //             );
            //             self.camera.center += start - current_pos;
            //         }
            //     }
            // }
            // geng::Event::TouchEnd { .. } => {
            //     self.drag_start = None;
            // }
            _ => {}
        }
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new("Moving");
    let state = State::new(&geng);
    geng.run(state);
}
