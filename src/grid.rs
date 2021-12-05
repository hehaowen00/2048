use crate::action::Direction;
use crate::util::{WIDTH, reverse, transpose};
use rand::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid {
    pub cells: [[u32; WIDTH]; WIDTH],
    pub width: usize,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [[0; WIDTH]; WIDTH],
            width: WIDTH,
        }
    }

    pub fn add_tiles(&mut self, count: usize) {
        let mut empty = self.get_empty();
        let mut rng = thread_rng();

        if empty.len() == 0 {
            return;
        }

        for _ in 0..count {
            match empty.len() {
                0 => return,
                len => {
                    let idx = rng.gen_range(0..len);
                    let prob_v = rng.gen_range(0.0..1.0);

                    if prob_v < 0.9 {
                        self[empty[idx]] = 2;
                    } else {
                        self[empty[idx]] = 4;
                    }

                    empty.remove(idx);
                }
            }
        }
    }

    pub fn get_empty(&self) -> Vec<(usize, usize)> {
        let mut empty = Vec::new();

        for i in 0..self.width {
            for j in 0..self.width {
                if self[(i,j)] == 0 {
                    empty.push((i,j));
                }
            }
        }

        empty
    }

    pub fn get_actions(&self) -> Vec<Direction> {
        let mut actions = Vec::with_capacity(4);
        let mut copy = self.cells.clone();

        transpose(&mut copy);
        if Self::test_merge(&mut copy) {
            actions.push(Direction::Up);
        }
        transpose(&mut copy);

        if Self::test_merge(&mut copy) {
            actions.push(Direction::Left);
        }

        reverse(&mut copy);
        if Self::test_merge(&mut copy) {
            actions.push(Direction::Right);
        }
        reverse(&mut copy);

        transpose(&mut copy);
        reverse(&mut copy);

        if Self::test_merge(&mut copy) {
            actions.push(Direction::Down);
        }

        actions
    }

    pub fn shift(&mut self, direction: Direction) -> (bool, u32) {
        match direction {
            Direction::Up => {
                transpose(&mut self.cells);
                let result = Self::merge(&mut self.cells);
                transpose(&mut self.cells);
                result
            },
            Direction::Down => {
                transpose(&mut self.cells);
                reverse(&mut self.cells);
                let result = Self::merge(&mut self.cells);
                reverse(&mut self.cells);
                transpose(&mut self.cells);
                result
            },
            Direction::Left => {
                Self::merge(&mut self.cells)
            },
            Direction::Right => {
                reverse(&mut self.cells);
                let result = Self::merge(&mut self.cells);
                reverse(&mut self.cells);
                result
            },
        }
    }

    fn merge(grid: &mut [[u32; WIDTH]; WIDTH]) -> (bool, u32) {
        let mut changed = false;
        let mut score = 0;

        for i in 0..WIDTH {
            for j in 0..WIDTH {
                let mut a = grid[i][j];

                if a == 0 {
                    for k in j + 1 ..WIDTH {
                        let b = grid[i][k];
                        if b > 0 {
                            grid[i][j] = b;
                            grid[i][k] = 0;
                            a = b;
                            changed = true;
                            break;
                        }
                    }
                }

                if a == 0 {
                    continue;
                }

                for k in j + 1 ..WIDTH {
                    let b = grid[i][k];

                    if a == b {
                        grid[i][j] += b;
                        grid[i][k] = 0;
                        score += grid[i][j];
                        changed = true;
                        break;
                    } 

                    if b != 0 {
                        break;
                    }
                }
            }
        }

        (changed, score)
    }

    fn test_merge(grid: &mut [[u32; WIDTH]; WIDTH]) -> bool {
        for i in 0..WIDTH {
            for j in 0..WIDTH {
                let a = grid[i][j];

                if a == 0 {
                    for k in j + 1 ..WIDTH {
                        let b = grid[i][k];
                        if b > 0 {
                            return true;
                        }
                    }
                }

                if a == 0 {
                    continue;
                }

                for k in j + 1 ..WIDTH {
                    let b = grid[i][k];

                    if a == b {
                        return true;
                    } 

                    if b != 0 {
                        break;
                    }
                }
            }
        }

        false
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = u32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.0][index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[index.0][index.1]
    }
}
