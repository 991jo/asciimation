# ASCIImation

This is a collection of interesting animations that output to ASCII in the terminal.

# Running

# Contributing

Feel free to add your own animations.

Your animation has to be a struct that implements the `animation::Animation` Trait.

The `name()` and `author()` functions should return the name of your animation and your name.

The `render()` function gets a `frame::Frame` in which it has to write it's output.
This function is called about 60 times per second.
If your render function takes longer you are reducing the frame rate.

Keep in mind that the size of the frame can change between different executions of `render()` when the terminal
is resized.

As a starting point, this is how a very basic animation (assuming it is it's own file in `animations/`) looks:
``` Rust
use super::prelude::*;

pub struct Basic;

impl Animation for Basic {
    fn name(&self) -> &'static str {
        "basic"
    }

    fn author(&self) -> &'static str {
        "Your Name"
    }

    fn render(&mut self, frame: &mut Frame) {
        // Just rendering 01234.. on line 23.
        let y = 23;
        for x in 0..frame.x {
            let character = Character {
                character: char::from_digit(x as u32 % 10, 10).unwrap(),
                color: HSVColor {
                    h: 0.908,
                    s: 0.71,
                    v: 0.87,
                }
                .into(),
            };
            frame.set_at(x, y, character);
        }
    }
}
```

Additionally you need to include your new animation into `animations.rs`:
```Rust
mod basic;
pub use basic::Basic;
```

To run your animation, add it to the animations in `src/bin/shuffle.rs`.
(For testing you can comment out the other animations).
