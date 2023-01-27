use geng::prelude::*;

pub struct Ball {
    pub position: vec2<f32>,
    pub radius: f32,
    pub velocity: vec2<f32>,
    pub color: Rgba<f32>,
}

impl Ball {
    pub fn new(position: vec2<f32>, radius: f32, color: Rgba<f32>) -> Self {
        Self {
            position,
            radius,
            color,
            velocity: vec2::ZERO,
        }
    }

    pub fn movement(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }
}
