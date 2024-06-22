use asciimation::animations::{Animation, Rainbow, RandomWalkers, TextOverlay};
use asciimation::frame::Frame;
use std::thread;
use std::time;

use terminal_size::terminal_size;

fn main() {
    let mut animations: Vec<fn() -> Box<dyn Animation>> = Vec::new();

    animations.push(|| Box::new(Rainbow::default()));
    animations.push(|| Box::new(RandomWalkers::default()));

    let mut last_step = time::Instant::now();
    let step_length = time::Duration::from_millis(16);

    let animation_duration = time::Duration::from_secs(5);

    loop {
        for animation_fn in animations.iter() {
            let mut animation = animation_fn();

            let animation_start = time::Instant::now();

            while animation_start + animation_duration >= time::Instant::now() {
                let size = terminal_size();
                let (width, height) = size.unwrap();

                // sleep until we are ready
                let sleep_time = step_length - last_step.elapsed();

                if sleep_time > time::Duration::ZERO {
                    thread::sleep(sleep_time);
                }

                // build a frame
                let mut frame = Frame::new(width.0 as usize, height.0 as usize);
                animation.render(&mut frame);

                // insert an overlay
                let mut overlay = TextOverlay {
                    text: format!(
                        "Resolution: {}, {}\nAnimation: {}\nBy: {}",
                        width.0,
                        height.0,
                        animation.name(),
                        animation.author()
                    ),
                };

                overlay.render(&mut frame);

                frame.render();
                last_step = time::Instant::now();
            }
        }
    }
}
