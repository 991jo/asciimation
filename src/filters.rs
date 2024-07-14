use crate::frame::{Character, Color, Frame};
use rand::seq::SliceRandom;

/// Fades the given frame to a blank screen.
///
/// a fade value of 1.0 does not alter the frame, a value of 0.0 is completly blank.
///
/// The screen is faded out by scaling down the colors with the given values and setting them to
/// whitespace if they are below a certain luminance threshold.
pub fn fadeout(frame: &mut Frame, fade: f32) {
    for char in frame.data.iter_mut() {
        char.color.scale(fade);

        if char.color.luminance() < 0.1 {
            char.character = ' ';
        }
    }
}
