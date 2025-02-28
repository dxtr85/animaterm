use super::Glyph;
use super::Graphic;
#[derive(Debug)]
/// List of successful results after performing an action.
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
    FrameSwapped(Vec<Glyph>),
}
