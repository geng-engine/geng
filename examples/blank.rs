use geng::prelude::*;

struct State;

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }
}

fn main() {
    let geng = Geng::new(geng::ContextOptions {
        title: "Blank".to_owned(),
        ..default()
    });
    geng::run(&geng, State)
}
