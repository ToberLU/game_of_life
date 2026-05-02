use std::time::Duration;

use crate::grid::Grid;

pub struct Simulation {
    pub grid: Grid,
    pub paused: bool,
    pub delay_ms: Duration,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        tracing::info!("init Simulation");
        let mut grid = Grid::new(width, height);
        grid.randomize((width * height) / 20);

        Simulation {
            grid,
            paused: true,
            delay_ms: Duration::from_millis(500),
        }
    }

    pub fn update(&mut self) {
        let mut new_cells: Vec<bool> = vec![false; self.grid.width * self.grid.height];

        for i in 0..self.grid.height {
            for j in 0..self.grid.width {
                let is_alive = self.grid.cells[i * self.grid.width + j];
                let neighbors = self.grid.count_live_neighbors(i, j);
                new_cells[i * self.grid.width + j] =
                    matches!((is_alive, neighbors), (true, 2 | 3) | (false, 3));
            }
        }
        self.grid.cells = new_cells;
    }
}
