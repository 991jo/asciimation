use crate::frame::{value_to_char, Character, Color, Frame, HSVColor};
use nalgebra::Vector2;
use rand::Rng;

pub trait Animation {
    /// returns the name of the animation
    fn name(&self) -> &'static str;

    /// returns the author of the animation
    fn author(&self) -> &'static str;

    /// writes the next animation step into the given frame.
    fn render(&mut self, frame: &mut Frame);
}

pub struct TextOverlay {
    pub text: String,
}

impl TextOverlay {
    const NAME: &'static str = "TextOverlay";
    const AUTHOR: &'static str = "Jo";
}

impl Animation for TextOverlay {
    fn name(&self) -> &'static str {
        TextOverlay::NAME
    }

    fn author(&self) -> &'static str {
        TextOverlay::AUTHOR
    }

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
impl Rainbow {
    const NAME: &'static str = "Rainbow";
    const AUTHOR: &'static str = "Jo";
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
    fn name(&self) -> &'static str {
        Rainbow::NAME
    }

    fn author(&self) -> &'static str {
        Rainbow::AUTHOR
    }

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
impl RandomWalkers {
    const NAME: &'static str = "RandomWalkers";
    const AUTHOR: &'static str = "Jo";
}

impl Animation for RandomWalkers {
    fn name(&self) -> &'static str {
        RandomWalkers::NAME
    }

    fn author(&self) -> &'static str {
        RandomWalkers::AUTHOR
    }

    /// writes the next animation step into the given frame.
    fn render(&mut self, frame: &mut Frame) {
        for walker in self.walkers.iter_mut() {
            walker.walk(frame);

            //dbg!(walker.clone());

            *frame.get_mut(walker.x, walker.y) = walker.character.clone();
        }
    }
}

pub struct GOL {
    data: Vec<bool>,
    step_counter: usize,
    speed: usize,
    x: usize,
    y: usize,
    initialized: bool,
}

impl GOL {
    const NAME: &'static str = "Game of Life";
    const AUTHOR: &'static str = "Jo";

    pub fn step(&mut self) {
        // only make a game step every self.speed steps
        if self.step_counter.rem_euclid(self.speed) != 0 {
            self.step_counter += 1;
            return;
        }
        self.step_counter += 1;

        let mut new_data = vec![false; self.x * self.y];

        for y in 0..self.y {
            for x in 0..self.x {
                let mut neighbors = 0;
                for x_offset in -1..=1 {
                    for y_offset in -1..=1 {
                        // skip the cell itself
                        if x_offset == 0 && y_offset == 0 {
                            continue;
                        }

                        let x_index = (x as i32 + x_offset).rem_euclid(self.x as i32) as usize;
                        let y_index = (y as i32 + y_offset).rem_euclid(self.y as i32) as usize;

                        if self.data[y_index * self.x + x_index] {
                            neighbors += 1;
                        }
                    }
                }
                if neighbors == 2 && self.data[y * self.x + x] {
                    new_data[y * self.x + x] = true;
                } else if neighbors == 3 {
                    new_data[y * self.x + x] = true;
                }
            }
        }

        self.data = new_data;
    }

    pub fn initialize(&mut self) {
        let mut rng = rand::thread_rng();

        for cell in self.data.iter_mut() {
            if rng.gen::<f32>() < 0.25 {
                *cell = true;
            }
        }
    }
}

impl Default for GOL {
    fn default() -> GOL {
        GOL {
            data: vec![],
            step_counter: 0,
            speed: 8,
            x: 0,
            y: 0,
            initialized: false,
        }
    }
}

impl Animation for GOL {
    fn name(&self) -> &'static str {
        GOL::NAME
    }

    fn author(&self) -> &'static str {
        GOL::AUTHOR
    }

    fn render(&mut self, frame: &mut Frame) {
        // fill the data if it is not big enough
        while self.data.len() < frame.data.len() {
            self.data.push(false);
        }
        self.x = frame.x;
        self.y = frame.y;

        // initialize, if not done yet
        if !self.initialized {
            self.initialize();
            self.initialized = true;
        }

        self.step();

        // draw
        for (index, cell) in self.data.iter().enumerate() {
            if *cell {
                frame.data[index] = Character {
                    character: '@',
                    color: Color::white(),
                };
            }
        }
    }
}

pub struct Hill {
    pos: Vector2<f32>,
    direction: Vector2<f32>,
    size: f32,
    height: f32,
}

impl Hill {
    fn random(x: f32, y: f32) -> Hill {
        let mut rng = rand::thread_rng();

        let pos = Vector2::new(rng.gen_range(0.0..(x + 1.0)), rng.gen_range(0.0..(y + 1.0)));
        let direction = Vector2::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5) * 0.2;

        let size = rng.gen::<f32>() * (x.min(y));
        let height = rng.gen::<f32>() * 0.5;

        Hill {
            pos,
            direction,
            size,
            height,
        }
    }

    fn eval(&self, point: Vector2<f32>) -> f32 {
        let distance = self.pos - point;
        let distance = distance / self.size;

        (-(distance.x.powi(2) + distance.y.powi(2))).exp() * self.height
    }
}

pub struct Hills {
    hills: Vec<Hill>,
    initialized: bool,
    x: f32,
    y: f32,
}

impl Default for Hills {
    fn default() -> Self {
        Hills {
            hills: Vec::new(),
            initialized: false,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Hills {
    const NAME: &'static str = "Hills";
    const AUTHOR: &'static str = "Jo";

    fn initialize(&mut self, frame: &Frame) {
        if self.initialized {
            return;
        };

        self.x = frame.x as f32;
        self.y = frame.y as f32;

        for _ in 0..10 {
            self.hills.push(Hill::random(self.x, self.y));
        }
        self.initialized = true;
    }

    fn step(&mut self, frame: &Frame) {
        for hill in self.hills.iter_mut() {
            hill.pos = hill.pos + hill.direction;
            hill.pos.x = hill.pos.x.rem_euclid(frame.x as f32 + 1.0);
            hill.pos.y = hill.pos.y.rem_euclid(frame.y as f32 + 1.0);
        }
    }

    fn eval(&self, x: usize, y: usize) -> f32 {
        let mut value = 0.0;

        for hill in self.hills.iter() {
            let mut best_value: f32 = 0.0;

            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    best_value = best_value.max(hill.eval(Vector2::new(
                        x as f32 + (x_offset as f32 * self.x),
                        y as f32 + (y_offset as f32 * self.y),
                    )));
                }
            }

            value += best_value;
        }

        return value;
    }
}

impl Animation for Hills {
    fn name(&self) -> &'static str {
        Hills::NAME
    }
    fn author(&self) -> &'static str {
        Hills::AUTHOR
    }

    fn render(&mut self, frame: &mut Frame) {
        self.initialize(&frame);
        self.step(&frame);

        for x in 0..frame.x {
            for y in 0..frame.y {
                let cell = frame.get_mut(x, y);

                let value = self.eval(x, y);

                let char = value_to_char(value);

                cell.character = char;
                cell.color = Color::from(HSVColor {
                    h: value.rem_euclid(1.0),
                    s: 1.0,
                    v: 1.0,
                });
            }
        }
    }
}
