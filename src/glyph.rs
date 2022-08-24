use super::color::{Color, ColorName};
enum ExpectedToken {
    Any,
    ColorByte,
    ColorSpecifier,
}

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

    pub fn black() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::black());
        g
    }

    pub fn red() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::red());
        g
    }

    pub fn green() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::green());
        g
    }

    pub fn yellow() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::yellow());
        g
    }

    pub fn blue() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::blue());
        g
    }

    pub fn magenta() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::magenta());
        g
    }

    pub fn cyan() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::cyan());
        g
    }

    pub fn white() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::white());
        g
    }

    pub fn orange() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::orange());
        g
    }

    pub fn indigo() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::indigo());
        g
    }

    pub fn violet() -> Self {
        let mut g = Glyph::default();
        g.set_background(Color::violet());
        g
    }

    pub fn default_with_char(character: char) -> Self {
        let mut g = Glyph::default();
        g.set_char(character);
        g
    }

    pub fn update_from_str(&mut self, style_definition: &str) {
        if style_definition.len() == 0 {
            panic!("can not update empty style");
        }
        //println!("Got str: {}", style_definition);
        let mut tokens_started = false;
        let mut tokens = Vec::with_capacity(16);
        let mut current_token = String::with_capacity(3);
        for char in style_definition.chars() {
            match char {
                '\x1b' => tokens_started = true,
                '[' => {
                    if tokens_started {
                        continue;
                    } else {
                        self.set_char(char);
                    }
                }
                'm' => {
                    if tokens_started {
                        tokens_started = false;
                        if current_token.len() > 0 {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    } else {
                        self.set_char(char);
                    }
                }
                ';' => {
                    if current_token.len() > 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
                _ => {
                    if tokens_started {
                        current_token.push(char);
                    } else {
                        self.set_char(char);
                    }
                }
            }
        }
        if tokens.len() == 0 {
            panic!("empty token list!")
        }
        //println!("Got tokens: {:?}", tokens);
        let mut color_8bit: u8;
        let mut color_red: u8 = 0;
        let mut color_green: u8 = 0;
        let mut color_blue: u8;

        //let mut next_bg_sophisticated = false;
        let mut next_token = ExpectedToken::Any;
        let mut defining_background = false;
        let mut defining_truecolor = false;
        let mut color_bytes_left_to_read: u8 = 0;
        for token in tokens.iter() {
            match next_token {
                ExpectedToken::Any => match &token[..] {
                    "0" => {
                        self.set_transparent(true);
                    }
                    "1" => {
                        self.set_bright(true);
                    }
                    "2" => {
                        //println!("Processing 2, no transparent");
                        self.set_transparent(false);
                    }
                    "23" => {
                        //println!("Processing 23, no italic");
                        self.set_italic(false);
                    }
                    "3" => {
                        self.set_italic(true);
                    }
                    "24" => {
                        //println!("Processing 24, no underline");
                        self.set_underline(false);
                    }
                    "4" => {
                        self.set_underline(true);
                    }
                    "25" => {
                        //println!("Processing 25, no blink");
                        self.set_blink(false);
                    }
                    "5" => {
                        //println!("Processing 5, blink true");
                        self.set_blink(true);
                    }
                    "26" => {
                        //println!("Processing 26, no blinkfast");
                        self.set_blinkfast(false);
                    }
                    "6" => {
                        self.set_blinkfast(true);
                    }
                    "27" => {
                        //println!("Processing 27, no reverse");
                        self.set_reverse(false);
                    }
                    "7" => {
                        self.set_reverse(true);
                    }
                    "29" => {
                        //println!("Processing 29, no strike");
                        self.set_strike(false);
                    }
                    "9" => {
                        self.set_strike(true);
                    }
                    "30" => {
                        //println!("Processing 30, color black");
                        self.set_color(Color::black());
                    }
                    "31" => {
                        self.set_color(Color::red());
                    }
                    "32" => {
                        self.set_color(Color::green());
                    }
                    "33" => {
                        self.set_color(Color::yellow());
                    }
                    "34" => {
                        self.set_color(Color::blue());
                    }
                    "35" => {
                        self.set_color(Color::magenta());
                    }
                    "36" => {
                        self.set_color(Color::cyan());
                    }
                    "37" => {
                        self.set_color(Color::white());
                    }
                    "38" => {
                        next_token = ExpectedToken::ColorSpecifier;
                    }
                    "40" => {
                        self.set_background(Color::black());
                    }
                    "41" => {
                        self.set_background(Color::red());
                    }
                    "42" => {
                        self.set_background(Color::green());
                    }
                    "43" => {
                        self.set_background(Color::yellow());
                    }
                    "44" => {
                        self.set_background(Color::blue());
                    }
                    "45" => {
                        self.set_background(Color::magenta());
                    }
                    "46" => {
                        self.set_background(Color::cyan());
                    }
                    "47" => {
                        self.set_background(Color::white());
                    }
                    "48" => {
                        //println!("Processing 48, stage for bg color set");
                        next_token = ExpectedToken::ColorSpecifier;
                        defining_background = true;
                    }
                    "90" => {
                        self.set_color(Color::black());
                        self.set_bright(true);
                    }
                    "91" => {
                        self.set_color(Color::red());
                        self.set_bright(true);
                    }
                    "92" => {
                        self.set_color(Color::green());
                        self.set_bright(true);
                    }
                    "93" => {
                        self.set_color(Color::yellow());
                        self.set_bright(true);
                    }
                    "94" => {
                        self.set_color(Color::blue());
                        self.set_bright(true);
                    }
                    "95" => {
                        self.set_color(Color::magenta());
                        self.set_bright(true);
                    }
                    "96" => {
                        self.set_color(Color::cyan());
                        self.set_bright(true);
                    }
                    "97" => {
                        self.set_color(Color::white());
                        self.set_bright(true);
                    }
                    "100" => {
                        self.set_background(Color::black());
                        self.set_bright(true);
                    }
                    "101" => {
                        self.set_background(Color::red());
                        self.set_bright(true);
                    }
                    "102" => {
                        self.set_background(Color::green());
                        self.set_bright(true);
                    }
                    "103" => {
                        self.set_background(Color::yellow());
                        self.set_bright(true);
                    }
                    "104" => {
                        self.set_background(Color::blue());
                        self.set_bright(true);
                    }
                    "105" => {
                        self.set_background(Color::magenta());
                        self.set_bright(true);
                    }
                    "106" => {
                        self.set_background(Color::cyan());
                        self.set_bright(true);
                    }
                    "107" => {
                        self.set_background(Color::white());
                        self.set_bright(true);
                    }
                    _ => {
                        continue;
                    }
                },
                ExpectedToken::ColorSpecifier => match &token[..] {
                    "2" => {
                        //println!("Processing 2, three bytes for color expected ");
                        next_token = ExpectedToken::ColorByte;
                        color_bytes_left_to_read = 3;
                        defining_truecolor = true;
                    }
                    "5" => {
                        //println!("Processing 5, single byte color next");
                        next_token = ExpectedToken::ColorByte;
                        color_bytes_left_to_read = 1;
                    }
                    _ => panic!("Was expecting 2 or 5, got {}", token),
                },
                ExpectedToken::ColorByte => {
                    // println!(
                    //     "reading byte color {}, left to read: {}",
                    //     token, color_bytes_left_to_read
                    // );
                    if defining_truecolor {
                        //println!("defining truecolor");
                        match color_bytes_left_to_read {
                            3 => {
                                color_red = u8::from_str_radix(token, 10).unwrap_or_default();
                            }
                            2 => {
                                color_green = u8::from_str_radix(token, 10).unwrap_or_default();
                            }
                            1 => {
                                color_blue = u8::from_str_radix(token, 10).unwrap_or_default();
                                //println!("setting truecolor");
                                next_token = ExpectedToken::Any;
                                if defining_background {
                                    self.set_background(Color::new_truecolor(
                                        color_red,
                                        color_green,
                                        color_blue,
                                    ));
                                } else {
                                    self.set_color(Color::new_truecolor(
                                        color_red,
                                        color_green,
                                        color_blue,
                                    ));
                                }
                            }
                            _ => continue,
                        }
                    } else {
                        // println!(
                        //     "defining 8-bit color, bytes left: {}",
                        //     color_bytes_left_to_read
                        // );
                        match color_bytes_left_to_read {
                            1 => {
                                color_8bit = u8::from_str_radix(token, 10).unwrap_or_default();
                                //println!("setting 8-bit color");
                                next_token = ExpectedToken::Any;
                                if defining_background {
                                    if color_8bit > 231 {
                                        self.set_background(Color::new_gray(color_8bit - 232));
                                    } else {
                                        self.set_background(Color::EightBit(color_8bit));
                                    }
                                } else {
                                    if color_8bit > 231 {
                                        self.set_color(Color::new_gray(color_8bit - 232));
                                    } else {
                                        self.set_color(Color::EightBit(color_8bit));
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                    if color_bytes_left_to_read > 0 {
                        color_bytes_left_to_read -= 1;
                    }
                }
            }
        }
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
    pub fn set_transparent(&mut self, transparent: bool) {
        self.plain = transparent;
    }
    pub fn set_bright(&mut self, bright: bool) {
        self.bright = bright;
    }
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }
    pub fn set_underline(&mut self, underline: bool) {
        self.underline = underline;
    }
    pub fn set_blink(&mut self, blink: bool) {
        self.blink = blink;
    }
    pub fn set_blinkfast(&mut self, blink: bool) {
        self.blink_fast = blink;
    }
    pub fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }
    pub fn set_strike(&mut self, strike: bool) {
        self.strike = strike;
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
