# ASCIImation

This is a collection of interesting animations that output to ASCII in the terminal.

# Running

# Contributing

Feel free to add your own animations.
(It would be nice ifyour code is formatted by running `cargo fmt` and passes `cargo clippy` without any issues.)

Your animation has to be a struct that implements the `animation::Animation` Trait.

The `name()` and `author()` functions should return the name of your animation and your name.

The `render()` function gets a `frame::Frame` in which it has to write it's output.
This function is called about 60 times per second.
If your render function takes longer you are reducing the frame rate.

Keep in mind that the size of the frame can change between different executions of `render()` when the terminal
is resized.

To run your animation, add it to the animations in `src/bin/shuffle.rs`.
(For testing you can comment out the other animations).
