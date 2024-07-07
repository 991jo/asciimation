use super::prelude::*;
use rand::Rng;

pub struct GOL {
    data: Vec<bool>,
    step_counter: usize,
    speed: usize,
    x: usize,
    y: usize,
    initialized: bool,
}

impl GOL {
    const NAME: &'static str = "Game of Life";
    const AUTHOR: &'static str = "Jo";

    pub fn step(&mut self) {
        // only make a game step every self.speed steps
        if self.step_counter.rem_euclid(self.speed) != 0 {
            self.step_counter += 1;
            return;
        }
        self.step_counter += 1;

        let mut new_data = vec![false; self.x * self.y];

        for y in 0..self.y {
            for x in 0..self.x {
                let mut neighbors = 0;
                for x_offset in -1..=1 {
                    for y_offset in -1..=1 {
                        // skip the cell itself
                        if x_offset == 0 && y_offset == 0 {
                            continue;
                        }

                        let x_index = (x as i32 + x_offset).rem_euclid(self.x as i32) as usize;
                        let y_index = (y as i32 + y_offset).rem_euclid(self.y as i32) as usize;

                        if self.data[y_index * self.x + x_index] {
                            neighbors += 1;
                        }
                    }
                }
                if neighbors == 2 && self.data[y * self.x + x] || neighbors == 3 {
                    new_data[y * self.x + x] = true;
                }
            }
        }

        self.data = new_data;
    }

    pub fn initialize(&mut self) {
        let mut rng = rand::thread_rng();

        for cell in self.data.iter_mut() {
            if rng.gen::<f32>() < 0.25 {
                *cell = true;
            }
        }
    }
}

impl Default for GOL {
    fn default() -> GOL {
        GOL {
            data: vec![],
            step_counter: 0,
            speed: 8,
            x: 0,
            y: 0,
            initialized: false,
        }
    }
}

impl Animation for GOL {
    fn name(&self) -> &'static str {
        GOL::NAME
    }

    fn author(&self) -> &'static str {
        GOL::AUTHOR
    }

    fn render(&mut self, frame: &mut Frame) {
        // fill the data if it is not big enough
        while self.data.len() < frame.data.len() {
            self.data.push(false);
        }
        self.x = frame.x;
        self.y = frame.y;

        // initialize, if not done yet
        if !self.initialized {
            self.initialize();
            self.initialized = true;
        }

        self.step();

        // draw
        for (index, cell) in self.data.iter().enumerate() {
            if *cell {
                frame.data[index] = Character {
                    character: '@',
                    color: Color::white(),
                };
            }
        }
    }
}
