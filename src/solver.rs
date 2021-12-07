use crate::action::{Action, Direction};
use crate::grid::Grid;
use crate::player::Player;
use crate::util::{dot_product, transpose};

pub struct Expectimax;

impl Expectimax {
    pub fn new() -> Self {
        Self {}
    }
}

impl Player for Expectimax {
    fn next_action(&mut self, grid: &Grid) -> Option<Action> {
        let depth = 6;
        let (action, _) = max(grid, depth);

        match action {
            Some(direction) => {
                println!("Action: {:?}", direction);
                Some(Action::Move(direction))
            },
            _ => None,
        }
    }
}

fn max(grid: &Grid, depth: u16) -> (Option<Direction>, f64) {
    if depth == 0 {
        return (None, heuristic(grid));
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

        let score = chance(&copy, depth - 1);

        if score > best_score {
            best_action = Some(action);
            best_score = score;
        }
    }

    (best_action, best_score)
}

fn chance(grid: &Grid, depth: u16) -> f64 {
    if depth == 0 {
        return heuristic(grid);
    }

    let mut score = 0.0;
    let mut copy = grid.clone();

    let empty = copy.get_empty();
    let count = empty.len() as f64;

    for (i, j) in empty {
        copy.cells[i][j] = 4;
        let (_, s1) = max(&copy, depth - 1);

        copy.cells[i][j] = 2;
        let (_, s2) = max(&copy, depth - 1);

        copy.cells[i][j] = 0;
        score += (0.1 * s1 + 0.9 * s2) / count;
    }

    score
}

fn heuristic(grid: &Grid) -> f64 {
    let corner_max = corner_max(grid);
    let empty_tiles = empty_tiles(grid);
    let monotonicity = monotonicity(grid);
    let smoothness = smoothness(grid);
    let weighted_sum = weighted_sum(grid);
    let moves_count = moves_count(grid);

    corner_max + empty_tiles +  monotonicity +
    moves_count + smoothness + weighted_sum
}

fn corner_max(grid: &Grid) -> f64 {
    let mut max_value = 0;
    
    for row in grid.cells {
        for el in row {
            if el > max_value {
                max_value = el;
            }
        }
    }

    match grid.cells[0][0] == max_value {
        true => max_value as f64,
        false => 0.0,
    }
}

fn empty_tiles(grid: &Grid) -> f64 {
    let empty = grid.get_empty();
    let len = empty.len();

    (len * len) as f64
}

fn monotonicity(grid: &Grid) -> f64 {
    let mut score = 0.0;

    for row in grid.cells {
        let mut prev = row[0] as f64;
        for el in &row[1..] {
            let el = *el as f64;
            if el == prev {
                score += 1.0;
            }
            prev = el;
        }
    }

    let mut g2 = grid.clone();
    transpose(&mut g2.cells);

    for row in g2.cells {
        let mut prev = row[0] as f64;
        for el in &row[1..] {
            let el = *el as f64;
            if el == prev {
                score += 1.0;
            }
            prev = el;
        }
    }

    score
}

fn moves_count(grid: &Grid) -> f64 {
    let moves = grid.get_actions();
    let len = moves.len();

    match len {
        0 => f64::NEG_INFINITY,
        _ => (len * len) as f64
    }
}

fn smoothness(grid: &Grid) -> f64 {
    let mut score = 0.0;

    for row in grid.cells {
        let mut prev = row[0] as f64;
        for el in &row[1..] {
            let el = *el as f64;
            if prev == 0.0 || el == 0.0 {
                prev = el;
                continue;
            }
            score -= (prev - el).abs();
            prev = el;
        }
    }

    let mut g2 = grid.clone();
    transpose(&mut g2.cells);

    for row in g2.cells {
        let mut prev = row[0] as f64;
        for el in &row[1..] {
            let el = *el as f64;
            if prev == 0.0 || el == 0.0 {
                prev = el;
                continue;
            }
            score -= (prev - el).abs();
            prev = el;
        }
    }

    score
}

fn weighted_sum(grid: &Grid) -> f64 {
    const WEIGHTS: [[f64; 4]; 4] = [
        [13.0,8.0,5.0,3.0],
        [0.0,0.5,1.0,2.0],
        [-0.25,-0.5,-0.75,-1.25],
        [-8.5,-5.25,-3.25,-2.0],
    ];

    dot_product(&grid.cells, &WEIGHTS)
}
