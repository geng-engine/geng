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
        ugli::clear(framebuffer, Some(Rgba::TRANSPARENT_BLACK), None, None);
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            "Hello, World!",
            framebuffer.size().map(|x| x as f32 / 2.0),
            geng::TextAlign::CENTER,
            32.0,
            Rgba::WHITE,
        );
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new_with(geng::ContextOptions {
        title: "Transparent".to_owned(),
        transparency: true,
        ..default()
    });
    let state = State::new(&geng);
    geng.run(state);
}
