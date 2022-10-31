use super::glyph::Glyph;
use super::glyphcake::GlyphCake;

/// A structure representing a screen display.
pub struct Display {
    pub id: usize,
    pub array: Vec<GlyphCake>,
}

impl Display {
    /// Create a new instance fo Display structure with provided dimentions and fill it with provided glyph.
    pub fn new(id: usize, glyph: Glyph, cols: usize, rows: usize) -> Self {
        let mut array = Vec::with_capacity(cols * rows);
        for j in 1..rows + 1 {
            for i in 1..cols + 1 {
                array.push(GlyphCake::new(i, j, Some(glyph.clone()), 0));
            }
        }
        Display { id, array }
    }
}
