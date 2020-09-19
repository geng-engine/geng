use geng::prelude::*;

struct State;

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }
}

fn main() {
    geng::run(Rc::new(Geng::new(default())), State)
}
