use super::prelude::*;
use crate::frame::HSVColor;
use crate::utils::sample;
use nalgebra::base::Vector2;
use rand::Rng;

#[derive(Default)]
pub struct Drops {
    step: usize,
    initialized: bool,
    centers: Vec<Vector2<f32>>,
}

impl Drops {
    fn initialize(&mut self, _frame: &mut Frame, ratio: f32) {
        let mut rng = rand::thread_rng();

        for _ in 0..3 {
            self.centers
                .push(Vector2::new(rng.gen::<f32>() * ratio, rng.gen()));
        }
    }
}

impl Animation for Drops {
    fn name(&self) -> &'static str {
        "Drops"
    }

    fn author(&self) -> &'static str {
        "Jo"
    }

    fn render(&mut self, frame: &mut Frame) {
        // calculate a frame height
        let ratio = frame.x as f32 / frame.y as f32 / 2.0;

        if !self.initialized {
            self.initialized = true;
            self.initialize(frame, ratio);
        }

        sample(
            frame,
            0.0,
            ratio,
            0.0,
            1.0,
            self.step as f32 / 60.0,
            &|x, y, time| drop(&self.centers, x, y, time),
        );
        self.step += 1;
    }
}

fn drop(centers: &[Vector2<f32>], x: f32, y: f32, time: f32) -> Character {
    let pos = Vector2::new(x, y);
    let grown_distance = time * 0.1;

    let mut height: f32 = 0.0;

    for center in centers.iter() {
        let distance = center.metric_distance(&pos);
        if distance > grown_distance {
            continue;
        } else {
            let distance_from_start = grown_distance - distance;

            height += (distance_from_start * 35.0).sin() * 0.3 / distance.clamp(0.0, 1.0) / 2.0;
        }
    }

    let mut color = HSVColor {
        h: (0.4f32 + 0.01 * time).rem_euclid(1.0),
        s: 1.0,
        v: height.abs().clamp(0.0, 1.0),
    };
    if height < 0.0 {
        color.h = (color.h + 0.5).rem_euclid(1.0);
    }

    Character {
        color: color.into(),
        character: '@',
    }
}
