// This imports a lot of useful stuff >:)
use geng::{prelude::*, Draw2d};

// Struct representing game state (blank in this example)
struct State {
    geng: Geng,
}

impl geng::State for State {
    // Specify how to draw each game frame
    fn draw(
        &mut self,
        framebuffer: &mut ugli::Framebuffer, // The framebuffer to draw onto
    ) {
        // Clear the whole framebuffer
        ugli::clear(
            framebuffer,
            Some(Color::BLACK), // using black color
            None,               // without clearing depth buffer
        );

        let camera = geng::Camera2d {
            center: vec2(0.0, 0.0),
            rotation: 0.0,
            fov: 30.0,
        };

        let mut vertices = vec![
            Vec2 {
                x: -5.480977,
                y: -4.200317,
            },
            Vec2 {
                x: -1.5747049,
                y: -3.5644336,
            },
            Vec2 {
                x: 1.8909947,
                y: -3.0002687,
            },
            Vec2 {
                x: 5.3566937,
                y: -2.4361043,
            },
            Vec2 {
                x: 9.262966,
                y: -1.8002218,
            },
        ];

        vertices.push(camera.screen_to_world(
            framebuffer.size().map(|x| x as f32),
            self.geng.window().mouse_pos().map(|x| x as f32),
        ));

        for &vertex in &vertices {
            draw_2d::Ellipse::circle(vertex, 0.25, Color::rgba(1.0, 0.0, 0.0, 0.5)).draw_2d(
                &self.geng,
                framebuffer,
                &camera,
            );
        }

        let curve = CardinalSpline::new(vertices, 0.5);
        let chain = curve.chain(1);

        for &vertex in &chain.vertices {
            draw_2d::Ellipse::circle(vertex, 0.2, Color::rgba(0.0, 1.0, 0.0, 0.5)).draw_2d(
                &self.geng,
                framebuffer,
                &camera,
            );
        }

        let chain = draw_2d::Chain::new(chain, 0.5, Color::rgba(0.0, 0.0, 1.0, 0.5), 0);
        chain.draw_2d(&self.geng, framebuffer, &camera);

        let vertices = vec![
            Vec2 {
                x: -1.5747049,
                y: -3.5644336,
            },
            Vec2 {
                x: -0.58769673,
                y: -3.4037633,
            },
            Vec2 {
                x: -2.561713,
                y: -3.7251039,
            },
            Vec2 {
                x: -0.9500098,
                y: -2.7835648,
            },
            Vec2 {
                x: -1.4185312,
                y: -3.3692164,
            },
        ];

        for &vertex in &vertices {
            draw_2d::Ellipse::circle(vertex, 0.2, Color::rgba(0.0, 1.0, 0.0, 0.5)).draw_2d(
                &self.geng,
                framebuffer,
                &camera,
            );
        }

        let vertices = vec![
            Vec2 {
                x: -5.521145,
                y: -3.953565,
            },
            Vec2 {
                x: -5.4408092,
                y: -4.447069,
            },
            Vec2 {
                x: -1.4185312,
                y: -3.3692164,
            },
            Vec2 {
                x: -5.4408092,
                y: -4.447069,
            },
            Vec2 {
                x: -1.4185312,
                y: -3.3692164,
            },
            Vec2 {
                x: -1.338196,
                y: -3.8627205,
            },
            Vec2 {
                x: -1.4185312,
                y: -3.3692164,
            },
            Vec2 {
                x: -1.338196,
                y: -3.8627205,
            },
            Vec2 {
                x: -1.338196,
                y: -3.8627205,
            },
            Vec2 {
                x: -1.4185312,
                y: -3.3692164,
            },
            Vec2 {
                x: -1.338196,
                y: -3.8627205,
            },
            Vec2 {
                x: 1.8106594,
                y: -2.7567647,
            },
            Vec2 {
                x: -1.338196,
                y: -3.8627205,
            },
            Vec2 {
                x: 1.8106594,
                y: -2.7567647,
            },
            Vec2 {
                x: 1.8909947,
                y: -3.2502687,
            },
            Vec2 {
                x: 1.8909947,
                y: -3.2502687,
            },
            Vec2 {
                x: 1.8106595,
                y: -2.7567647,
            },
            Vec2 {
                x: 1.8106594,
                y: -2.7567647,
            },
            Vec2 {
                x: 1.8106595,
                y: -2.7567647,
            },
            Vec2 {
                x: 1.8909947,
                y: -3.2502687,
            },
            Vec2 {
                x: 5.2763586,
                y: -2.1926003,
            },
            Vec2 {
                x: 1.8909947,
                y: -3.2502687,
            },
            Vec2 {
                x: 5.2763586,
                y: -2.1926003,
            },
            Vec2 {
                x: 5.3566937,
                y: -2.6861043,
            },
            Vec2 {
                x: 5.3566937,
                y: -2.6861043,
            },
            Vec2 {
                x: 5.2763586,
                y: -2.1926003,
            },
            Vec2 {
                x: 5.2763586,
                y: -2.1926003,
            },
            Vec2 {
                x: 5.2763586,
                y: -2.1926003,
            },
            Vec2 {
                x: 5.3566937,
                y: -2.6861043,
            },
            Vec2 {
                x: 9.222798,
                y: -1.5534698,
            },
            Vec2 {
                x: 5.3566937,
                y: -2.6861043,
            },
            Vec2 {
                x: 9.222798,
                y: -1.5534698,
            },
            Vec2 {
                x: 9.303134,
                y: -2.046974,
            },
        ];

        for &vertex in &vertices {
            draw_2d::Ellipse::circle(vertex, 0.1, Color::rgba(1.0, 1.0, 0.0, 0.25)).draw_2d(
                &self.geng,
                framebuffer,
                &camera,
            );
        }

        // self.geng.draw_2d_helper().draw(
        //     framebuffer,
        //     &camera,
        //     &vertices,
        //     Color::rgba(0.0, 1.0, 1.0, 0.5),
        //     ugli::DrawMode::Triangles,
        // );
    }
}

fn main() {
    // Initialize logger
    logger::init().unwrap();

    // Initialize the engine using default options
    let geng = Geng::new("Blank");

    // Create the game state
    let state = State { geng: geng.clone() };

    // Run the game
    geng::run(&geng, state)
}
