# A library for terminal user interface (TUI) creation
The goal is to make TUI creation fast & simple by providing sufficient documentation
with many boiled down examples that will allow developers to focus on their application's logic
instead of having to think too much about presentation layer annoyances.

# Why
Inspired by [this post](http://xn--rpa.cc/irl/term.html) I decided that as my first more serious
project written in Rust I will try to create a generic library for terminal user interface management.

By making your programs more user-friendly you have a higher chance of growing a happy user base
of utilities you create.

Allowing users customizing the look and feel of those solutions with community driven
skins/themes and keybindings makes it more likely people will stay attached to those apps.

# How to use it
This library uses [termios](https://docs.rs/termios/latest/termios/) as it's only dependency, which allows you to use it on multiple OSes.

Graphic objects can be defined and loaded as a plaintext file. Graphic's building blocks - frames
are also text files, with optional ANSI escape codes that allow using colors and various styles.

Internally rendering is being served on a separate thread, so waiting for a key press does not make any difference to animation playback.

Attached [studio](../studio/index.html) app is a working proof this library does work. It can be used by non-programmers
so that people more design focused can help with user interface's look and feel.

## Setup
In order to use this library, you need to create a [`Manager`]:
```no_run
use animaterm::prelude::*;
use std::time::Duration;

let capture_keyboard = true;
let cols = Some(40);
let rows = None;  // use all rows available
let glyph = Some(Glyph::default());  // initially fill the screen with this
// You can crank refresh_timeout down, but anything below 1ms won't make a difference,
// other than high CPU usage.
// With default 30ms you get as high as 33 FPS, probably enough for a terminal application.
let refresh_timeout = Some(Duration::from_milis(10));  
let mut mgr = Manager::new(capture_keyboard, cols, rows, glyph, refresh_timeout);
```

Please also note that in order to see the progress you are making with this library
you need to keep your program running, since animaterm makes use of an alternate terminal buffer.
After your program finishes you go back to original buffer, wiping out all the graphics you have
placed on the screen so far.

In order to keep your program running you can use a loop like this:
```no_run
let mut keep_running = true;
 while keep_running {
    if let Some(key) = mgr.read_key() {
        match key {
            Key::Q | Key::ShiftQ => {
                keep_running = false;
            }
            _ => continue
        }
    }
}
mgr.terminate();

```
## Functionality
With `mgr` under your control you can do all kinds of things:
* create a [`Graphic`] containing multiple frames with fully adjustable [`Color`] and [`Glyph`] - [see example](#create-a-graphic-containing-multiple-frames);
* add an [`Animation`] to a [`Graphic`] and run it - [see example](#add-an-animation-to-a-graphic);
* take action according to [`Key`] press - [see example](#take-action-according-to-key-press);
* change Frame or [`Animation`] of displayed [`Graphic`] to a different one - [see example1](#switch-selected-graphic-to-a-different-frame) [or example_2](#switch-selected-graphic-to-a-different-animation);
* pause selected [`Animation`] on selected Frame - [see example](#pause-selected-animation);
* switch back and forth between multiple [`Display`] instances - [see example](#switch-between-displays);
* stack [`Graphic`] over or under others on a [`Display`] with layer change - [see example](#change-graphic-layer);
* move [`Graphic`] up/down/left/right on a [`Display`] - [see example](#move-graphic);
* update selected [`Glyph`] within existing Frame of a [`Graphic`] - [see example](#update-selected-glyph);
* make parts of a [`Graphic`] transparent by changing [`Glyph`] property - [see example](#make-parts-of-a-graphic-transparent);
* add cloned or completely new Frame to [`Graphic`] - [see example_1](#add-cloned-frame) [or example_2](#add-new-frame);
* and more.

# Examples
Please also check the [`examples`](../../../examples) directory for working examples of how to use this library.
## Create a Graphic containing multiple Frames
```
let cols = 10;
let rows = 5;
let start_frame = 0;
let mut library = HashMap::with_capacity(2);

library.insert(
    start_frame,
    vec![Glyph::default(); rows * cols]);
library.insert(
    start_frame+1,
    vec![Glyph::plain(); rows * cols]);
 let animations = None;
 let mut gr = Graphic::new(cols, rows, start_frame, library, animations);

let layer = 0;
let offset = (15, 5);
let graphic_id = mgr.add_graphic(gr, layer, offset).unwrap();
mgr.set_graphic(graphic_id, start_frame);
```
## Add an Animation to a Graphic
```
// You can define some animations upon creation of a graphic:
let running = false;
let looping = true;
let seconds = 0;
let miliseconds = 500;
let ordering = vec![
    (start_frame, Timestamp::new(seconds, miliseconds)),
    (start_frame + 1, Timestamp::new(seconds, miliseconds)),
];
let start_time = Timestamp::now();
let animation = Animation::new(running, looping, ordering, start_time);
let mut animations = HashMap::new();
let anim_id = 0;
animations.insert(anim_id, animation);

let mut gr = Graphic::new(cols, rows, start_frame, library, Some(animations));

// Or add a new animation to existing graphic
let option_anim_id = gr.add_animation(Animation::new(running, looping, ordering, start_time));

// You can even create additional animation via a manager
mgr.add_animation(
    graphic_id,
    Animation::new(running, looping, ordering, Timestamp::now()),
);

let mut var_anim_id = None;
 if let Ok(AnimOk::AnimationAdded(anim_id)) = mgr.read_result() {
    var_anim_id = Some(anim_id);
}
```
## Take action according to Key press
For more agile solution allowing user-defined key bindings see how [studio](../../../src/bin/studio/main.rs) implements user input loop.
```
let mut keep_running = true;
while keep_running {
    if let Some(key) = mgr.read_key() {
        match key {
            Key::Left => mgr.move_graphic(graphic_id, layer, (-1, 0)),
            Key::Right => mgr.move_graphic(graphic_id, layer, (1, 0)),
            Key::Up => mgr.move_graphic(graphic_id, layer, (0, -1)),
            Key::Down => mgr.move_graphic(graphic_id, layer, (0, 1)),
            Key::Q | Key::ShiftQ => {
                keep_running = false;
            }
            _ => continue,
        }
    }
}
 mgr.terminate();
```
## Switch selected Graphic to a different Frame
```
// Use force wisely, since it causes entire screen to be refreshed,
// thus app is becoming less responsive.
let force = true;
mgr.set_graphic(graphic_id, frame_id, force)
```
## Switch selected Graphic to a different Animation
```
mgr.start_animation(graphic_id, anim_id);
```
## Pause selected Animation
```
//You can pause immediately
mgr.pause_animation(graphic_id);

//You can pause on a selected frame
mgr.pause_animation_on_frame(graphic_id, frame_id);
```
## Switch between Displays
```
let keep_existing = true;
let second_display_id = mgr.new_display(keep_existing);
// default display has id = 0
mgr.restore_display(0, keep_existing);
```
## Change Graphic layer
```
mgr.move_graphic(graphic_id, new_layer, (0, 0));
```
## Move Graphic
```
mgr.move_graphic(graphic_id, layer, (offset_cols, offset_rows));
```
## Update selected Glyph
```
// Change a Glyph for a selected Frame
a_graphic.set_frame(frame_id);
a_graphic.set_glyph(new_glyph, col, row, graphic_offset);

// Change a Glyph for current Frame of an on Screen Graphic
mgr.set_glyph(graphic_id, new_glyph, col, row);
```
## Make parts of a Graphic transparent
```
// Make a transparent Glyph for a selected Frame
a_graphic.set_frame(frame_id, graphic_offset);
a_graphic.set_glyph(Glyph::transparent(), col, row, graphic_offset);

// Change a Glyph to transparent for current Frame of an on Screen Graphic
mgr.set_glyph(graphic_id, Glyph::transparent(), col, row, graphic_offset));
```
## Add cloned Frame
```
// In both cases empty Frame becomes current for that Graphic

// if source_frame_id is None current Frame will get cloned
let source_frame_id = Some(id);

// Add a new Frame to a graphic directly
let frame_id_option = a_graphic.clone_frame(source_frame_id);

// Or use a Manager to do so
mgr.clone_frame(graphic_id);
if let Ok(AnimOk::FrameAdded(graphic_id, frame_id)) = mgr.read_result() {
    let added_frame_id = frame_id;
}
```
## Add new Frame
```
// In both cases empty Frame becomes current for that Graphic

// Add a new Frame to a graphic directly
let frame_id_option = a_graphic.empty_frame();

// Or use a Manager to do so
mgr.empty_frame(graphic_id);
if let Ok(AnimOk::FrameAdded(graphic_id, frame_id)) = mgr.read_result() {
    let added_frame_id = frame_id;
}
```



