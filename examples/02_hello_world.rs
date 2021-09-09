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
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw text using default font
        self.geng.default_font().draw_aligned(
            framebuffer,
            &geng::PixelPerfectCamera, // using pixel coordinates
            "Hello, World!",
            framebuffer.size().map(|x| x as f32 / 2.0), // in the middle of the screen
            0.5,  // center-aligned (0.0 is left, 1.0 is right)
            32.0, // 32 pixels high
            Color::WHITE,
        );
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Hello, World!");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
