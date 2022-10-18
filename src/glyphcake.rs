use super::Glyph;
use std::mem::replace;
pub struct GlyphCake {
    pub col: usize,
    pub row: usize,
    pub glyphs: Vec<Option<Glyph>>,
    top_layer: usize,
    pub modified: bool,
}

impl GlyphCake {
    pub fn new(col: usize, row: usize, glyph: Option<Glyph>, layer: usize) -> Self {
        let mut glyphs = vec![None; layer];
        let modified = false;
        glyphs.insert(layer, glyph);
        GlyphCake {
            col,
            row,
            glyphs,
            top_layer: layer,
            modified,
        }
    }

    pub fn update(&mut self, glyph: Glyph, layer: usize) {
        let what_to_insert = if glyph.transparent { None } else { Some(glyph) };
        if layer >= self.top_layer {
            for i in self.top_layer + 1..layer + 1 {
                self.glyphs.insert(i, None);
            }
            let _old = replace(&mut self.glyphs[layer], what_to_insert);
            self.modified = true;
            self.top_layer = layer;
            if what_to_insert.is_none() {
                self.decrease_top_layer();
            }
        } else {
            self.modified = true;
            let _old = replace(&mut self.glyphs[layer], what_to_insert);
        }
    }

    pub fn decrease_top_layer(&mut self) {
        let mut current_glyph = self.glyphs.get(self.top_layer).unwrap();
        while current_glyph.is_none() && self.top_layer > 0 {
            self.top_layer -= 1;
            self.glyphs.pop();
            current_glyph = self.glyphs.get(self.top_layer).unwrap();
        }
        if current_glyph.is_none() {
            self.glyphs.insert(0, Some(Glyph::default()));
        }
    }

    pub fn get_glyph(&mut self) -> Glyph {
        self.modified = false;
        let glyph = self.glyphs.get(self.top_layer);
        if glyph.is_none() || glyph.unwrap().is_none() {
            eprintln!("Cake has no glyphs!");
            return Glyph::default();
        };
        glyph.unwrap().unwrap().clone()
    }
}
