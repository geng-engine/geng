use geng::prelude::*;

struct State {
    geng: Rc<Geng>,
    ball: Ball,
}

impl State {
    fn new(geng: &Rc<Geng>) -> Self {
        Self {
            geng: geng.clone(),
            ball: Ball {
                radius: 10.0,
                position: vec2(0.0, 0.0),
                velocity: vec2(10.0, -5.0),
            },
        }
    }

    fn reset(&mut self) {
        self.ball = Ball {
            radius: 10.0,
            position: vec2(0.0, 0.0),
            velocity: vec2(10.0, -5.0),
        };
    }
}

impl geng::State for State {
    fn update(&mut self, delta_time: f64) {
        self.ball.position += self.ball.velocity * delta_time as f32;
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        self.geng.draw_2d().circle(
            framebuffer,
            self.ball.position + framebuffer.size().map(|x| (x as f32) / 2.0),
            self.ball.radius,
            Color::RED,
        );
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key: geng::Key::R } => self.reset(),
            _ => (),
        }
    }
}

fn main() {
    let geng = Rc::new(Geng::new(default()));
    let state = State::new(&geng);
    geng::run(geng, state);
}

struct Ball {
    radius: f32,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}
