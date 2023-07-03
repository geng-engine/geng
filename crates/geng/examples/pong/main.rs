use geng::prelude::*;

mod ball;
mod collision;
mod game_state;
mod player;

use game_state::GameState;

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run_with(
        &geng::ContextOptions {
            window: {
                let mut options = geng::window::Options::new("Pong");
                options.antialias = true;
                options
            },
            ..default()
        },
        |geng| async move {
            geng.run_state(GameState::new(&geng)).await;
        },
    );
}
