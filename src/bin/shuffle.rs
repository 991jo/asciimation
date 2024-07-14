use asciimation::animations::{
    Animation, Matrix, MovingBlocks, QrCode, Rainbow, RandomWalkers, TextOverlay, GOL,
};
use asciimation::frame::Frame;
use clap::Parser;
use std::thread;
use std::time;

use terminal_size::terminal_size;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable debug output
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Time in seconds each animation is shown
    #[arg(short, long, default_value_t = 60)]
    animation_time: usize,
}

fn main() {
    let args = Args::parse();

    let animations: Vec<fn() -> Box<dyn Animation>> = vec![
        || Box::<MovingBlocks>::default(),
        || Box::<Rainbow>::default(),
        || Box::<RandomWalkers>::default(),
        || Box::<GOL>::default(),
        || Box::new(QrCode::new("https://github.com/991jo/asciimation", (5, 6))),
        || Box::<Matrix>::default(),
    ];

    let mut step_start = time::Instant::now();
    let step_length = time::Duration::from_millis(16);

    let animation_duration = time::Duration::from_secs(args.animation_time as u64);

    // handle exit via Ctrl+C/SIGINT
    ctrlc::set_handler({
        || {
            // Show cursor again
            print!("\x1B[?25h");
            std::process::exit(1);
        }
    })
    .expect("Error setting handler for Ctrl+C");

    loop {
        for animation_fn in animations.iter() {
            let mut animation = animation_fn();

            let animation_start = time::Instant::now();

            step_start = time::Instant::now();

            while animation_start + animation_duration >= step_start {
                let animation_time_remaining = (animation_start + animation_duration) - step_start;
                let size = terminal_size();
                let (width, height) = size.unwrap();

                // build a frame
                let mut frame = Frame::new(width.0 as usize, height.0 as usize);
                animation.render(&mut frame);

                let elapsed = step_start.elapsed();

                // insert an overlay
                let mut overlay = TextOverlay {
                    text: format!(
                        "Resolution: {}, {}\nAnimation: {}\nBy: {}",
                        width.0,
                        height.0,
                        animation.name(),
                        animation.author(),
                    ),
                };

                if args.debug {
                    // insert an overlay
                    overlay = TextOverlay {
                        text: format!(
                                  "Resolution: {}, {}\nAnimation: {}\nBy: {}\nRender Time:{}/{}µs\nTime remaining: {}s",
                                  width.0,
                                  height.0,
                                  animation.name(),
                                  animation.author(),
                                  elapsed.as_micros(),
                                  step_length.as_micros(),
                                  animation_time_remaining.as_secs(),
                              ),
                    };
                }

                overlay.render(&mut frame);

                frame.render();

                if elapsed < step_length {
                    let sleep_time = step_length - elapsed;
                    thread::sleep(sleep_time);
                }
                step_start = time::Instant::now();
            }
        }
    }
}
