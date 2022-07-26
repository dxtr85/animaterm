#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

//8 color
// 8-bit color RGB
// 8-bit color Grayscale
// 24-bit color RGB TrueColor

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum NewColor {
    Basic(Color),

    // \x1b[38;5;248;48;5;232m
    EightBit(u8), // 16 + 36r + 6g + b, where r, g, and b are integers ranging between 0 and 5.

    // \x1b[38;5;248;48;5;232m
    Grayscale(u8), // indices 232 through 255 are a grayscale ramp from dark (232) to light (255).

    // \x1b[38;2;(r);(g);(b)m
    // \x1b[48;2;(r);(g);(b)m
    Truecolor(u8, u8, u8),
}

impl NewColor {
    pub fn new(value: Color) -> NewColor {
        NewColor::Basic(value)
    }

    pub fn black() -> NewColor {
        NewColor::Basic(Color::Black)
    }

    pub fn red() -> NewColor {
        NewColor::Basic(Color::Red)
    }

    pub fn green() -> NewColor {
        NewColor::Basic(Color::Green)
    }

    pub fn yellow() -> NewColor {
        NewColor::Basic(Color::Yellow)
    }

    pub fn blue() -> NewColor {
        NewColor::Basic(Color::Blue)
    }

    pub fn magenta() -> NewColor {
        NewColor::Basic(Color::Magenta)
    }

    pub fn cyan() -> NewColor {
        NewColor::Basic(Color::Cyan)
    }

    pub fn white() -> NewColor {
        NewColor::Basic(Color::White)
    }

    pub fn new8Bit(red: u8, green: u8, blue: u8) -> NewColor {
        NewColor::EightBit(16 + (36 * (red % 6)) + (6 * (green % 6)) + blue)
    }
    pub fn newGray(brightness: u8) -> NewColor {
        NewColor::Grayscale(brightness % 24 + 232)
    }
    pub fn newTruecolor(red: u8, green: u8, blue: u8) -> NewColor {
        NewColor::Truecolor(red, green, blue)
    }
}
