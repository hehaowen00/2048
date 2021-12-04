mod action;
mod game;
mod grid;
mod player;
mod util;

use game::Game;
use player::Human;

fn main() {
    let mut game = Game::new(Human::new());
    game.run();
}
