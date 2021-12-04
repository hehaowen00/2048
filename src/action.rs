#[derive(Debug, Clone, Copy)]
pub enum Action {
    NOOP,
    Move(Direction),
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Action {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'u' => Action::Move(Direction::Up),
            'd' => Action::Move(Direction::Down),
            'l' => Action::Move(Direction::Left),
            'r' => Action::Move(Direction::Right),
            'q' =>  Action::Exit,
            _ => Action::NOOP
        }
    }
}
