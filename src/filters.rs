use crate::frame::{Character, Color, Frame};
use rand::seq::SliceRandom;

/// Fades the given frame to a blank screen.
///
/// a fade value of 1.0 does not alter the frame, a value of 0.0 is completly blank.
///
/// The screen is faded out by scaling down the colors with the given values and setting them to
/// whitespace if they are below a certain luminance threshold.
pub fn fadeout(frame: &mut Frame, fade: f32) {
    for char in frame.data.iter_mut() {
        char.color.scale(fade);

        if char.color.luminance() < 0.1 {
            char.character = ' ';
        }
    }
}

pub struct DVDLogo {
    x: isize,
    y: isize,
    x_speed: isize,
    y_speed: isize,
}

impl DVDLogo {
    const LOGO: &'static str = "     @@@@@@@@@@@@@@@@@@@@       @@@@@@@@@@@@@@@@@@@
               @@@@@@@@@@@     @@@@@          @@@@@@
    @@@@@       @@@@@ @@@@@  @@@@@  @@@@@       @@@@@
    @@@@@      @@@@@@ @@@@@ @@@@@   @@@@@      @@@@@
   @@@@@     @@@@@@    @@@@@@@@    @@@@@    @@@@@@@
   @@@@@@@@@@@@@        @@@@@      @@@@@@@@@@@@@
                         @@@
                         @
      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
 @@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@
     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
                                               @";

    const WIDTH: isize = 54;
    const HEIGHT: isize = 12;

    pub fn step(&mut self, frame: &Frame) {
        // x movement
        if self.x_speed > 0 {
            if (self.x + DVDLogo::WIDTH) as isize + self.x_speed >= frame.x as isize {
                self.x_speed = -self.x_speed;
            }
        } else {
            if self.x as isize + self.x_speed < 0 {
                self.x_speed = -self.x_speed;
            }
        }

        // y movement
        if self.y_speed > 0 {
            if (self.y + DVDLogo::HEIGHT) as isize + self.y_speed >= frame.y as isize {
                self.y_speed = -self.y_speed;
            }
        } else {
            if self.y as isize + self.y_speed < 0 {
                self.y_speed = -self.y_speed;
            }
        }

        self.x += self.x_speed;
        self.y += self.y_speed;
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        for (y_index, line) in DVDLogo::LOGO.split("\n").enumerate() {
            for (x_index, character) in line.chars().enumerate() {
                if character != ' ' {
                    frame.set_at_clipping(
                        self.x + x_index as isize,
                        self.y + y_index as isize,
                        Character {
                            color: Color::WHITE,
                            character,
                        },
                    );
                }
            }
        }
    }
}

impl Default for DVDLogo {
    fn default() -> Self {
        DVDLogo {
            x: 0,
            y: 0,
            x_speed: 2,
            y_speed: 1,
        }
    }
}
