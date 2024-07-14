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

/// Samples the function on a grid given by the frame, scaled to the
/// given x/y min/max values.
///
/// The paramters of f are x, y and a time coordinate.
pub fn sample(
    frame: &mut Frame,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    time: f32,
    f: &dyn Fn(f32, f32, f32) -> Character,
) {
    let width = x_max - x_min;
    let height = y_max - y_min;

    for x in 0..frame.x {
        let x_query = x_min + (x as f32 / frame.x as f32 * width);
        for y in 0..frame.y {
            let y_query = y_min + (y as f32 / frame.y as f32 * height);

            frame.set_at_clipping(x as isize, y as isize, f(x_query, y_query, time))
        }
    }
}
