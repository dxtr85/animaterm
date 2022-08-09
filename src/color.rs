#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ColorName {
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
pub enum Color {
    Basic(ColorName),

    // \x1b[38;5;248;48;5;232m
    EightBit(u8), // 16 + 36r + 6g + b, where r, g, and b are integers ranging between 0 and 5.

    // \x1b[38;5;248;48;5;232m
    Grayscale(u8), // indices 232 through 255 are a grayscale ramp from dark (232) to light (255).

    // \x1b[38;2;(r);(g);(b)m
    // \x1b[48;2;(r);(g);(b)m
    Truecolor(u8, u8, u8),
}

impl Color {
    pub fn new(value: ColorName) -> Color {
        Color::Basic(value)
    }

    pub fn black() -> Color {
        Color::Basic(ColorName::Black)
    }

    pub fn red() -> Color {
        Color::Basic(ColorName::Red)
    }

    pub fn green() -> Color {
        Color::Basic(ColorName::Green)
    }

    pub fn yellow() -> Color {
        Color::Basic(ColorName::Yellow)
    }

    pub fn blue() -> Color {
        Color::Basic(ColorName::Blue)
    }

    pub fn magenta() -> Color {
        Color::Basic(ColorName::Magenta)
    }

    pub fn cyan() -> Color {
        Color::Basic(ColorName::Cyan)
    }

    pub fn white() -> Color {
        Color::Basic(ColorName::White)
    }

    pub fn new_8bit(red: u8, green: u8, blue: u8) -> Color {
        Color::EightBit(16 + (36 * (red % 6)) + (6 * (green % 6)) + blue)
    }
    pub fn new_gray(brightness: u8) -> Color {
        Color::Grayscale(brightness % 24 + 232)
    }
    pub fn new_truecolor(red: u8, green: u8, blue: u8) -> Color {
        Color::Truecolor(red, green, blue)
    }
}
