use crate::animations::{Animation, Rainbow, RandomWalkers, TextOverlay};
use crate::frame::Frame;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

use terminal_size::terminal_size;

pub mod animations;
pub mod frame;

fn main() {
    let mut last_step = time::Instant::now();
    let step_length = time::Duration::from_millis(16);
    let mut animation = RandomWalkers::default();
    let should_run = Arc::new(AtomicBool::new(true));
    ctrlc::set_handler({
        let should_run = should_run.clone();
        move || {
            should_run.store(false, Ordering::SeqCst);
        }
    })
    .expect("Error setting handler for Ctrl+C");

    while should_run.load(Ordering::SeqCst) {
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
    // Show cursor again
    print!("\x1B[?25h");
}
