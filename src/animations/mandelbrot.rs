use super::Animation;
use crate::frame::{value_to_char, Character, Color, Frame, HSVColor};
use nalgebra::{Complex, ComplexField};

pub struct Mandelbrot {
    width: f64,
}

impl Default for Mandelbrot {
    fn default() -> Mandelbrot {
        Mandelbrot { width: 8.0 }
    }
}

impl Mandelbrot {
    const NAME: &'static str = "Mandelbrot";
    const AUTHOR: &'static str = "Marco";
    const BOUND: f64 = 2.0;

    fn eval(&mut self, c: Complex<f64>) -> i32 {
        let mut z = Complex::new(0.0, 0.0);

        let max_iterations = self.calculate_max_iterations();

        for iteration in 0..max_iterations {
            z = z.powf(2.0) + c;

            if z.abs() > Mandelbrot::BOUND {
                return iteration;
            }
        }

        max_iterations
    }

    fn calculate_max_iterations(&self) -> i32 {
        (50.0 + (4.0 / self.width).log10().powf(2.0)).round() as i32
    }
}

impl Animation for Mandelbrot {
    fn name(&self) -> &'static str {
        Mandelbrot::NAME
    }

    fn author(&self) -> &'static str {
        Mandelbrot::AUTHOR
    }

    fn render(&mut self, frame: &mut Frame) {
        let height = self.width * (frame.y as f64 / frame.x as f64) * 2.5;
        let center = Complex::new(-0.608118878, -0.615161994);

        for y in 0..frame.y {
            let current_im = (y as f64 / frame.y as f64) * height - height / 2.0 + center.im;

            for x in 0..frame.x {
                let current_re =
                    (x as f64 / frame.x as f64) * self.width - self.width / 2.0 + center.re;

                let current_coord = Complex::new(current_re, current_im);
                let iterations_until_diverged = self.eval(current_coord);

                let num_colors = 30;
                let max_iterations = self.calculate_max_iterations();
                let value = if iterations_until_diverged == max_iterations {
                    1.0
                } else {
                    ((iterations_until_diverged % num_colors) as f32) / (num_colors as f32)
                };

                let the_char = value_to_char(value);

                frame.data[y * frame.x + x] = Character {
                    character: the_char,
                    color: Color::from(HSVColor {
                        h: value.rem_euclid(1.0),
                        s: 1.0,
                        v: 1.0,
                    }),
                }
            }
        }

        self.width *= 0.985;
    }
}
