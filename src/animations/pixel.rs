use super::prelude::*;
use nalgebra::DMatrix;
pub struct Pixels {
    pub pos_top_left: (usize, usize),
    pub image: DMatrix<bool>,
}

impl Pixels {
    // Return value of image at coordinates or false if out of bounds.
    fn get_image_at(&self, x: usize, y: usize) -> bool {
        self.image.get((x, y)).copied().unwrap_or(false)
    }
}

impl Animation for Pixels {
    fn name(&self) -> &'static str {
        "pixels"
    }

    /// returns the author of the animation
    fn author(&self) -> &'static str {
        "Imarok"
    }

    /// writes the next animation step into the given frame.
    fn render(&mut self, frame: &mut Frame) {
        for (x, _) in self.image.row_iter().enumerate() {
            for (y, _) in self.image.column_iter().enumerate().step_by(2) {
                let val = (self.get_image_at(x, y), self.get_image_at(x, y + 1));
                let character = match val {
                    (false, false) => ' ',
                    (true, false) => '▀',
                    (false, true) => '▄',
                    (true, true) => '█',
                };
                let frame_pixel =
                    frame.get_mut(x + self.pos_top_left.0, y / 2 + self.pos_top_left.1);
                *frame_pixel = Character {
                    character,
                    color: HSVColor {
                        h: 0.0,
                        s: 0.0,
                        v: 0.4,
                    }
                    .into(),
                };
            }
        }
    }
}
