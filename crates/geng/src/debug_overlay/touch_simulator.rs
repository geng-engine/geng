use super::*;

pub struct TouchSimulator {
    geng: Geng,
    touches: Vec<Vec2<f64>>,
    holding: Option<usize>,
}

const RADIUS: f64 = 10.0;

impl TouchSimulator {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            touches: Vec::new(),
            holding: None,
        }
    }
    pub fn handle_event(&mut self, event: &Event) -> Option<Vec<Event>> {
        match event {
            &Event::MouseDown {
                position,
                button: MouseButton::Left,
            } => {
                if let Some(index) = self
                    .touches
                    .iter()
                    .position(|&touch| (touch - position).len() < RADIUS)
                {
                    self.holding = Some(index);
                } else {
                    return Some(self.new_touch(position));
                }
            }
            &Event::MouseMove { position, .. } => {
                if let Some(index) = self.holding {
                    return Some(self.move_touch(index, position));
                } else {
                    return Some(vec![]);
                }
            }
            &Event::MouseDown {
                position,
                button: MouseButton::Right,
            } => {
                if let Some(index) = self
                    .touches
                    .iter()
                    .position(|&touch| (touch - position).len() < RADIUS)
                {
                    self.touches.remove(index);
                    return Some(vec![Event::TouchEnd {
                        touches: self
                            .touches
                            .iter()
                            .map(|&position| TouchPoint { position })
                            .collect(),
                    }]);
                } else {
                    return Some(vec![]);
                }
            }
            Event::MouseDown { .. } => return Some(vec![]),
            Event::MouseUp { .. } => {
                self.holding = None;
                return Some(vec![]);
            }
            _ => {}
        }
        None
    }
    fn new_touch(&mut self, position: Vec2<f64>) -> Vec<Event> {
        self.holding = Some(self.touches.len());
        self.touches.push(position);
        vec![Event::TouchStart {
            touches: self
                .touches
                .iter()
                .map(|&position| TouchPoint { position })
                .collect(),
        }]
    }
    fn move_touch(&mut self, index: usize, position: Vec2<f64>) -> Vec<Event> {
        self.touches[index] = position;
        vec![Event::TouchMove {
            touches: self
                .touches
                .iter()
                .map(|&position| TouchPoint { position })
                .collect(),
        }]
    }
    pub fn draw(&self, framebuffer: &mut ugli::Framebuffer) {
        for &touch in &self.touches {
            self.geng.draw_2d(
                framebuffer,
                &PixelPerfectCamera,
                &draw_2d::Ellipse::circle_with_cut(
                    touch.map(|x| x as f32),
                    RADIUS as f32 - 2.0,
                    RADIUS as f32 + 2.0,
                    Color::rgba(0.5, 0.5, 0.5, 0.5),
                ),
            );
        }
    }
}
