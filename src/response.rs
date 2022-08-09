use super::glyph::Glyph;
#[derive(Debug)]
pub enum AnimOk {
    AnimationAdded(usize),
    FrameAdded(usize, usize),
    AllResultsRead,
    DisplayCreated(usize),
    DisplayRestored(usize),
    GlyphRetrieved(usize, Glyph),
}
