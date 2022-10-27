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
//! use std::time::Duration;
//!
//! let capture_keyboard = true;
//! let cols = Some(40);
//! let rows = None;  // use all rows available
//! let glyph = Some(Glyph::default());  // initially fill the screen with this
//! // You can crank refresh_timeout down, but anything below 1ms won't make a difference,
//! // other than high CPU usage.
//! // With default 30ms you get as high as 33 FPS, probably enough for a terminal application.
//! let refresh_timeout = Some(Duration::from_milis(10));  
//! let mut mgr = Manager::new(capture_keyboard, cols, rows, glyph, refresh_timeout);
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
//! * create a [`graphic.Graphic`] containing multiple Frame with fully adjustable [`Color`] and [`Glyph`] - [see example](#create-a-graphic-containing-multiple-frames);
//! * add an [`animation.Animation`] to a [`graphic.Graphic`] and run it - [see example](#add-an-animation-to-a-graphic);
//! * take action according to [`key.Key`] press - [see example](#take-action-according-to-key-press);
//! * change Frame or [`animation.Animation`] of displayed [`graphic.Graphic`] to a different one - [see example1](#switch-selected-graphic-to-a-different-frame) [or example_2](#switch-selected-graphic-to-a-different-animation);
//! * pause selected [`animation.Animation`] on selected Frame - [see example](#pause-selected-animation);
//! * switch back and forth between multiple [`screen.Screen`] instances - [see example](#switch-between-displays);
//! * stack [`graphic.Graphic`] over or under others on a ['screen.Screen'] with layer change - [see example](#change-graphic-layer);
//! * move [`graphic.Graphic`] up/down/left/right on a [`screen.Screen`] - [see example](#move-graphic);
//! * update selected [`glyph.Glyph`] within existing Frame of a [`graphic.Graphic`] - [see example](#update-selected-glyph);
//! * make parts of a [`graphic.Graphic`] transparent by changing [`glyph.Glyph`] property - [see example](#make-parts-of-a-graphic-transparent);
//! * add cloned or completely new Frame to [`graphic.Graphic`] - [see example_1](#add-cloned-frame) [or example_2](#add-new-frame);
//! * and more.
//!
//! # Examples
//! ## Create a Graphic containing multiple Frames
//! ```
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
//!  let animations = None;
//!  let mut gr = Graphic::new(cols, rows, start_frame, library, animations);
//!
//! let layer = 0;
//! let offset = (15, 5);
//! let graphic_id = mgr.add_graphic(gr, layer, offset);
//! mgr.set_graphic(graphic_id, start_frame);
//! ```
//! ## Add an Animation to a Graphic
//! ```
//! // You can define some animations upon creation of a graphic:
//! let running = false;
//! let looping = true;
//! let seconds = 0;
//! let miliseconds = 500;
//! let ordering = vec![
//!     (start_frame, Timestamp::new(seconds, miliseconds)),
//!     (start_frame + 1, Timestamp::new(seconds, miliseconds)),
//! ];
//! let start_time = Timestamp::now();
//! let animation = Animation::new(running, looping, ordering, start_time);
//! let mut animations = HashMap::new();
//! let anim_id = 0;
//! animations.insert(anim_id, animation);
//!
//! let mut gr = Graphic::new(cols, rows, start_frame, library, Some(animations));
//!
//! // Or add a new animation to existing graphic
//! let anim_id = gr.add_animation(Animation::new(running, looping, ordering, start_time));
//!
//! // You can even create additional animation via a manager
//! mgr.add_animation(
//!     graphic_id,
//!     Animation::new(running, looping, ordering, Timestamp::now()),
//! );
//!
//! let mut var_anim_id = 0;
//!  if let Ok(AnimOk::AnimationAdded(anim_id)) = mgr.read_result() {
//!     var_anim_id = anim_id;
//! }
//! ```
//! ## Take action according to Key press
//! ```
//! let mut keep_running = true;
//! while keep_running {
//!     if let Some(key) = mgr.read_key() {
//!         match key {
//!             Key::Left => mgr.move_graphic(graphic_id, layer, (-1, 0)),
//!             Key::Right => mgr.move_graphic(graphic_id, layer, (1, 0)),
//!             Key::Up => mgr.move_graphic(graphic_id, layer, (0, -1)),
//!             Key::Down => mgr.move_graphic(graphic_id, layer, (0, 1)),
//!             Key::Q | Key::q => {
//!                 keep_running = false;
//!             }
//!             _ => continue,
//!         }
//!     }
//! }
//!  mgr.terminate();
//! ```
//! ## Switch selected Graphic to a different Frame
//! ```
//! let force = true;
//! mgr.set_graphic(graphic_id, frame_id, force)
//! ```
//! ## Switch selected Graphic to a different Animation
//! ```
//! mgr.new_start_animation(graphic_id, anim_id);
//! ```
//! ## Pause selected Animation
//! ```
//! //You can pause immediately
//! mgr.pause_animation(graphic_id);
//!
//! //You can pause on a selected frame
//! mgr.pause_animation_on_frame(graphic_id, frame_id);
//! ```
//! ## Switch between Displays
//! ```
//! let second_display_id = mgr.new_display(keep_existing);
//! mgr.restore_display(0, true);
//! ```
//! ## Change Graphic layer
//! ```
//! mgr.move_graphic(graphic_id, new_layer, (0, 0));
//! ```
//! ## Move Graphic
//! ```
//! mgr.move_graphic(graphic_id, layer, (offset_cols, offset_rows));
//! ```
//! ## Update selected Glyph
//! ```
//! // Change a Glyph for a selected Frame
//! a_graphic.set_frame(frame_id);
//! a_graphic.set_glyph(new_glyph, col, row);
//!
//! // Change a Glyph for current Frame of an on Screen Graphic
//! mgr.set_glyph(graphic_id, new_glyph, col, row);
//! ```
//! ## Make parts of a Graphic transparent
//! ```
//! // Make a transparent Glyph for a selected Frame
//! a_graphic.set_frame(frame_id);
//! a_graphic.set_glyph(Glyph::transparent(), col, row);
//!
//! // Change a Glyph to transparent for current Frame of an on Screen Graphic
//! mgr.set_glyph(graphic_id, Glyph::transparent(), col, row);
//! ```
//! ## Add cloned Frame
//! ```
//! // In both cases empty Frame becomes current for that Graphic
//!
//! // if source_frame_id is None current Frame will get cloned
//! let source_frame_id = Some(id);
//!
//! // Add a new Frame to a graphic directly
//! let frame_id_option = a_graphic.clone_frame(source_frame_id);
//!
//! // Or use a Manager to do so
//! mgr.clone_frame(graphic_id);
//! if let Ok(AnimOk::FrameAdded(graphic_id, frame_id)) = mgr.read_result() {
//!     let added_frame_id = frame_id;
//! }
//! ```
//! ## Add new Frame
//! ```
//! // In both cases empty Frame becomes current for that Graphic
//!
//! // Add a new Frame to a graphic directly
//! let frame_id_option = empty_frame();
//!
//! // Or use a Manager to do so
//! mgr.empty_frame(graphic_id);
//! if let Ok(AnimOk::FrameAdded(graphic_id, frame_id)) = mgr.read_result() {
//!     let added_frame_id = frame_id;
//! }
//! ```
//!
//! # TODO:
//! * Add Result sending to various RPCs requests
//! * Write nice documentation
//! * Graphic move should contain layer and offset as Option
//! * Write screenshot function
//! * Optimize for memory & CPU usage
//! * Write functions to load/store Graphics from/to text files
//! * Write animaterm studio for easy Graphics creation
//!

mod error;
pub use error::AnimError;
mod manager;
pub use manager::{Manager, Message};
mod response;
pub use response::AnimOk;
mod screen;
mod time;
pub use time::Timestamp;
mod graphic;
pub use graphic::Graphic;
mod animation;
pub use animation::Animation;
mod color;
pub use color::{Color, ColorName};
pub mod glyph;
mod pixel;
pub use glyph::Glyph;
mod display;
mod glyphcake;
mod helpers;
mod key;
pub use key::str_to_key;
pub use key::Key;
mod tests;
pub mod utilities;
pub use display::Display;
mod frame;
pub use frame::from_file;

pub mod prelude {
    pub use crate::animation::Animation;
    pub use crate::color::Color;
    pub use crate::color::ColorName;
    pub use crate::error::AnimError;
    pub use crate::from_file as frame_from_file;
    pub use crate::glyph::Glyph;
    pub use crate::graphic::Graphic;
    pub use crate::key::Key;
    pub use crate::manager::Manager;
    pub use crate::response::AnimOk;
    pub use crate::time::Timestamp;
}
