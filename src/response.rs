use super::error::AnimError;

#[derive(Debug)]
pub enum AnimResult {
    Ok(AnimOk),
    Err(AnimError),
}

#[derive(Debug)]
pub enum AnimOk {
    AnimationAdded(usize),
}
