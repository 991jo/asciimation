use crate::frame::{Character, Color, Frame, HSVColor};
use rand::Rng;

pub trait Animation {
    const NAME: &'static str;
    const AUTHOR: &'static str;
    /// returns the name of the animation
    fn name(&self) -> &'static str {
        Self::NAME
    }

    /// returns the author of the animation
    fn author(&self) -> &'static str {
        Self::AUTHOR
    }

    /// writes the next animation step into the given frame.
    fn render(&mut self, frame: &mut Frame);
}

pub struct TextOverlay {
    pub text: String,
}

impl Animation for TextOverlay {
    const NAME: &'static str = "TextOverlay";
    const AUTHOR: &'static str = "Jo";

    fn render(&mut self, frame: &mut Frame) {
        let mut line = 0;
        let mut char_in_line = 0;
        for character in self.text.chars() {
            // handling of new lines
            if character == '\n' {
                line += 1;
                char_in_line = 0;
                continue;
            }

            // handling of text wrapping
            if char_in_line >= frame.x {
                line += 1;
                char_in_line = 0;
            }

            // return if the screen is full
            if line >= frame.y {
                return;
            }

            // actually place the character
            frame.data[line * frame.x + char_in_line] = Character {
                character,
                color: Color::white(),
            };

            char_in_line += 1;
        }
    }
}

pub struct Rainbow {
    color_shift: f32,
    rotation: f32,
}

impl Default for Rainbow {
    fn default() -> Rainbow {
        Rainbow {
            color_shift: 0.0,
            rotation: 0.0,
        }
    }
}

impl Animation for Rainbow {
    const NAME: &'static str = "Rainbow";
    const AUTHOR: &'static str = "Jo";

    fn render(&mut self, frame: &mut Frame) {
        self.color_shift += 0.01;
        self.rotation += 0.01;
        for y in 0..frame.y {
            for x in 0..frame.x {
                // translate coordinates to [0, 1]
                //
                // often characters in the terminal as twice as high as wide
                // however this is only an approximation.
                let long_edge = frame.x.max(frame.y * 2);

                let x_t = x as f32 / long_edge as f32;
                let y_t = 2.0 * y as f32 / long_edge as f32;

                let a_1 = self.rotation.cos();
                let a_2 = self.rotation.sin();

                let b_1 = -self.rotation.sin();
                let b_2 = self.rotation.cos();

                let l_1 = (x_t - (b_1 / b_2 * y_t)) / (a_1 - (b_1 / b_2 * a_2));

                // let hue = x as f32 / frame.x as f32;
                let hue = l_1;
                let hue = (hue + self.color_shift).rem_euclid(1.0);

                let hsv_color = HSVColor {
                    h: hue,
                    s: 1.0,
                    v: 1.0,
                };
                let color = Color::from(hsv_color);

                frame.data[y * frame.x + x] = Character {
                    character: 'A',
                    color,
                };
            }
        }
    }
}

#[derive(Debug, Clone)]
struct RandomWalker {
    x: usize,
    y: usize,
    character: Character,
}

impl RandomWalker {
    fn random() -> RandomWalker {
        let mut rng = rand::thread_rng();

        RandomWalker {
            x: rng.gen_range(0..1024),
            y: rng.gen_range(0..1024),
            character: Character::random(),
        }
    }

    fn walk(&mut self, frame: &Frame) {
        let direction = rand::thread_rng().gen_range(0..4);

        // dbg!(direction, self.x, self.y);

        match direction {
            0 => self.x = self.x.wrapping_add(1),
            1 => self.x = self.x.wrapping_sub(1),
            2 => self.y = self.y.wrapping_add(1),
            _ => self.y = self.y.wrapping_sub(1),
        };

        self.x = self.x.rem_euclid(frame.x);
        self.y = self.y.rem_euclid(frame.y);
    }
}

pub struct RandomWalkers {
    walkers: Vec<RandomWalker>,
}

impl Default for RandomWalkers {
    fn default() -> RandomWalkers {
        let mut walkers = RandomWalkers {
            walkers: Vec::new(),
        };

        for _ in 0..10 {
            walkers.walkers.push(RandomWalker::random())
        }

        walkers
    }
}

impl Animation for RandomWalkers {
    const NAME: &'static str = "RandomWalkers";
    const AUTHOR: &'static str = "Jo";

    /// writes the next animation step into the given frame.
    fn render(&mut self, frame: &mut Frame) {
        for walker in self.walkers.iter_mut() {
            walker.walk(frame);

            //dbg!(walker.clone());

            *frame.get_mut(walker.x, walker.y) = walker.character.clone();
        }
    }
}
