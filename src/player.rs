use crate::action::Action;
use crate::grid::Grid;
use std::io::{Write, stdin, stdout};

pub trait Player {
    fn next_action(&mut self, grid: &Grid) -> Option<Action>;
}

impl Player for Box<dyn Player> {
    fn next_action(&mut self, grid: &Grid) -> Option<Action> {
        (**self).next_action(grid) 
    }
}

#[derive(Debug)]
pub struct Human {
    buffer: String,
}

impl Human {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}

impl Player for Human {
    fn next_action(&mut self, _grid: &Grid) -> Option<Action> {
        print!("Action: ");
        let _ = stdout().flush();
        let _ =  stdin().read_line(&mut self.buffer);

        let ch = self.buffer.chars().next()?; 
        self.buffer.clear();
        Some(Action::from_char(ch))
    }
}
