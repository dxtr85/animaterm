use super::Glyph;

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub print: bool,
    pub g: Glyph,
}

impl Pixel {
    pub fn new(x: usize, y: usize, print: bool, g: Glyph) -> Pixel {
        Pixel { x, y, print, g }
    }
    pub fn set_xy(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
    pub fn set_print(&mut self, print: bool) {
        self.print = print;
    }
    pub fn offset(&mut self, offset: (usize, usize)) {
        self.x += offset.0;
        self.y += offset.1;
    }
}
