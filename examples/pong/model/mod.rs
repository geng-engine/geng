use super::*;

pub struct Model {
    pub ball: Ball,
    pub player_left: Player,
    pub player_right: Player,
}

impl Model {
    pub fn new() -> Self {
        Self {
            ball: Ball {
                radius: 15.0,
                position: vec2(0.0, 0.0),
                velocity: vec2(150.0, -30.0),
            },
            player_left: Player {
                size: vec2(20.0, 80.0),
                position: vec2(-500.0, 0.0),
            },
            player_right:  Player {
                size: vec2(20.0, 80.0),
                position: vec2(500.0, 0.0),
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

pub struct Player {
    pub size: Vec2<f32>,
    pub position: Vec2<f32>,
}
