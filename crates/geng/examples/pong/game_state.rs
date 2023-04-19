use geng::prelude::*;

use crate::{ball::Ball, collision::collide, player::Player};

// Render constants
const BOUNDARY_COLOR: Rgba<f32> = Rgba::GRAY;

const PLAYER_LEFT_COLOR: Rgba<f32> = Rgba::GREEN;
const PLAYER_RIGHT_COLOR: Rgba<f32> = Rgba::BLUE;

const BALL_COLOR: Rgba<f32> = Rgba::RED;

// Game constants
const ARENA_SIZE_X: f32 = 450.0;
const ARENA_SIZE_Y: f32 = 300.0;

const BOUNDARY_WIDTH: f32 = 5.0;

const PLAYER_WIDTH: f32 = 10.0;
const PLAYER_HEIGHT: f32 = 50.0;
const PLAYER_SIZE: vec2<f32> = vec2(PLAYER_WIDTH, PLAYER_HEIGHT);
const PLAYER_SPEED: f32 = 100.0;

const BALL_RADIUS: f32 = 5.0;
const BALL_SPEED: f32 = 100.0;
const BALL_START_ANGLE_MIN: f32 = 0.5;
const BALL_START_ANGLE_MAX: f32 = 0.7;

// Our game state that will hold players, ball and boundaries
pub struct GameState {
    geng: Geng,
    camera: geng::Camera2d,
    boundary: Aabb2<f32>,
    ball: Ball,
    players: [Player; 2],
    scores: [u32; 2],
}

impl GameState {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            camera: geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov: 400.0,
            },
            boundary: Aabb2::ZERO.extend_symmetric(vec2(ARENA_SIZE_X, ARENA_SIZE_Y) / 2.0),
            ball: Self::new_ball(),
            players: {
                // Distance from the arena edge to the player
                let player_offset = PLAYER_WIDTH / 2.0 + 5.0;

                // Spawn left player
                let player_left = Player::new(
                    vec2(-ARENA_SIZE_X / 2.0 + player_offset, 0.0),
                    PLAYER_SIZE,
                    PLAYER_SPEED,
                    PLAYER_LEFT_COLOR,
                    geng::Key::W,
                    geng::Key::S,
                );

                // Spawn right player
                let player_right = Player::new(
                    vec2(ARENA_SIZE_X / 2.0 - player_offset, 0.0),
                    PLAYER_SIZE,
                    PLAYER_SPEED,
                    PLAYER_RIGHT_COLOR,
                    geng::Key::Up,
                    geng::Key::Down,
                );

                [player_left, player_right]
            },
            scores: [0, 0],
        }
    }

    /// Creates a new ball at the center of the world and assigns a random velocity to it
    fn new_ball() -> Ball {
        // Create new ball
        let mut ball = Ball::new(vec2(0.0, 0.0), BALL_RADIUS, BALL_COLOR);

        // Generate a random velocity
        let angle_range = BALL_START_ANGLE_MAX - BALL_START_ANGLE_MIN;
        let random_angle = rand::thread_rng().gen_range(0.0..=angle_range * 4.0);

        // Pick side to shoot: 1.0 - right, -1.0 - left
        let horizontal_mult = (random_angle / angle_range / 2.0).floor() * 2.0 - 1.0;
        // Pick vertical direction to shoot: 1.0 - up, -1.0 - down
        let quarter = (random_angle / angle_range).floor() as i32;
        let vertical_mult = (quarter % 2 * 2 - 1) as f32;

        // Check that values are correct
        debug_assert!(
            horizontal_mult.abs() == 1.0,
            "generated wrong (not +-1) horizontal direction: {horizontal_mult}"
        );
        debug_assert!(
            vertical_mult.abs() == 1.0,
            "generated wrong (not +-1) vertical direction: {vertical_mult}"
        );

        // Generate random direction
        let angle = BALL_START_ANGLE_MIN + random_angle - (quarter as f32) * angle_range;
        let (sin, cos) = angle.sin_cos();
        let direction = vec2(cos * horizontal_mult, sin * vertical_mult);

        ball.velocity = direction * BALL_SPEED;
        ball
    }

    fn control_players(&mut self) {
        let window = self.geng.window();
        for player in &mut self.players {
            player.control(window);
        }
    }

    fn movement(&mut self, delta_time: f32) {
        // Move and clamp players
        for player in &mut self.players {
            player.movement(delta_time);
            player.position.y = player
                .position
                .y
                .clamp(self.boundary.min.y, self.boundary.max.y - player.size.y);
        }

        // Move, bounce, and bounce ball
        let ball = &mut self.ball;
        ball.movement(delta_time);
        if ball.position.y - ball.radius <= self.boundary.min.y
            || ball.position.y + ball.radius >= self.boundary.max.y
        {
            ball.velocity.y *= -1.0;
        }
        ball.position.y = ball.position.y.clamp(
            self.boundary.min.y + ball.radius,
            self.boundary.max.y - ball.radius,
        );
    }

    fn collision(&mut self) {
        // Check for collisions with every player
        let ball = &mut self.ball;
        for player in &self.players {
            // Check if collision occurred
            if let Some(collision) = collide(ball, player) {
                // Move and change velocity of the ball
                ball.position += collision.normal * collision.penetration;
                ball.velocity +=
                    vec2::dot(ball.velocity, -collision.normal) * collision.normal * 2.0;
            }
        }
    }

    /// Checks whether someone scored, adds the score, and resets the round
    fn check_round_end(&mut self) {
        let ball = &self.ball;
        let score = if ball.position.x - ball.radius > self.boundary.max.x {
            // Left player scored
            self.scores[0] += 1;
            true
        } else if ball.position.x + ball.radius < self.boundary.min.x {
            // Right player scored
            self.scores[1] += 1;
            true
        } else {
            // Noone scored
            false
        };

        if score {
            // Reset the ball
            self.ball = Self::new_ball();
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        // Convert delta_time to f32 because that is
        // the type we are going to use in our game data
        let delta_time = delta_time as f32;

        self.control_players();
        self.movement(delta_time);
        self.collision();

        self.check_round_end();
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        // Clear background
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        // Draw boundaries
        let boundary = Aabb2::point(self.boundary.center())
            .extend_symmetric(vec2(self.boundary.width(), BOUNDARY_WIDTH) / 2.0);
        let boundary_translate = self.boundary.height() / 2.0 + BOUNDARY_WIDTH / 2.0;
        self.geng.draw2d().quad(
            framebuffer,
            &self.camera,
            boundary.translate(vec2(0.0, boundary_translate)),
            BOUNDARY_COLOR,
        );
        self.geng.draw2d().quad(
            framebuffer,
            &self.camera,
            boundary.translate(vec2(0.0, -boundary_translate)),
            BOUNDARY_COLOR,
        );

        // Draw players
        for player in &self.players {
            self.geng
                .draw2d()
                .quad(framebuffer, &self.camera, player.aabb(), player.color);
        }

        // Draw ball
        let ball = &self.ball;
        self.geng.draw2d().circle(
            framebuffer,
            &self.camera,
            ball.position,
            ball.radius,
            ball.color,
        );

        // Display scores in format: "00 - 00"
        let scores = format!("{:02} - {:02}", self.scores[0], self.scores[1]);
        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            &scores,
            vec2::splat(geng::TextAlign::CENTER),
            // Just above the top boundary
            mat3::translate(vec2(0.0, self.boundary.max.y + 10.0)) * mat3::scale_uniform(32.0),
            Rgba::WHITE,
        );
    }
}
