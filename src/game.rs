use crate::action::Action;
use crate::grid::Grid;
use crate::player::Player;
use crate::util::{WIDTH, CELL_WIDTH};

use std::fmt;

#[derive(Debug)]
pub struct Game<P: Player> {
    status: Status,
    score: u32,
    grid: Grid,
    player: P,
    separator: String,
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    GameOver,
    Running,
    Exit,
}

impl<P> Game<P>
where
    P: Player
{
    pub fn new(player: P) -> Self {
        let mut grid = Grid::new();
        grid.add_tiles(2);

        let len = (WIDTH * CELL_WIDTH) + WIDTH + 1;
        let separator = "-".repeat(len);

        Self {
            status: Status::Running,
            score: 0,
            grid,
            player,
            separator
        }
    }

    pub fn render(&self) {
        println!("{}", self);
    }

    pub fn process(&mut self, action: Action) {
        match action {
            Action::Exit => {
                self.status = Status::Exit;
            },
            Action::Move(direction) => {
                let (changed, score) = self.grid.shift(direction);

                if changed {
                    self.grid.add_tiles(1);

                    let actions = self.grid.get_actions();
                    if actions.len() == 0 {
                        self.status = Status::GameOver;
                    }
                }

                if score > 0 {
                    self.score += score;
                }
            },
            Action::NOOP => {}
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.status {
                Status::Exit => break,
                Status::GameOver => {
                    let padding = self.separator.len() - 2;
                    println!("\n{}", self.separator);
                    println!("|{: ^padding$}|", "Game Over", padding=padding);
                    println!("{}", self.separator);
                    break;
                },
                Status::Running => {
                    self.render();

                    if let Some(action) = self.player.next_action(&self.grid) {
                        self.process(action);
                    }
                }
            }
        }
    }
}

impl<P> fmt::Display for Game<P>
where
    P: Player
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const TITLE_LEN: usize = 6;
        const SCORE_LEN: usize = 16;

        let spacing = self.separator.len() - TITLE_LEN - SCORE_LEN;
        write!(f, "\n[2048]{:spacing$}Score:{:10}", "", self.score, spacing=spacing)?;

        for i in 0..WIDTH {
            write!(f, "\n{}\n|", self.separator)?;

            for j in 0..WIDTH {
                write!(f, "{: ^padding$}|", self.grid[(i,j)], padding=CELL_WIDTH)?;
            }
        }

        write!(f, "\n{}", self.separator)
    }
}

