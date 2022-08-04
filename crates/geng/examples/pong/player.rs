use geng::prelude::*;

pub struct Player {
    /// Key to move the player up
    key_up: geng::Key,
    /// Key to move the player down
    key_down: geng::Key,
    /// Position of the bottom-left corner of the player
    pub position: Vec2<f32>,
    pub size: Vec2<f32>,
    velocity: Vec2<f32>,
    speed: f32,
    pub color: Rgba<f32>,
}

impl Player {
    /// Creates a new player with given center position
    pub fn new(
        position: Vec2<f32>,
        size: Vec2<f32>,
        speed: f32,
        color: Rgba<f32>,
        key_up: geng::Key,
        key_down: geng::Key,
    ) -> Self {
        Self {
            key_up,
            key_down,
            position: position - size / 2.0,
            size,
            speed,
            color,
            velocity: Vec2::ZERO,
        }
    }

    /// Handles input to control the player's velocity
    pub fn control(&mut self, window: &geng::Window) {
        // Check input
        let mut direction_y = 0.0;
        if window.is_key_pressed(self.key_up) {
            direction_y += 1.0;
        }
        if window.is_key_pressed(self.key_down) {
            direction_y -= 1.0;
        }

        // Update velocity
        self.velocity = vec2(0.0, self.speed * direction_y);
    }

    /// Moves the player according to its velocity
    pub fn movement(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }

    /// Returns an AABB representing the player's shape
    pub fn aabb(&self) -> AABB<f32> {
        AABB::point(self.position).extend_positive(self.size)
    }
}
