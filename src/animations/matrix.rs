use super::prelude::*;
use crate::frame::Character;
use rand::Rng;

#[derive(Default, Clone)]
pub struct Matrix {
    columns: Vec<Column>,
    initialized: bool,
    special_char_mode: SpecialCharMode,
    text: Vec<char>,
}

#[derive(Default, Clone)]
pub struct Column {
    chars: Vec<usize>,
    decay_factor: f32,
    position: f32,
    speed: f32,
    done: bool,
    special_chars: Vec<(usize, Character)>,
    special_char_mode: SpecialCharMode,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
enum SpecialCharMode {
    #[default]
    Appear,
    Disappear,
}

impl Animation for Matrix {
    fn name(&self) -> &'static str {
        "The Matrix"
    }

    fn author(&self) -> &'static str {
        "Jo"
    }

    fn render(&mut self, frame: &mut Frame) {
        if !self.initialized {
            self.initialize(frame);
            self.initialized = true;
        }

        let mut all_done = true;

        for (x, column) in self.columns.iter_mut().enumerate() {
            column.render(frame, x);

            all_done = all_done && column.done;
        }

        if all_done {
            self.columns.clear();
            if self.special_char_mode == SpecialCharMode::Appear {
                self.special_char_mode = SpecialCharMode::Disappear;
            } else {
                self.special_char_mode = SpecialCharMode::Appear;
            }
            self.initialize(frame);
        }
    }
}

impl Matrix {
    pub const TEXTE: &'static [&'static str] = &[
        "Fanatischer Aalen Termin",
        "Fragwürdiger Aalen Termin",
        "Fantasievoller Aalen Termin",
        "Fabelhafter Aalen Termin",
        "Fantasievoller Aalen Termin",
        "Fehlerharter Aalen Termin",
        "Fehlerfreier Aalen Termin",
        "Feinmotorischer Aalen Termin",
        "Fiktionaler Aalen Termin",
        "Floraler Aalen Termin",
        "Frecher Aalen Termin",
    ];

    pub fn get_text() -> Vec<char> {
        let mut rng = rand::thread_rng();

        let funny = rng.gen();

        if funny {
            let text = Matrix::TEXTE[rng.gen_range(0..Matrix::TEXTE.len())];
            return text.chars().collect();
        } else {
            return "Fantastischer Aalen Termin".chars().collect();
        }
    }
    pub fn initialize(&mut self, frame: &Frame) {
        self.columns = vec![];

        for _ in 0..frame.x {
            self.columns.push(Column::random(frame.y));
        }

        for column in self.columns.iter_mut() {
            column.special_char_mode = self.special_char_mode;
        }

        if self.special_char_mode == SpecialCharMode::Appear {
            self.text = Matrix::get_text();
        }

        let offset = (frame.x - self.text.len()) / 2;

        let y = frame.y / 2;

        for (index, character) in self.text.iter().enumerate() {
            let x = offset + index;

            if x > frame.x {
                break;
            }

            self.columns[x].special_chars.push((
                y,
                Character {
                    color: Color::WHITE.clone(),
                    character: *character,
                },
            ));
        }
    }
}

impl Column {
    pub const BASE_COLOR: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    pub fn random(height: usize) -> Self {
        let mut rng = rand::thread_rng();

        let decay_factor: f32 = rng.gen::<f32>() / (height as f32 * 0.75) + 0.02; // decay between 0.05 and 0.1 => lines
                                                                                  // are 10 to 20 chars long
        let mut column = Column {
            chars: vec![],
            decay_factor,
            speed: rng.gen::<f32>() / 1.5 + 0.5,
            position: rng.gen_range(-(height as f32)..0.0),
            done: false,
            special_chars: vec![],
            special_char_mode: SpecialCharMode::Appear,
        };

        for _ in 0..height {
            column.chars.push(rng.gen_range(0..94));
        }

        column
    }
    pub fn render(&mut self, frame: &mut Frame, x: usize) {
        if !self.done {
            self.render_falling_chars(frame, x);
            self.step(frame, x)
        }

        // render the special chars
        //
        match self.special_char_mode {
            SpecialCharMode::Appear => {
                if self.position >= 0.0 {
                    let position = self.position as usize;
                    for (char_pos, char) in self.special_chars.iter() {
                        if position >= *char_pos {
                            frame.set_at(x, *char_pos, char.clone());
                        }
                    }
                }
            }
            SpecialCharMode::Disappear => {
                let position = self.position as isize;
                for (char_pos, char) in self.special_chars.iter() {
                    if position < *char_pos as isize {
                        frame.set_at(x, *char_pos, char.clone());
                    }
                }
            }
        }
    }

    pub fn render_falling_chars(&mut self, frame: &mut Frame, x: usize) {
        // early abort if we are not in the frame yet
        if self.position < 0.0 {
            return;
        };

        let mut color = Column::BASE_COLOR.clone();

        let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect();

        self.done = true;

        for (index, char) in self.chars.iter().enumerate() {
            // dimm the color, but not for the first character
            if index != 0 {
                color.subtract(self.decay_factor);
            }

            // once the chars are to dimm, don't draw them anymore
            if color.luminance() < 0.1 {
                continue;
            }

            let target_pos = self.position - (index as f32);
            let target_pos = target_pos as isize;

            // out of bounds check
            if target_pos < 0 || target_pos >= (frame.y as isize) {
                continue;
            }

            let target_pos = target_pos as usize;

            frame.set_at(
                x,
                target_pos,
                Character {
                    color: color.clone(),
                    character: characters[*char],
                },
            );
            self.done = false;
        }
    }

    pub fn step(&mut self, _frame: &mut Frame, _x: usize) {
        self.position += self.speed;
    }
}
