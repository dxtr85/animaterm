#[derive(Debug)]
/// Defines errors returned when library is not used as expected.
pub enum AnimError {
    FailAddingAnimation(usize),
    FailGettingGlyph(usize),
    FailAddingFrame(usize),
    ResultReceiverNotSet,
    FrameNotFound,
    UnableToOpenFile,
    UnableToReadFile,
    UnableToBuildGraphicFromFile,
}
