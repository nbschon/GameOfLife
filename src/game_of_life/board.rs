use core::panic;

use rand::distributions::{Uniform, Distribution};

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<bool>>,
    pub new_cells: Vec<Vec<bool>>,
}

impl Board {
    pub fn with_size(width: i32, height: i32) -> Self {
        Board {
            width,
            height,
            cells: vec![vec![false; height as usize]; width as usize],
            new_cells: vec![vec![false; height as usize]; width as usize],
        }
    }

    pub fn get_neighbor_count(&self, x: i32, y: i32) -> u8 {
        let mut count = 0u8;

        for row in x - 1..=x + 1 {
            for col in y - 1..=y + 1 {
                if row != x || col != y {
                    let new_row = row.rem_euclid(self.height);
                    let new_col = col.rem_euclid(self.width);

                    if self.cells[new_col as usize][new_row as usize] {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn evolve_cell(&self, x: i32, y: i32) -> bool {
        let neighbors = self.get_neighbor_count(x, y);

        if self.cells[y as usize][x as usize] {
            matches!(neighbors, 2 | 3)
        } else {
            neighbors == 3
        }
    }

    pub fn step_game(&mut self) {
        for (y_pos, row) in self.cells.iter().enumerate() {
            for (x_pos, _) in row.iter().enumerate() {
                let cell_status = self.evolve_cell(x_pos as i32, y_pos as i32);
                self.new_cells[y_pos][x_pos as usize] = cell_status;
            }
        }

        std::mem::swap(&mut self.cells, &mut self.new_cells);
    }

    pub fn reset(&mut self) {
        for col in &mut self.cells {
            for row in col {
                *row = false;
            }
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(0..=1);

        for col in &mut self.cells {
            for row in col {
                *row = match dist.sample(&mut rng) {
                    0 => false,
                    1 => true,
                    _ => panic!("oh no!"),
                };
            }
        }
    }
}
