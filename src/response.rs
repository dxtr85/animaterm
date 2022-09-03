use super::Glyph;
use super::Graphic;
#[derive(Debug)]
pub enum AnimOk {
    AnimationAdded(usize),
    FrameAdded(usize, usize),
    AllResultsRead,
    DisplayCreated(usize),
    DisplayRestored(usize),
    GlyphRetrieved(usize, Glyph),
    GraphicAdded(usize),
    GraphicCreated(Graphic),
    PrintScreen(Vec<String>),
}
