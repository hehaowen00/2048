pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

impl Action {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            'u' => Some(Action::Up),
            'd' => Some(Action::Down),
            'l' => Some(Action::Left),
            'r' => Some(Action::Right),
            'q' => Some(Action::Quit),
            _ => None,
        }
    }
}

