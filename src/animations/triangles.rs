use super::prelude::*;
use crate::utils::plot_line;

use nalgebra::Vector2;

#[derive(Default)]
pub struct Triangles {
    step: usize,
}

impl Animation for Triangles {
    fn name(&self) -> &'static str {
        "Triangles"
    }

    fn author(&self) -> &'static str {
        "Jo"
    }

    fn render(&mut self, frame: &mut Frame) {
        self.step += 1;

        let radius= (frame.x / 2).min(frame.y) as f32;
        let center = Vector2::new((frame.x/2) as f32, (frame.y/2) as f32);

        let angle1 = std::f32::consts::PI * self.step as f32 / 360.0;
        let angle2 = angle1 + (2.0 * std::f32::consts::PI / 3.0);
        let angle3 = angle2 + (2.0 * std::f32::consts::PI / 3.0);

        let p0 = Vector2::new(angle1.sin(), angle1.cos()/2.0) * radius + center;
        let p1 = Vector2::new(angle2.sin(), angle2.cos()/2.0) * radius + center;
        let p2 = Vector2::new(angle3.sin(), angle3.cos()/2.0) * radius + center;

        let character1 = Character { color: Color { r: 1.0, g: 0.0, b: 0.0 }, character: 'a' };
        let character2 = Character { color: Color { r: 0.0, g: 1.0, b: 0.0 }, character: 'b' };
        let character3 = Character { color: Color { r: 0.0, g: 0.0, b: 1.0 }, character: 'c' };

        let color_func = |_, _| character1.clone();

        plot_line(frame, p0, p1, &color_func);
        plot_line(frame, p1, p2, &color_func);
        plot_line(frame, p2, p0, &color_func);

    }
}
