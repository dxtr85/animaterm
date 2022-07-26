//! An easy to use library for terminal user interface (TUI)
mod error;
pub use error::AnimError;
mod manager;
pub use manager::Manager;
mod response;
pub use response::{AnimOk, AnimResult};
mod screen;
mod time;
pub use time::Timestamp;
mod graphic;
pub use graphic::Graphic;
mod animation;
pub use animation::Animation;
mod color;
pub use color::{Color, NewColor};
mod glyph;
mod pixel;
pub use glyph::Glyph;
mod glyphcake;
mod helpers;
mod key;
mod tests;
pub mod utilities;

pub mod prelude {
    pub use crate::animation::Animation;
    pub use crate::color::Color;
    pub use crate::error::AnimError;
    pub use crate::glyph::Glyph;
    pub use crate::graphic::Graphic;
    pub use crate::key::Key;
    pub use crate::manager::Manager;
    pub use crate::response::{AnimOk, AnimResult};
    pub use crate::time::Timestamp;
}
