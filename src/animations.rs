use crate::frame::Frame;

mod gol;
mod hills;
mod matrix;
mod pixel;
mod prelude;
mod qrcode;
mod rainbow;
mod random_walkers;
mod text_overlay;
pub use gol::GOL;
pub use hills::Hills;
pub use matrix::Matrix;
pub use pixel::Pixels;
pub use qrcode::QrCode;
pub use rainbow::Rainbow;
pub use random_walkers::RandomWalkers;
pub use text_overlay::TextOverlay;

pub trait Animation {
    /// returns the name of the animation
    fn name(&self) -> &'static str;

    /// returns the author of the animation
    fn author(&self) -> &'static str;

    /// writes the next animation step into the given frame.
    fn render(&mut self, frame: &mut Frame);
}
