use geng::prelude::*;

struct State {
    geng: Geng,
    camera: geng::Camera2d,
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw a segment
        // A line segment connecting two points in space
        // let segment = Segment {};
        // segment.draw_2d(&self.geng, framebuffer, &self.camera);

        // Draw a chain
        // A polygonal chain connecting a vector of points in space
        // let chain = Chain {};
        // chain.draw_2d(&self.geng, framebuffer, &self.camera);

        // TODO: Draw a curve
    }
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Lines");
    let state = State {
        geng: geng.clone(),
        camera: geng::Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 50.0,
        },
    };

    geng::run(&geng, state)
}
