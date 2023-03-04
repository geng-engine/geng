use geng::prelude::*;

// We can hold a handle to the engine in the game state since we need it to get default font
struct State {
    geng: Geng,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(), // Internally Geng is a just smart pointer, like an Rc
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        // Draw text using default font
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera, // using pixel coordinates
            "Hello, World!",
            framebuffer.size().map(|x| x as f32 / 2.0), // in the middle of the screen
            geng::TextAlign::CENTER,                    // center-aligned
            32.0,                                       // 32 pixels high
            Rgba::WHITE,
        );
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Hello, World!");
    let state = State::new(&geng);
    geng.run(state);
}
