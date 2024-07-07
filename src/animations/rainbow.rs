use super::prelude::*;

pub struct Rainbow {
    color_shift: f32,
    rotation: f32,
}
impl Rainbow {
    const NAME: &'static str = "Rainbow";
    const AUTHOR: &'static str = "Jo";
}

impl Default for Rainbow {
    fn default() -> Rainbow {
        Rainbow {
            color_shift: 0.0,
            rotation: 0.0,
        }
    }
}

impl Animation for Rainbow {
    fn name(&self) -> &'static str {
        Rainbow::NAME
    }

    fn author(&self) -> &'static str {
        Rainbow::AUTHOR
    }

    fn render(&mut self, frame: &mut Frame) {
        self.color_shift += 0.01;
        self.rotation += 0.01;
        for y in 0..frame.y {
            for x in 0..frame.x {
                // translate coordinates to [0, 1]
                //
                // often characters in the terminal as twice as high as wide
                // however this is only an approximation.
                let long_edge = frame.x.max(frame.y * 2);

                let x_t = x as f32 / long_edge as f32;
                let y_t = 2.0 * y as f32 / long_edge as f32;

                let a_1 = self.rotation.cos();
                let a_2 = self.rotation.sin();

                let b_1 = -self.rotation.sin();
                let b_2 = self.rotation.cos();

                let l_1 = (x_t - (b_1 / b_2 * y_t)) / (a_1 - (b_1 / b_2 * a_2));

                // let hue = x as f32 / frame.x as f32;
                let hue = l_1;
                let hue = (hue + self.color_shift).rem_euclid(1.0);

                let hsv_color = HSVColor {
                    h: hue,
                    s: 1.0,
                    v: 1.0,
                };
                let color = Color::from(hsv_color);

                frame.data[y * frame.x + x] = Character {
                    character: 'A',
                    color,
                };
            }
        }
    }
}
