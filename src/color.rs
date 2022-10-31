#[derive(PartialEq, Clone, Copy, Debug)]
/// A helper enum used for better code readability.
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
/// Definition of all available color types.
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
    /// Create a new color by name.
    pub fn new(value: ColorName) -> Color {
        Color::Basic(value)
    }

    /// Get a black color.
    pub fn black() -> Color {
        Color::Basic(ColorName::Black)
    }

    /// Get a red color.
    pub fn red() -> Color {
        Color::Basic(ColorName::Red)
    }

    /// Get a orange color.
    pub fn orange() -> Color {
        Color::new_8bit(5, 2, 0)
    }

    /// Get a indigo color.
    pub fn indigo() -> Color {
        Color::new_8bit(2, 1, 5)
    }

    /// Get a violet color.
    pub fn violet() -> Color {
        Color::new_8bit(4, 0, 4)
    }

    /// Get a green color.
    pub fn green() -> Color {
        Color::Basic(ColorName::Green)
    }

    /// Get a yellow color.
    pub fn yellow() -> Color {
        Color::Basic(ColorName::Yellow)
    }

    /// Get a blue color.
    pub fn blue() -> Color {
        Color::Basic(ColorName::Blue)
    }

    /// Get a magenta color.
    pub fn magenta() -> Color {
        Color::Basic(ColorName::Magenta)
    }

    /// Get a cyan color.
    pub fn cyan() -> Color {
        Color::Basic(ColorName::Cyan)
    }

    /// Get a white color.
    pub fn white() -> Color {
        Color::Basic(ColorName::White)
    }

    /// Get a new 8-bit color defined by three values ranging from 0 to 5.
    pub fn new_8bit(red: u8, green: u8, blue: u8) -> Color {
        Color::EightBit(16 + (36 * (red % 6)) + (6 * (green % 6)) + blue)
    }

    /// Get a new Grayscale color defined by value from 0 to 23.
    pub fn new_gray(brightness: u8) -> Color {
        Color::Grayscale(brightness % 24 + 232)
    }

    /// Get a new Truecolor defined by three values ranging from 0 to 255 each.
    pub fn new_truecolor(red: u8, green: u8, blue: u8) -> Color {
        Color::Truecolor(red, green, blue)
    }
}
