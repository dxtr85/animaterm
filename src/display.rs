use super::glyph::Glyph;
use super::glyphcake::GlyphCake;

pub struct Display {
    pub id: usize,
    pub array: Vec<GlyphCake>,
}

impl Display {
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
