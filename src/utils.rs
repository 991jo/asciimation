use crate::frame::{Character, Frame};
use nalgebra::Vector2;

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

pub fn clip(
    p1: Vector2<f32>,
    p2: Vector2<f32>,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> Option<(Vector2<f32>, Vector2<f32>)> {
    let wec_left = |p: f32| p - x_min;
    let wec_right = |p: f32| x_max - p;
    let wec_bottom = |p: f32| p - y_min;
    let wec_top = |p: f32| y_max - p;

    let op1 = [
        wec_left(p1.x),
        wec_right(p1.x),
        wec_top(p1.y),
        wec_bottom(p1.y),
    ];
    let op2 = [
        wec_left(p2.x),
        wec_right(p2.x),
        wec_top(p2.y),
        wec_bottom(p2.y),
    ];

    let outcode1 = op1.map(|x| x.is_sign_negative());
    let outcode2 = op2.map(|x| x.is_sign_negative());

    // trivial accept
    if !(outcode1[0]
        || outcode1[1]
        || outcode1[2]
        || outcode1[3]
        || outcode2[0]
        || outcode2[1]
        || outcode2[2]
        || outcode2[3])
    {
        return Some((p1, p2));
    }

    // trivial reject
    for (i, out1) in outcode1.iter().enumerate() {
        let out2 = outcode2[i];

        if *out1 && out2 {
            return None;
        }
    }

    let mut a_min: f32 = 0.0;
    let mut a_max: f32 = 1.0;

    for (i, out1) in op1.iter().enumerate() {
        let out2 = &op2[i];

        let a_s = out1 / (out1 - out2);

        if outcode1[i] {
            a_min = a_min.max(a_s);
        } else if outcode2[i] {
            a_max = a_max.min(a_s);
        }
    }

    if a_min > a_max {
        return None;
    }

    Some((p1 + a_min * (p2 - p1), p1 + a_max * (p2 - p1)))
}

/// plots the line for lines with slope between [-1, 1].
pub fn plot_line_low(
    frame: &mut Frame,
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
    character: &dyn Fn(isize, isize) -> Character,
) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;

    let mut yi: isize = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }

    let mut d = (2 * dy) - dx;
    let mut y = y0;

    for x in x0..=x1 {
        frame.set_at_clipping(x, y, character(x, y));

        if d > 0 {
            y += yi;
            d += 2 * (dy - dx);
        } else {
            d += 2 * dy;
        }
    }
}

/// plots the line for lines with slope between [-1, 1].
pub fn plot_line_high(
    frame: &mut Frame,
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
    character: &dyn Fn(isize, isize) -> Character,
) {
    let mut dx = x1 - x0;
    let dy = y1 - y0;

    let mut xi: isize = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }

    let mut d = (2 * dx) - dy;
    let mut x = x0;

    for y in y0..=y1 {
        frame.set_at_clipping(x, y, character(x, y));

        if d > 0 {
            x += xi;
            d += 2 * (dx - dy);
        } else {
            d += 2 * dx;
        }
    }
}

pub fn plot_line(
    frame: &mut Frame,
    start: Vector2<f32>,
    end: Vector2<f32>,
    character: &dyn Fn(isize, isize) -> Character,
) {
    let x0 = start.x as isize;
    let y0 = start.y as isize;

    let x1 = end.x as isize;
    let y1 = end.y as isize;

    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            plot_line_low(frame, x1, y1, x0, y0, character);
        } else {
            plot_line_low(frame, x0, y0, x1, y1, character);
        }
    } else {
        if y0 > y1 {
            plot_line_high(frame, x1, y1, x0, y0, character);
        } else {
            plot_line_high(frame, x0, y0, x1, y1, character);
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_clip() {
        // trivial accept
        let p1 = Vector2::new(1.0, 1.0);
        let p2 = Vector2::new(2.0, 2.0);

        let (r1, r2) = clip(p1, p2, -2.0, 4.0, 0.5, 2.5).unwrap();

        assert_eq!(p1, r1);
        assert_eq!(p2, r2);

        // trivial reject
        assert_eq!(clip(p1, p2, -2.0, 0.0, 0.5, 2.5), None);

        // test actual clip
        let (r1, r2) = clip(p1, p2, 1.5, 4.0, -0.5, 2.5).unwrap();

        let expected_r1 = Vector2::new(1.5, 1.5);

        assert!((r1.x - expected_r1.x).abs() < 1.0e-5);
        assert!((r1.y - expected_r1.y).abs() < 1.0e-5);
        assert_eq!(p2, r2);
        // assert_eq!(p2, r1);
    }
}
