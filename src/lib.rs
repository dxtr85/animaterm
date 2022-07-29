//! # An easy to use library for terminal user interface (TUI) creation
//! Inspired by [this post](http://xn--rpa.cc/irl/term.html) I decided that as my first more serious
//! project written in Rust I will try to create a generic library for terminal user interface management.
//!
//! This library uses [termios](https://docs.rs/termios/latest/termios/) as it's only dependency, which allows you to use it on multiple OSes.
//!
//! The goal is to make TUI creation fast & simple by providing sufficient documentation
//! with many boiled down examples that will allow developers to focus on their application's logic
//! instead of having to think too much about presentation layer annoyances.
//!
//! By making your programs more user-friendly you have a higher chance of growing a happy user base
//! of utilities you are creating.
//!
//! ## Setup
//! In order to use this library, you need to create a [`Manager`]:
//! ```no_run
//! use animaterm::prelude::*;
//!
//! let capture_keyboard = true;
//! let cols = Some(40);
//! let rows = None;  // use all rows available
//! let glyph = Some(Glyph::default());  // initially fill the screen with this
//! let mut mgr = Manager::new(capture_keyboard, cols, rows, glyph);
//! ```
//!
//! Please also note that in order to see the progress you are making with this library
//! you need to keep your program running, since animaterm makes use of an alternate buffer.
//! After your program finishes you go back to original buffer, wiping out all the graphics you have
//! placed on the screen so far.
//!
//! In order to keep your program running you can use a loop like this:
//! ```no_run
//! let mut keep_running = true;
//! while keep_running {
//!     if let Some(key) = mgr.read_key() {
//!         match key {
//!             Key::Q | Key::q => {
//!                 keep_running = false;
//!             }
//!             _ => continue
//!         }
//!     }
//! }
//! mgr.terminate();
//!
//! ```
//! ## Functionality
//! With `mgr` under your control you can do all kinds of things:
//! * create a [`Graphic`] containing multiple [`Frame`] with fully adjustable [`Color`] and [`Glyph`] - [see example](#create-a-graphic-containing-multiple-frames);
//! * add an [`Animation`] to a [`Graphic`] and run it - [see example](#add-an-animation-to-a-graphic);
//! * take action according to [`Key`] press - [see example](#take-action-according-to-key-press);
//! * change [`Frame`] or [`Animation`] of displayed [`Graphic`] to a different one - [see example1](#switch-selected-graphic-to-a-different-frame) [or example_2](#switch-selected-graphic-to-a-different-animation);
//! * pause selected [`Animation`] on selected [`Frame`] - [see example](#pause-selected-animation);
//! * switch back and forth between multiple [`Screen`] instances - [see example](#switch-between-screens);
//! * stack [`Graphic`] over or under others on a ['Screen'] with layer change - [see example](#change-graphic-layer);
//! * move [`Graphic`] up/down/left/right on a [`Screen`] - [see example](#move-graphic);
//! * update selected [`Glyph`] within existing [`Frame`] of a [`Graphic`] - [see example](#update-selected-glyph);
//! * make parts of a [`Graphic`] transparent by changing [`Glyph`] property - [see example](#make-parts-of-a-graphic-transparent);
//! * add cloned or completely new [`Frame`] to [`Graphic`] - [see example_1](#add-cloned-frame) [or example_2](#add-new-frame);
//! * and more.
//!
//! # Examples
//! ## Create a Graphic containing multiple Frames
//! ```
//! # use animaterm::prelude::*;
//! # let mut mgr = Manager::new(true, None, None, None);
//! let cols = 10;
//! let rows = 5;
//! let start_frame = 0;
//! let mut library = HashMap::with_capacity(2);
//!
//! library.insert(
//!     start_frame,
//!     vec![Glyph::default(); rows * cols]);
//! library.insert(
//!     start_frame+1,
//!     vec![Glyph::plain(); rows * cols]);
//!  let mut gr = Graphic::new(cols, rows, start_frame, library, None);
//!
//! let layer = 0;
//! let offset = (15, 5);
//! let graphic_id = mgr.add_graphic(gr, layer, offset);
//! mgr.set_graphic(graphic_id, start_frame);
//! ```
//! ## Add an Animation to a Graphic
//! ```
//! # use animaterm::prelude::*;
//! # let mut mgr = Manager::new(true, None, None, None);
//! let a = 1;
//!
//!
//! ```
//! ## Take action according to Key press
//! ```
//!
//!
//!
//! ```
//! ## Switch selected Graphic to a different Frame
//! ```
//!
//!
//!
//! ```
//! ## Switch selected Graphic to a different Animation
//! ```
//!
//!
//!
//! ```
//! ## Pause selected Animation
//! ```
//!
//!
//!
//! ```
//! ## Switch between Screens
//! ```
//!
//!
//!
//! ```
//! ## Change Graphic layer
//! ```
//!
//!
//!
//! ```
//! ## Move Graphic
//! ```
//!
//!
//!
//! ```
//! ## Update selected Glyph
//! ```
//!
//!
//!
//! ```
//! ## Make parts of a Graphic transparent
//! ```
//!
//!
//!
//! ```
//! ## Add cloned Frame
//! ```
//!
//!
//!
//! ```
//! ## Add new Frame
//! ```
//!
//!
//!
//! ```
//!
//! # TODO:
//! * Add Result sending to various RPCs requests
//! * Rename NewColor & Color
//! * Write nice documentation
//! * Graphic move should contain layer and offset as Option
//! * Write screenshot function
//! * Optimize for memory & CPU usage
//! * Write functions to load/store Graphics from/to text files
//!

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
    pub use crate::color::NewColor;
    pub use crate::error::AnimError;
    pub use crate::glyph::Glyph;
    pub use crate::graphic::Graphic;
    pub use crate::key::Key;
    pub use crate::manager::Manager;
    pub use crate::response::{AnimOk, AnimResult};
    pub use crate::time::Timestamp;
}
