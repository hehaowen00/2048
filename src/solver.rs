use crate::action::{Action, Direction};
use crate::grid::Grid;
use crate::player::Player;
use crate::util::dot_product;

pub struct Expectimax {
}

impl Expectimax {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn max(grid: &Grid, depth: u16, max_depth: u16) -> (Option<Direction>, f64) {
        if depth == max_depth {
            return (None, Self::heuristic(grid));
        }

        let actions = grid.get_actions();
        let mut best_action = None;
        let mut best_score = f64::NEG_INFINITY;

        for action in actions {
            if best_action.is_none() {
                best_action = Some(action);
            }

            let mut copy = grid.clone();
            copy.shift(action);

            let score = Self::chance(&copy, depth + 1, max_depth);
            if score > best_score {
                best_action = Some(action);
                best_score = score;
            }
        }

        (best_action, best_score)
    }

    fn chance(grid: &Grid, depth: u16, max_depth: u16) -> f64 {
        if depth == max_depth {
            return Self::heuristic(grid);
        }

        let mut score = 0.0;
        let mut copy = grid.clone();

        let empty = copy.get_empty();
        let count = empty.len();

        for (i, j) in empty {
            copy.cells[i][j] = 2;
            let (_, score_2) = Self::max(&copy, depth + 1, max_depth);
            let weighted = (0.9 * score_2) / (count as f64);
            score += weighted;

            copy.cells[i][j] = 4;
            let (_, score_4) = Self::max(&copy, depth + 1, max_depth);
            let weighted = (0.1 * score_4) / (count as f64);
            score += weighted;

            copy.cells[i][j] = 0;
        }

        score
    }

    fn heuristic(grid: &Grid) -> f64 {
        let moves = grid.get_actions();
        if moves.len() == 0 {
            return f64::NEG_INFINITY;
        }

        let empty = grid.get_empty();
        let len = empty.len();

        const WEIGHTS: [[f64; 4]; 4] = [
            [13.0,8.0,5.0,3.0],
            [0.0,1.0,1.0,2.0],
            [-0.25,-0.5,-0.75,-1.25],
            [-8.5,-5.25,-3.25,-2.0],
        ];

        (len * len) as f64 + dot_product(&grid.cells, &WEIGHTS)
    }

}

impl Player for Expectimax {
    fn next_action(&mut self, grid: &Grid) -> Option<Action> {
        let empty = grid.get_empty();
        let max_depth = if empty.len() > 7 {
            4
        } else {
            8
        };

        let (action, _) = Self::max(grid, 0, max_depth);
        match action {
            Some(direction) => {
                println!("Action: {:?}", direction);
                Some(Action::Move(direction))
            },
            _ => None,
        }
    }
}
