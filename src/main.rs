mod action;
mod game;
mod grid;
mod player;
mod solver;
mod util;

use game::Game;
use player::Human;
use solver::Expectimax;

fn main() {
    // let mut game = Game::new(Human::new());
    let mut game = Game::new(Expectimax::new());
    game.run();
}
