use geng::prelude::*;

pub struct Ball {
    pub position: Vec2<f32>,
    pub radius: f32,
    pub velocity: Vec2<f32>,
    pub color: Color<f32>,
}

impl Ball {
    pub fn new(position: Vec2<f32>, radius: f32, color: Color<f32>) -> Self {
        Self {
            position,
            radius,
            color,
            velocity: Vec2::ZERO,
        }
    }

    pub fn movement(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }
}
