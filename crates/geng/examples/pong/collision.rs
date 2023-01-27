use geng::prelude::*;

use crate::{ball::Ball, player::Player};

pub struct Collision {
    pub normal: vec2<f32>,
    pub penetration: f32,
}

pub fn collide(ball: &Ball, player: &Player) -> Option<Collision> {
    let dx = ball.position.x - player.position.x;
    let dy = ball.position.y - player.position.y;

    if dx >= 0.0 && dx <= player.size.x {
        if dy <= 0.0 && dy >= -ball.radius {
            // Bottom
            Some(Collision {
                normal: vec2(0.0, -1.0),
                penetration: dy + ball.radius,
            })
        } else if dy >= player.size.y && dy <= player.size.y + ball.radius {
            // Top
            Some(Collision {
                normal: vec2(0.0, 1.0),
                penetration: player.size.y + ball.radius - dy,
            })
        } else {
            None
        }
    } else if dy >= 0.0 && dy <= player.size.y {
        if dx <= 0.0 && dx >= -ball.radius {
            // Left
            Some(Collision {
                normal: vec2(-1.0, 0.0),
                penetration: dx + ball.radius,
            })
        } else if dx >= player.size.x && dx <= player.size.x + ball.radius {
            // Right
            Some(Collision {
                normal: vec2(1.0, 0.0),
                penetration: player.size.x + ball.radius - dx,
            })
        } else {
            None
        }
    } else {
        let corner = if dx <= 0.0 {
            if dy <= 0.0 {
                // Bottom left
                player.position
            } else {
                // Top left
                player.position + vec2(0.0, player.size.y)
            }
        } else if dy <= 0.0 {
            // Bottom right
            player.position + vec2(player.size.x, 0.0)
        } else {
            // Top right
            player.position + player.size
        };
        let normal = ball.position - corner;
        let penetration = ball.radius - normal.len();
        let normal = normal.normalize();
        if penetration >= 0.0 {
            Some(Collision {
                normal,
                penetration,
            })
        } else {
            None
        }
    }
}
