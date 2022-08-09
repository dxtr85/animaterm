#[derive(Debug)]
pub enum AnimError {
    FailAddingAnimation(usize),
    FailGettingGlyph(usize),
    FailAddingFrame(usize),
    ResultReceiverNotSet,
    FrameNotFound,
}
