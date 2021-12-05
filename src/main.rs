mod action;
mod game;
mod grid;
mod player;
mod solver;
mod util;

use clap::{App, Arg};
use game::Game;
use player::{Player, Human};
use solver::Expectimax;

fn main() {
    let matches = App::new("2048")
        .version("1.0")
        .author("hehaowen00")
        .about("Rust implementation of 2048 with an Expectimax solver")
        .arg(Arg::with_name("solver")
             .long("solver")
             .short("s")
             .help("Selects the solver to use")
             .takes_value(true)
             .required(false))
        .get_matches();

    let player: Box<dyn Player> = match matches.value_of("solver") {
        Some("expectimax") => {
            Box::new(Expectimax::new())
        },
        Some(s) => {
            println!("error: unknown solver `{}`", s);
            return;
        }
        None => {
            Box::new(Human::new())
        },
    };

    let mut game = Game::new(player);
    game.run();
}
