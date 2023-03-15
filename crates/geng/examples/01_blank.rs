// This imports a lot of useful stuff >:)
use geng::prelude::*;

// Struct representing game state (blank in this example)
struct State;

impl geng::State for State {
    // Specify how to draw each game frame
    fn draw(
        &mut self,
        framebuffer: &mut ugli::Framebuffer, // The framebuffer to draw onto
    ) {
        // Clear the whole framebuffer
        ugli::clear(
            framebuffer,
            Some(Rgba::BLACK), // using black color
            None,              // without clearing depth buffer
            None,              // or stencil
        );
    }
}

fn main() {
    // Initialize logger
    logger::init();
    geng::setup_panic_handler();

    // Initialize the engine using default options
    let geng = Geng::new("Blank");

    // Create the game state
    let state = State;

    // Run the game
    geng.run(state);
}
