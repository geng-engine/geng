use geng::prelude::*;

mod ball;
mod collision;
mod game_state;
mod player;

use game_state::GameState;

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new_with(geng::ContextOptions {
        title: "Pong".to_owned(),
        antialias: true,
        ..default()
    });
    let state = GameState::new(&geng);
    geng.run(state);
}
