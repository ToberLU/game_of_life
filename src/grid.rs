use rand::prelude::*;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        tracing::info!("init grid");

        Grid {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn randomize(&mut self, count: usize) {
        tracing::info!("randomize...");

        let mut rng = rand::rng();

        for _i in 0..count {
            let rand = rng.random_range(0..(self.width * self.height));
            self.cells[rand] = true;
        }

        //tracing::info!("{:?}", self.cells);
    }

    pub fn count_live_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let neighbor_row = row as i32 + i;
                let neighbor_col = col as i32 + j;
                // on est dans la grid
                if neighbor_row >= 0
                    && neighbor_row < self.height as i32
                    && neighbor_col >= 0
                    && neighbor_col < self.width as i32
                {
                    let neighbor_index = neighbor_row as usize * self.width + neighbor_col as usize;
                    if let Some(&true) = self.cells.get(neighbor_index) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}
