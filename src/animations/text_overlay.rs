use super::prelude::*;

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
