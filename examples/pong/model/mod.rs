use super::*;

pub struct Model {
    pub ball: Ball,
}

impl Model {
    pub fn new() -> Self {
        Self {
            ball: Ball {
                radius: 10.0,
                position: vec2(0.0, 0.0),
                velocity: vec2(10.0, -5.0),
            },
        }
    }
    pub fn reset(&mut self) {
        self.ball = Ball {
            radius: 10.0,
            position: vec2(0.0, 0.0),
            velocity: vec2(10.0, -5.0),
        };
    }
    pub fn update(&mut self, delta_time: f32) {
        self.ball.position += self.ball.velocity * delta_time;
    }
    pub fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key: geng::Key::R } => self.reset(),
            _ => (),
        }
    }
}

pub struct Ball {
    pub radius: f32,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
}
