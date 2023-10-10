pub struct Board {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<bool>>,
    pub new_cells: Vec<Vec<bool>>
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

    pub fn set_coords(&mut self, x: u32, y: u32, status: bool) {
        self.cells[x as usize][y as usize] = status;
    }

    pub fn get_neighbor_count(&self, x: i32, y: i32) -> u32 {
        let mut count: u32 = 0;

        for row in x - 1..=x + 1 {
            for col in y - 1..=y + 1 {
                let mut new_row = row;
                let mut new_col = col;

                if row == x && col == y {
                    continue;
                } else {
                    if row < 0 {
                        new_row = self.height + row;
                    } else if row >= self.height {
                        new_row -= self.height;
                    }

                    if col < 0 {
                        new_col = self.width + col;
                    } else if col >= self.width {
                        new_col -= self.width;
                    }

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
        let mut x_pos = 0;

        for (y_pos, row) in self.cells.iter().enumerate() {
            for _ in row {
                let cell_status = self.evolve_cell(x_pos, y_pos as i32);
                self.new_cells[y_pos][x_pos as usize] = cell_status;
                x_pos += 1;
            }
            x_pos = 0;
        }

        self.cells = self.new_cells.to_vec();
    }

    pub fn get_cell_status(&self, x: u32, y: u32) -> bool {
        self.cells[x as usize][y as usize]
    }

    pub fn reset(&mut self) {
        for col in &mut self.cells {
            for row in col {
                *row = false;
            }
        }
    }
}
