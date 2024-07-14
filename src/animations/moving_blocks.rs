use super::prelude::*;
use crate::utils::fill_block;

#[derive(Default)]
pub struct MovingBlocks {
    step: usize,
}

impl MovingBlocks {
    const CLOCK_DIVIDER: usize = 8;
    const BLOCK_SIZE: usize = 3;
}

impl Animation for MovingBlocks {
    fn name(&self) -> &'static str {
        "Moving Blocks"
    }

    fn author(&self) -> &'static str {
        "Jo"
    }

    fn render(&mut self, frame: &mut Frame) {
        // first set of blocks moving in x direction
        //
        let char_1 = Character {
            character: 'X',
            color: Color {
                r: 1.0,
                g: 0.839,
                b: 0.0,
            },
        };

        let mut y = 0;
        while y < frame.y {
            let mut x = -(MovingBlocks::BLOCK_SIZE as isize)
                + (self.step / MovingBlocks::CLOCK_DIVIDER + MovingBlocks::BLOCK_SIZE)
                    .rem_euclid(4 * MovingBlocks::BLOCK_SIZE) as isize;

            while x <= frame.x as isize {
                fill_block(
                    frame,
                    x,
                    y as isize,
                    MovingBlocks::BLOCK_SIZE,
                    MovingBlocks::BLOCK_SIZE,
                    char_1.clone(),
                );
                x += (4 * MovingBlocks::BLOCK_SIZE) as isize;
            }

            y += 4 * MovingBlocks::BLOCK_SIZE;
        }

        let char_2 = Character {
            character: '#',
            color: Color {
                r: 0.0,
                g: 0.550,
                b: 1.0,
            },
        };

        let mut y = 2 * MovingBlocks::BLOCK_SIZE;
        while y < frame.y {
            let mut x = -(MovingBlocks::BLOCK_SIZE as isize)
                + (4 * MovingBlocks::BLOCK_SIZE as isize)
                - (self.step / MovingBlocks::CLOCK_DIVIDER + MovingBlocks::BLOCK_SIZE)
                    .rem_euclid(4 * MovingBlocks::BLOCK_SIZE) as isize;

            while x <= frame.x as isize {
                fill_block(
                    frame,
                    x,
                    y as isize,
                    MovingBlocks::BLOCK_SIZE,
                    MovingBlocks::BLOCK_SIZE,
                    char_2.clone(),
                );
                x += (4 * MovingBlocks::BLOCK_SIZE) as isize;
            }

            y += 4 * MovingBlocks::BLOCK_SIZE;
        }

        // second set of blocks moving in y direction
        let char_3 = Character {
            character: 'O',
            color: Color {
                r: 0.75,
                g: 0.0,
                b: 1.0,
            },
        };
        let mut x = 2 * MovingBlocks::BLOCK_SIZE;
        while x < frame.x {
            let mut y = -(MovingBlocks::BLOCK_SIZE as isize)
                + (self.step / MovingBlocks::CLOCK_DIVIDER + MovingBlocks::BLOCK_SIZE)
                    .rem_euclid(4 * MovingBlocks::BLOCK_SIZE) as isize;

            while y <= frame.y as isize {
                fill_block(
                    frame,
                    x as isize,
                    y,
                    MovingBlocks::BLOCK_SIZE,
                    MovingBlocks::BLOCK_SIZE,
                    char_3.clone(),
                );
                y += (4 * MovingBlocks::BLOCK_SIZE) as isize;
            }

            x += 4 * MovingBlocks::BLOCK_SIZE;
        }

        // second set of blocks moving in y direction
        let char_4 = Character {
            character: '%',
            color: Color {
                r: 0.2,
                g: 0.77,
                b: 0.12,
            },
        };

        let mut x = 0;
        while x < frame.x {
            let mut y = -(MovingBlocks::BLOCK_SIZE as isize)
                + (4 * MovingBlocks::BLOCK_SIZE as isize)
                - (self.step / MovingBlocks::CLOCK_DIVIDER + MovingBlocks::BLOCK_SIZE)
                    .rem_euclid(4 * MovingBlocks::BLOCK_SIZE) as isize;

            while y <= frame.y as isize {
                fill_block(
                    frame,
                    x as isize,
                    y,
                    MovingBlocks::BLOCK_SIZE,
                    MovingBlocks::BLOCK_SIZE,
                    char_4.clone(),
                );
                y += (4 * MovingBlocks::BLOCK_SIZE) as isize;
            }

            x += 4 * MovingBlocks::BLOCK_SIZE;
        }

        self.step += 1;
    }
}
