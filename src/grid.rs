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
}
