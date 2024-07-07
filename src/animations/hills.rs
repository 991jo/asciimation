use super::prelude::*;
use nalgebra::Vector2;
use rand::Rng;

struct Hill {
    pos: Vector2<f32>,
    direction: Vector2<f32>,
    size: f32,
    height: f32,
}

impl Hill {
    fn random(x: f32, y: f32) -> Hill {
        let mut rng = rand::thread_rng();

        let pos = Vector2::new(rng.gen_range(0.0..(x + 1.0)), rng.gen_range(0.0..(y + 1.0)));
        let direction = Vector2::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5) * 0.2;

        let size = rng.gen::<f32>() * (x.min(y));
        let height = rng.gen::<f32>() * 0.5;

        Hill {
            pos,
            direction,
            size,
            height,
        }
    }

    fn eval(&self, point: Vector2<f32>) -> f32 {
        let distance = self.pos - point;
        let distance = distance / self.size;

        (-(distance.x.powi(2) + distance.y.powi(2))).exp() * self.height
    }
}

pub struct Hills {
    hills: Vec<Hill>,
    initialized: bool,
    x: f32,
    y: f32,
}

impl Default for Hills {
    fn default() -> Self {
        Hills {
            hills: Vec::new(),
            initialized: false,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Hills {
    const NAME: &'static str = "Hills";
    const AUTHOR: &'static str = "Jo";

    fn initialize(&mut self, frame: &Frame) {
        if self.initialized {
            return;
        };

        self.x = frame.x as f32;
        self.y = frame.y as f32;

        for _ in 0..10 {
            self.hills.push(Hill::random(self.x, self.y));
        }
        self.initialized = true;
    }

    fn step(&mut self, frame: &Frame) {
        for hill in self.hills.iter_mut() {
            hill.pos += hill.direction;
            hill.pos.x = hill.pos.x.rem_euclid(frame.x as f32 + 1.0);
            hill.pos.y = hill.pos.y.rem_euclid(frame.y as f32 + 1.0);
        }
    }

    fn eval(&self, x: usize, y: usize) -> f32 {
        let mut value = 0.0;

        for hill in self.hills.iter() {
            let mut best_value: f32 = 0.0;

            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    best_value = best_value.max(hill.eval(Vector2::new(
                        x as f32 + (x_offset as f32 * self.x),
                        y as f32 + (y_offset as f32 * self.y),
                    )));
                }
            }

            value += best_value;
        }

        value
    }
}

impl Animation for Hills {
    fn name(&self) -> &'static str {
        Hills::NAME
    }
    fn author(&self) -> &'static str {
        Hills::AUTHOR
    }

    fn render(&mut self, frame: &mut Frame) {
        self.initialize(frame);
        self.step(frame);

        for x in 0..frame.x {
            for y in 0..frame.y {
                let cell = frame.get_mut(x, y);

                let value = self.eval(x, y);

                let char = value_to_char(value);

                cell.character = char;
                cell.color = Color::from(HSVColor {
                    h: value.rem_euclid(1.0),
                    s: 1.0,
                    v: 1.0,
                });
            }
        }
    }
}
