use super::*;

pub struct Model {
    geng: Rc<Geng>,
    bounds: Vec2<f32>,
    pub ball: Ball,
    pub player_left: Player,
    pub player_right: Player,
}

impl Model {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self {
            geng: geng.clone(),
            bounds: vec2(550.0, 375.0),
            ball: Ball {
                radius: 15.0,
                position: vec2(0.0, 0.0),
                velocity: vec2(150.0, -200.0),
            },
            player_left: Player {
                size: vec2(20.0, 80.0),
                position: vec2(-500.0, 0.0),
                velocity: vec2(0.0, 0.0),
                max_speed: 200.0,
            },
            player_right: Player {
                size: vec2(20.0, 80.0),
                position: vec2(500.0, 0.0),
                velocity: vec2(0.0, 0.0),
                max_speed: 200.0,
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
        // Update velocities
        self.player_left
            .update_velocity(self.geng.window(), geng::Key::W, geng::Key::S);
        self.player_right
            .update_velocity(self.geng.window(), geng::Key::Up, geng::Key::Down);
        if self.ball.position.y.abs() > self.bounds.y - self.ball.radius {
            self.ball.velocity.y *= -1.0;
        }

        // Update positions
        self.ball.position += self.ball.velocity * delta_time;
        self.player_left.position += self.player_left.velocity * delta_time;
        self.player_right.position += self.player_right.velocity * delta_time;

        // Check for collisions
        if self.player_left.position.y.abs() > self.bounds.y - self.player_left.size.y / 2.0 {
            self.player_left.position.y = self.player_left.position.y.signum()
                * (self.bounds.y - self.player_left.size.y / 2.0);
        }
        if self.player_right.position.y.abs() > self.bounds.y - self.player_right.size.y / 2.0 {
            self.player_right.position.y = self.player_right.position.y.signum()
                * (self.bounds.y - self.player_right.size.y / 2.0);
        }
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
    pub velocity: Vec2<f32>,
    pub max_speed: f32,
}

impl Player {
    fn update_velocity(&mut self, window: &geng::Window, key_up: geng::Key, key_down: geng::Key) {
        let mut vel = 0;
        if window.is_key_pressed(key_up) {
            vel += 1;
        }
        if window.is_key_pressed(key_down) {
            vel -= 1;
        }

        self.velocity = vec2(0.0, vel as f32 * self.max_speed);
    }
}
