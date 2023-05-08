use super::*;

pub struct TouchSimulator {
    next_id: u64,
    draw2d: Rc<draw2d::Helper>,
    touches: Vec<Touch>,
    holding: Option<usize>,
}

const RADIUS: f64 = 10.0;

impl TouchSimulator {
    pub fn new(draw2d: &Rc<draw2d::Helper>) -> Self {
        Self {
            next_id: 0,
            draw2d: draw2d.clone(),
            touches: Vec::new(),
            holding: None,
        }
    }
    pub fn update(&mut self, _delta_time: f64) {}
    pub fn handle_event(&mut self, event: &Event) -> Option<Vec<Event>> {
        match event {
            &Event::MouseDown {
                position,
                button: MouseButton::Left,
            } => {
                if let Some(index) = self
                    .touches
                    .iter()
                    .position(|&touch| (touch.position - position).len() < RADIUS)
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
                    .position(|&touch| (touch.position - position).len() < RADIUS)
                {
                    return Some(vec![Event::TouchEnd(self.touches.remove(index))]);
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
    fn new_touch(&mut self, position: vec2<f64>) -> Vec<Event> {
        self.holding = Some(self.touches.len());
        let touch = Touch {
            id: self.next_id,
            position,
        };
        self.next_id += 1;
        self.touches.push(touch);
        vec![Event::TouchStart(touch)]
    }
    fn move_touch(&mut self, index: usize, position: vec2<f64>) -> Vec<Event> {
        self.touches[index].position = position;
        vec![Event::TouchMove(self.touches[index])]
    }
    pub fn draw(&self, framebuffer: &mut ugli::Framebuffer) {
        for &touch in &self.touches {
            self.draw2d.draw2d(
                framebuffer,
                &geng_camera::PixelPerfectCamera,
                &draw2d::Ellipse::circle_with_cut(
                    touch.position.map(|x| x as f32),
                    RADIUS as f32 - 2.0,
                    RADIUS as f32 + 2.0,
                    Rgba::new(0.5, 0.5, 0.5, 0.5),
                ),
            );
        }
    }
}
