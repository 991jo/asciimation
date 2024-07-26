use super::prelude::*;
use crate::utils::{clip, plot_line};

use crate::frame::Character;
use nalgebra::{Rotation2, Scale2, Vector2};

/// describes an array of Hexagons.
/// r is the radius from the center of a hexagon to one of the corners.
pub struct Hexagons {
    step: usize,
    r: f32,
}

impl Default for Hexagons {
    fn default() -> Self {
        Hexagons { step: 0, r: 5.0 }
    }
}

impl Animation for Hexagons {
    fn name(&self) -> &'static str {
        "Hexagons"
    }

    fn author(&self) -> &'static str {
        "Jo"
    }

    fn render(&mut self, frame: &mut Frame) {
        self.step += 1;

        let angle = self.step as f32 * std::f32::consts::PI * 2.0 / 360.0 * 0.1;

        self.r = (self.step as f32 * std::f32::consts::PI / 360.0).sin() + 5.0;

        let b = self.r * (3.0f32 / 4.0).sqrt();

        let x = frame.x as f32;
        let y = frame.y as f32;

        let mut y0 = -2.0 * self.r;

        let mut points: Vec<Vec<Vector2<f32>>> = Vec::new();

        let num_points_x = ((x + 4.0 * b) / (2.0 * b)) as usize;

        while y0 <= y + (2.0 * self.r) {
            for i in 0..4 {
                let mut x0 = match i {
                    1..=2 => -2.0 * b,
                    _ => -b,
                };

                let mut row = Vec::new();
                for _ in 0..num_points_x {
                    let point = Vector2::new(x0, y0);
                    row.push(point);
                    x0 += 2.0 * b;
                }
                points.push(row);

                match i {
                    0 | 2 => y0 += 0.5 * self.r,
                    _ => y0 += self.r,
                }
            }
        }

        let rot = Rotation2::new(angle);
        let translate = Vector2::new(frame.x as f32 / 2.0, frame.y as f32 / 2.0);
        let scale = Scale2::new(1.0, 0.5) * 4.0;

        let points: Vec<Vec<Vector2<f32>>> = points
            .iter()
            .map(|x| {
                x.iter()
                    .map(|p| scale * (rot * (p - translate)) + translate)
                    .collect()
            })
            .collect();

        let frame_width = frame.x as f32;

        let color_func = |x: isize, y: isize| {
            let value = x as f32 / frame_width;
            let red = Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            };
            let blue = Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
            };
            Character {
                character: 'o',
                color: red.interpolate(&blue, value),
            }
        };

        for y in (0..points.len()).step_by(4) {
            for x in 0..(num_points_x - 1) {
                let p0 = points[y][x];
                let p1 = points[y + 1][x];
                let p2 = points[y + 1][x + 1];
                let p3 = points[y + 2][x];
                let p4 = points[y + 2][x + 1];
                let p5 = points[y + 3][x];
                let mut lines = vec![(p1, p0), (p0, p2), (p1, p3), (p3, p5), (p5, p4)];

                if y + 4 != points.len() {
                    let p6 = points[y + 4][x];
                    lines.push((p5, p6))
                }

                for (start, end) in lines {
                    match clip(start, end, 0.0, frame.x as f32, 0.0, frame.y as f32) {
                        Some((start, end)) => plot_line(frame, start, end, &color_func),
                        None => (),
                    }
                }
            }
        }
    }
}
