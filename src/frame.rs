use rand::prelude::*;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone)]
pub struct Character {
    pub color: Color,
    pub character: char,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub x: usize,
    pub y: usize,
    pub data: Vec<Character>,
}

impl Color {
    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HSVColor {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl From<HSVColor> for Color {
    fn from(hsv: HSVColor) -> Self {
        let h = hsv.h * 360.0;

        let h_i = (h / 60.0) as usize;
        let f = h / 60.0 - h_i as f32;

        let p = hsv.v * (1.0 - hsv.s);
        let q = hsv.v * (1.0 - hsv.s * f);
        let t = hsv.v * (1.0 - hsv.s * (1.0 - f));

        match h_i {
            1 => Color {
                r: q,
                g: hsv.v,
                b: p,
            },
            2 => Color {
                r: p,
                g: hsv.v,
                b: t,
            },
            3 => Color {
                r: p,
                g: q,
                b: hsv.v,
            },
            4 => Color {
                r: t,
                g: p,
                b: hsv.v,
            },
            5 => Color {
                r: hsv.v,
                g: p,
                b: q,
            },
            _ => Color {
                r: hsv.v,
                g: t,
                b: p,
            },
        }
    }
}

impl HSVColor {
    /// generates a HSVColor with maximum saturation and value and a random hue.
    pub fn random_hue() -> HSVColor {
        HSVColor {
            h: rand::thread_rng().gen(),
            s: 1.0,
            v: 1.0,
        }
    }
}

impl Character {
    pub fn empty() -> Character {
        Character {
            color: Color::black(),
            character: ' ',
        }
    }

    pub fn render(&self) -> String {
        format!(
            "\x1b[38;2;{};{};{}m{}",
            (self.color.r * 256.0) as u8,
            (self.color.g * 256.0) as u8,
            (self.color.b * 256.0) as u8,
            self.character
        )
    }

    /// returns a Character with a random printable ASCII Character and a random color.
    pub fn random() -> Character {
        let character_code: u32 = rand::thread_rng().gen_range(32..128);
        let character = char::from_u32(character_code).unwrap();

        Character {
            color: Color::from(HSVColor::random_hue()),
            character,
        }
    }
}

impl<'a> Frame {
    pub fn new(x: usize, y: usize) -> Frame {
        Frame {
            x,
            y,
            data: vec![Character::empty(); x * y],
        }
    }

    pub fn render(&self) {
        // a rougth estimate of the string size
        let mut output = String::with_capacity(self.x * self.y * 20);

        // move cursor to top
        output += "\x1B[1;1H";

        // hide the cursor
        output += "\x1B[?25l";

        // write the pixels
        for y in 0..self.y {
            for x in 0..self.x {
                let outchar = self.data[y * self.x + x].render();
                output += &outchar;
            }

            if y != self.y - 1 {
                output += "\n";
            }
        }

        print!("{}", output);
        io::stdout().flush().unwrap();
    }

    pub fn get(&'a self, x: usize, y: usize) -> &'a Character {
        &self.data[y * self.x + x]
    }

    pub fn get_mut(&'a mut self, x: usize, y: usize) -> &'a mut Character {
        &mut self.data[y * self.x + x]
    }
}
