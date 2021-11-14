use geng::prelude::*;

struct State {
    geng: Geng,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self { geng: geng.clone() }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }
    fn ui(&mut self) -> Box<dyn geng::ui::Widget + '_> {
        use geng::ui;
        use geng::ui::*;
        Box::new(ui::Text::new("Hello, UI!", self.geng.default_font(), 32.0, Color::WHITE).center())
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Geng UI Demo!");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
