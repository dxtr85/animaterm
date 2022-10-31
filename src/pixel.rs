use super::Glyph;

#[derive(Clone, Copy, Debug)]
/// Represents a glyph and it's on screen location.
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub g: Glyph,
}

impl Pixel {
    /// Create a new pixel instance.
    pub fn new(x: usize, y: usize, g: Glyph) -> Pixel {
        Pixel { x, y, g }
    }

    /// Change location of a pixel.
    pub fn set_xy(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    /// Move a pixel from it's current location.
    pub fn offset(&mut self, offset: (usize, usize)) {
        self.x += offset.0;
        self.y += offset.1;
    }
}
