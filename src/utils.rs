use crate::frame::{Character, Frame};

pub fn fill_block(
    frame: &mut Frame,
    x: isize,
    y: isize,
    width: usize,
    height: usize,
    char: Character,
) {
    for x_index in 0..width {
        for y_index in 0..height {
            let x = x + x_index as isize;
            let y = y + y_index as isize;
            frame.set_at_clipping(x, y, char.clone());
        }
    }
}
