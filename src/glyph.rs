use super::color::{Color, ColorName};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Glyph {
    pub character: char,
    pub color: Color,
    pub background: Color,
    pub plain: bool,
    pub bright: bool,
    // pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub blink_fast: bool,
    pub reverse: bool,
    pub strike: bool,
}

impl Glyph {
    pub fn new(
        character: char,
        color: Color,
        background: Color,
        plain: bool,
        bright: bool,
        // dim: bool,
        italic: bool,
        underline: bool,
        blink: bool,
        blink_fast: bool,
        reverse: bool,
        strike: bool,
    ) -> Self {
        Glyph {
            character,
            color,
            background,
            plain,
            bright,
            // dim,
            italic,
            underline,
            blink,
            blink_fast,
            reverse,
            strike,
        }
    }
    pub fn plain() -> Self {
        Glyph {
            character: ' ',
            color: Color::Basic(ColorName::White),
            background: Color::Basic(ColorName::Black),
            plain: true,
            bright: false,
            // dim: false,
            italic: false,
            underline: false,
            blink: false,
            blink_fast: false,
            reverse: false,
            strike: false,
        }
    }

    pub fn transparent() -> Self {
        Glyph {
            character: ' ',
            color: Color::Basic(ColorName::White),
            background: Color::Basic(ColorName::Black),
            plain: true,
            bright: false,
            // dim: false,
            italic: false,
            underline: false,
            blink: false,
            blink_fast: false,
            reverse: false,
            strike: false,
        }
    }
    pub fn default_with_char(character: char) -> Self {
        let mut g = Glyph::default();
        g.set_char(character);
        g
    }

    pub fn set_char(&mut self, character: char) {
        self.character = character;
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }
    pub fn set_blink(&mut self, blink: bool) {
        self.blink = blink;
    }
    pub fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }
}
impl Default for Glyph {
    fn default() -> Self {
        Glyph {
            character: ' ',
            color: Color::Basic(ColorName::White),
            background: Color::Basic(ColorName::Black),
            plain: false,
            bright: false,
            // dim: false,
            italic: false,
            underline: false,
            blink: false,
            blink_fast: false,
            reverse: false,
            strike: false,
        }
    }
}
