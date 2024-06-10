use geng::prelude::*;

struct State {
    geng: Geng,
    camera: geng::Camera2d,        // Store camera in the game state
    framebuffer_size: vec2<f32>,   // Save framebuffer size to access it outside of draw call
    drag_start: Option<vec2<f32>>, // Store location that needs to stay under cursor
    prev_touch_distance: f32,
    prev_touch_angle: Angle,
    cursor_pos: Option<vec2<f64>>,
    touches: Vec<geng::Touch>,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            camera: geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: Angle::ZERO,
                fov: 15.0,
            },
            framebuffer_size: vec2(1.0, 1.0),
            prev_touch_distance: 0.0,
            prev_touch_angle: Angle::ZERO,
            drag_start: None,
            cursor_pos: None,
            touches: vec![],
        }
    }
}

impl geng::State for State {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        // Rotating camera
        if self.geng.window().is_key_pressed(geng::Key::Q) {
            self.camera.rotation -= Angle::from_radians(delta_time);
        }
        if self.geng.window().is_key_pressed(geng::Key::E) {
            self.camera.rotation += Angle::from_radians(delta_time);
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
            geng::Event::KeyPress {
                key: geng::Key::Space,
            } => {
                *self = Self::new(&self.geng);
            }
            // Scrolling to zoom
            geng::Event::Wheel { delta } => {
                self.camera.fov = (self.camera.fov * 1.01f32.powf(-delta as f32)).clamp(1.0, 30.0);
            }
            // Drag start
            geng::Event::MousePress {
                button: geng::MouseButton::Left,
            } => {
                if let Some(position) = self.cursor_pos {
                    self.drag_start = Some(
                        self.camera
                            .screen_to_world(self.framebuffer_size, position.map(|x| x as f32)),
                    );
                }
            }
            // Drag move
            geng::Event::CursorMove { position } => {
                self.cursor_pos = Some(position);
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
            geng::Event::MouseRelease {
                button: geng::MouseButton::Left,
            } => self.drag_start = None,
            geng::Event::TouchStart(touch) => {
                self.touches.push(touch);
                if self.touches.len() == 1 {
                    self.drag_start = Some(self.camera.screen_to_world(
                        self.framebuffer_size,
                        self.touches[0].position.map(|x| x as f32),
                    ));
                }
                if self.touches.len() == 2 {
                    let diff = self.touches[0].position - self.touches[1].position;
                    self.prev_touch_distance = diff.len() as f32;
                    self.prev_touch_angle = diff.map(|x| x as f32).arg();
                    self.drag_start = Some(self.camera.screen_to_world(
                        self.framebuffer_size,
                        (self.touches[0].position + self.touches[1].position).map(|x| x as f32)
                            / 2.0,
                    ));
                }
            }
            geng::Event::TouchMove(touch) => {
                if let Some(t) = self.touches.iter_mut().find(|t| t.id == touch.id) {
                    *t = touch;
                }
                if self.touches.len() == 1 {
                    if let Some(start) = self.drag_start {
                        let current_pos = self.camera.screen_to_world(
                            self.framebuffer_size,
                            self.touches[0].position.map(|x| x as f32),
                        );
                        self.camera.center += start - current_pos;
                    }
                } else if self.touches.len() == 2 {
                    let diff = self.touches[0].position - self.touches[1].position;
                    let now_dist = diff.len() as f32;
                    self.camera.fov /= now_dist / self.prev_touch_distance;
                    self.prev_touch_distance = now_dist;
                    let now_angle = diff.map(|x| x as f32).arg();
                    let angle_diff = (now_angle - self.prev_touch_angle).normalized_pi();
                    self.camera.rotation += angle_diff;
                    self.prev_touch_angle = now_angle;
                    if let Some(start) = self.drag_start {
                        let current_pos = self.camera.screen_to_world(
                            self.framebuffer_size,
                            (self.touches[0].position + self.touches[1].position).map(|x| x as f32)
                                / 2.0,
                        );
                        self.camera.center += start - current_pos;
                    }
                }
            }
            geng::Event::TouchEnd(touch) => {
                self.touches.retain(|t| t.id != touch.id);
                self.drag_start = None;
            }
            _ => {}
        }
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run("Moving", |geng| async move {
        geng.run_state(State::new(&geng)).await;
    });
}
