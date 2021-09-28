use geng::prelude::*;

mod game_state;
mod player;
mod collision;
mod ball;

use game_state::GameState;

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Pong");
    let state = GameState::new(&geng);
    geng::run(&geng, state);
}
