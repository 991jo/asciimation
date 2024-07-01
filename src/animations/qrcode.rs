use super::Animation;
use super::Pixels;
use crate::frame::Frame;

pub struct QrCode {
    pixels: Pixels,
}

impl Animation for QrCode {
    fn name(&self) -> &'static str {
        "QR Code"
    }

    /// returns the author of the animation
    fn author(&self) -> &'static str {
        "Imarok"
    }

    /// writes the next animation step into the given frame.
    fn render(&mut self, frame: &mut Frame) {
        self.pixels.render(frame)
    }
}

impl QrCode {
    pub fn new(text: &str, pos_top_left: (usize, usize)) -> Self {
        let pixels = qrcode::QrCode::new(text).unwrap().to_colors();
        use num_integer::Roots;
        // Height/width of the QRCode.
        let dimension = pixels.len().sqrt();
        let image = nalgebra::DMatrix::from_iterator(
            dimension,
            dimension,
            pixels.into_iter().map(|pixel| match pixel {
                qrcode::Color::Light => true,
                qrcode::Color::Dark => false,
            }),
        );
        // Add 2 pixels of white boarder.
        let image = image.insert_rows(dimension, 2, true);
        let image = image.insert_columns(dimension, 2, true);
        let image = image.insert_rows(0, 2, true);
        let image = image.insert_columns(0, 2, true);

        Self {
            pixels: Pixels {
                image,
                pos_top_left,
            },
        }
    }
}
