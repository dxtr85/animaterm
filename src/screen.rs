use super::animation::Animation;
use super::color::Color;
use super::display::Display;
use super::graphic::Graphic;
use super::helpers::ask_os_for_rows_and_cols;
use super::pixel::Pixel;
use super::time::Timestamp;
use super::Glyph;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::mem::replace;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

struct ShelvedItem(Display, HashMap<usize, (Graphic, usize, (isize, isize))>);

pub struct Screen {
    pub rows: usize,
    pub cols: usize,
    // display should support transparent Glyphs inside Animations
    // should be Vec containing a structure with layered Glyphs occupying given pixel
    // last non-transparent Glyph for each pixel should be returned.
    // it should be known which pixels on what layer belong to given id of animation
    // it should be possible to move animation from one layer to another
    // a flag on every pixel should notify if a given pixel should be printed
    pub display: Display,
    shelve: HashMap<usize, ShelvedItem>,
    shelve_id: usize,
    time: Timestamp,
    next_available_id: usize,
    // animations: HashMap<usize, (Animation, usize, (usize, usize))>,
    graphics: HashMap<usize, (Graphic, usize, (isize, isize))>,
    stdin: i32,
    stdout: io::Stdout,
    termios_orig: Termios,
    termios: Termios,
    c_color: Color,
    c_background: Color,
    c_x: usize,
    c_y: usize,
    // c_plain: bool,
    c_bright: bool,
    c_dim: bool,
    c_italic: bool,
    c_underline: bool,
    c_blink: bool,
    c_blink_fast: bool,
    c_reverse: bool,
    c_transparent: bool,
    c_strike: bool,
    chars_sent: usize,
    chars_refresh: usize,
}

impl Screen {
    /// Create a new Screen instance with given dimentions and fills it with provided glyph.
    pub fn new(cols: Option<usize>, rows: Option<usize>, glyph: Option<Glyph>) -> Self {
        let (new_rows, new_cols) = ask_os_for_rows_and_cols();
        eprintln!("OS provided {} rows and {} cols", new_rows, new_cols);
        let final_rows = if let Some(rows) = rows {
            rows
        } else {
            new_rows
        };
        let final_cols = if let Some(cols) = cols {
            cols
        } else {
            new_cols
        };
        let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work
                       // on /dev/stdin or /dev/tty
        let termios = Termios::from_fd(stdin).expect("Could not get Termios instance from stdin.");
        let new_termios = termios; // make a mutable copy of termios
                                   // that we will modify
        let c_x = final_cols;
        let c_y = final_rows;
        let mut dglyph = Glyph::default();
        if glyph.is_some() {
            dglyph = glyph.unwrap();
        }
        let display = Display::new(0, dglyph, final_cols, final_rows);
        Screen {
            rows: final_rows,
            cols: final_cols,
            display,
            shelve: HashMap::new(),
            shelve_id: 0,
            time: Timestamp::now(),
            next_available_id: 0,
            // animations: HashMap::with_capacity(5),
            graphics: HashMap::with_capacity(5),
            stdin,
            stdout: io::stdout(),
            termios_orig: termios,
            termios: new_termios,
            c_color: dglyph.color,
            c_background: dglyph.background,
            c_x,
            c_y,
            // c_plain: dglyph.plain,
            c_bright: dglyph.bright,
            c_dim: dglyph.dim,
            c_italic: dglyph.italic,
            c_blink: dglyph.blink,
            c_blink_fast: dglyph.blink_fast,
            c_underline: dglyph.underline,
            c_reverse: dglyph.reverse,
            c_transparent: dglyph.transparent,
            c_strike: dglyph.strike,
            chars_sent: 0,
            chars_refresh: 100,
        }
    }

    /// Swap current display with a new one. If required old display can be stored for later use.
    pub fn new_display(&mut self, display_id: usize, keep_existing: bool) -> usize {
        let new_display = Display::new(display_id, Glyph::default(), self.cols, self.rows);
        //let mut return_id = None;
        let old_display = replace(&mut self.display, new_display);
        //let current_time = self.time.tick();
        let old_graphics = self.graphics.drain().collect();
        if keep_existing {
            self.shelve
                .insert(old_display.id, ShelvedItem(old_display, old_graphics));
            //return_id = Some(self.shelve_id);
            self.shelve_id += 1;
        }
        self.clear_screen();
        display_id
    }

    /// Restore an old display, keep existing one if needed.
    pub fn restore_display(&mut self, display_id: usize, keep_existing: bool) -> Option<usize> {
        let mut return_id = None;
        if let Some(ShelvedItem(shelved_display, shelved_graphics)) =
            self.shelve.remove(&display_id)
        {
            return_id = Some(display_id);
            //let current_time = self.time.tick();
            let old_display = replace(&mut self.display, shelved_display);
            let old_graphics = replace(&mut self.graphics, shelved_graphics);
            if keep_existing {
                self.shelve
                    .insert(old_display.id, ShelvedItem(old_display, old_graphics));
            }
            let to_print = self.refresh(true);
            self.print_all(to_print);
        }
        return_id
    }

    /// Add a new graphic to screen's current display.
    pub fn add_graphic(&mut self, graphic: Graphic, layer: usize, offset: (isize, isize)) -> usize {
        let graphic_id = self.next_available_id;
        self.next_available_id += 1;
        self.graphics.insert(graphic_id, (graphic, layer, offset));
        graphic_id
    }

    /// Move a graphic to new layer and/or screen offset.
    pub fn move_graphic(&mut self, graphic_id: usize, layer: usize, offset: (isize, isize)) {
        let mut cl_args = Vec::new();
        let mut new_offset = None;
        let mut frame_id = None;
        if let Some(current_state) = self.graphics.get_mut(&graphic_id) {
            frame_id = Some(current_state.0.current_frame);
            let delta_zero = current_state.2 .0 + offset.0;
            let delta_one = current_state.2 .1 + offset.1;
            new_offset = Some((delta_zero, delta_one));

            // Layer has changed
            if current_state.1 != layer {
                cl_args.push((
                    current_state.1,
                    max(current_state.2 .0, 0) as usize,
                    max(current_state.2 .1, 0) as usize,
                    current_state.0.cols,
                    current_state.0.rows,
                ));
            }

            // Move left
            if offset.0 < 0 {
                cl_args.push((
                    layer,
                    max(current_state.2 .0 - 1 + current_state.0.cols as isize, 0) as usize,
                    max(current_state.2 .1, 0) as usize,
                    (offset.0).unsigned_abs() - 1,
                    current_state.0.rows,
                ));

                // Move right
            } else if offset.0 > 0 && current_state.2 .0 + offset.0 > 0 {
                cl_args.push((
                    layer,
                    max(current_state.2 .0, 0) as usize,
                    max(current_state.2 .1, 0) as usize,
                    min(current_state.0.cols, offset.0 as usize - 1),
                    current_state.0.rows,
                ));
            }

            // Move up
            if offset.1 < 0 {
                cl_args.push((
                    layer,
                    max(current_state.2 .0, 0) as usize,
                    max(current_state.2 .1 - 1 + current_state.0.rows as isize, 0) as usize,
                    current_state.0.cols + 1,
                    (offset.1).unsigned_abs() - 1,
                ));
            } else if offset.1 > 0 && current_state.2 .1 + offset.1 > 0 {
                cl_args.push((
                    layer,
                    max(current_state.2 .0, 0) as usize,
                    max(current_state.2 .1, 0) as usize,
                    current_state.0.cols,
                    min(current_state.0.rows, offset.1 as usize),
                    // offset.1 as usize,
                ));
            }
        }
        for c in cl_args {
            self.clear_area(c.0, c.1, c.2, c.3, c.4);
        }
        if let Some(no) = new_offset {
            self.update_graphics_layer(graphic_id, layer);
            self.update_graphics_offset(graphic_id, no);
        }
        if let Some(fid) = frame_id {
            self.set_graphic(&graphic_id, &fid, false);
            if let Some((graphic, layer, offset)) = self.graphics.get(&graphic_id) {
                let pixels = graphic.get_pixels(*offset);
                self.update(vec![(pixels, *layer)]);
            }
        }
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    /// Delete a graphic from display.
    pub fn delete_graphic(&mut self, graphic_id: &usize) {
        let mut clear_info = None;
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(graphic_id) {
            clear_info = Some((*layer, offset.0, offset.1, graphic.cols, graphic.rows));
        }
        if let Some(c) = clear_info {
            self.clear_area(c.0, max(c.1, 0) as usize, max(c.2, 0) as usize, c.3, c.4);
        }
    }

    /// Set a graphic to given frame.
    pub fn set_graphic(&mut self, graphic_id: &usize, frame_id: &usize, force: bool) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(graphic_id) {
            results.push((graphic.set_frame(frame_id, *offset, force), *layer));
        }
        self.update(results);
        let to_print = self.refresh(force);
        self.print_all(to_print);
    }

    /// Get a vector of strings representing current display's contents. Each string represents a line.
    pub fn print_screen(&mut self) -> Vec<String> {
        let to_print = self.refresh(true);
        self.get_text(to_print)
    }

    /// Get a vector of strings representing selected area of a display.
    pub fn print_screen_section(
        &mut self,
        offset: (usize, usize),
        cols: usize,
        rows: usize,
    ) -> Vec<String> {
        let whole_screen = self.refresh(true);
        let min_x = offset.0 + 1;
        let min_y = offset.1 + 1;
        let max_x = offset.0 + cols;
        let max_y = offset.1 + rows + 1;

        let mut to_print = Vec::new();
        for (x, y, glyph) in whole_screen {
            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                to_print.push((x, y, glyph));
            }
        }
        self.get_text(to_print)
    }

    /// Get a vector of strings representing given graphic.
    pub fn print_graphic(&mut self, graphic_id: usize, skip_border: bool) -> Vec<String> {
        let mut to_print = Vec::new();
        if let Some((graphic, _layer, _offset)) = self.graphics.get(&graphic_id) {
            let mut pixels = graphic.get_pixels((0, 0));
            if skip_border {
                let max_c = graphic.cols - 1;
                let max_r = graphic.rows;
                pixels.retain(|p| p.x > 0 && p.x < max_c && p.y > 0 && p.y < max_r);
            }
            for p in pixels {
                to_print.push((p.x, p.y, p.g));
            }
        }
        self.get_text(to_print)
    }

    /// Set all glyphs of graphic's frame to given color.
    pub fn set_graphic_color(&mut self, graphic_id: usize, color: Color) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(&graphic_id) {
            graphic.set_current_frame_color(color);
            let curr_frame = graphic.current_frame;
            results.push((graphic.set_frame(&curr_frame, *offset, true), *layer));
        }
        self.update(results);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    /// Set all glyphs of graphic's frame to given background color.
    pub fn set_graphic_background(&mut self, graphic_id: usize, color: Color) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(&graphic_id) {
            graphic.set_current_frame_background(color);
            let curr_frame = graphic.current_frame;
            results.push((graphic.set_frame(&curr_frame, *offset, true), *layer));
        }
        self.update(results);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    /// Set all glyphs of graphic's frame to given style.
    pub fn set_graphic_style(&mut self, graphic_id: usize, style: Glyph) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(&graphic_id) {
            graphic.set_current_frame_style(style);
            let curr_frame = graphic.current_frame;
            results.push((graphic.set_frame(&curr_frame, *offset, true), *layer));
        }
        self.update(results);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    /// Set visibility for given graphic.
    pub fn set_invisible(&mut self, graphic_id: usize, invisible: bool) {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&graphic_id) {
            let pixels = graphic.set_invisible(invisible, offset);
            let fid = graphic.current_frame;
            self.graphics.insert(graphic_id, (graphic, layer, offset));
            self.set_graphic(&graphic_id, &fid, false);
            self.update(vec![(pixels, layer)]);
            let to_print = self.refresh(false);
            self.print_all(to_print);
        }
    }

    /// Set one particular glyph of current graphic's frame to given value.
    pub fn set_glyph(&mut self, graphic_id: usize, glyph: Glyph, col: usize, row: usize) {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&graphic_id) {
            let pixel = graphic.set_glyph(glyph, col, row, offset);
            let fid = graphic.current_frame;
            self.graphics.insert(graphic_id, (graphic, layer, offset));
            self.set_graphic(&graphic_id, &fid, false);
            self.update(vec![(pixel, layer)]);
            let to_print = self.refresh(false);
            self.print_all(to_print);
        }
    }

    /// Get a glyph from a graphic located by given coordinates.
    pub fn get_glyph(&mut self, graphic_id: usize, col: usize, row: usize) -> Option<Glyph> {
        if let Some((gr, _l, _o)) = self.graphics.get(&graphic_id) {
            return gr.get_glyph(col, row);
        }
        None
    }

    /// Insert an empty frame to a graphic.
    pub fn empty_frame(&mut self, graphic_id: usize) -> Option<usize> {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&graphic_id) {
            let result = graphic.empty_frame();
            self.graphics.insert(graphic_id, (graphic, layer, offset));
            if let Some(result) = result {
                self.set_graphic(&graphic_id, &result, true);
            }
            return result;
        }
        None
    }

    /// Insert a new frame for given graphic cloned from requested frame (or current frame if None provided).
    pub fn clone_frame(&mut self, graphic_id: usize, frame_id: Option<usize>) -> Option<usize> {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&graphic_id) {
            let mut fr_id = graphic.current_frame;
            if let Some(id) = frame_id {
                fr_id = id;
            }
            let result = graphic.clone_frame(fr_id);
            self.graphics.insert(graphic_id, (graphic, layer, offset));
            if let Some(result) = result {
                self.set_graphic(&graphic_id, &result, true);
            }
            return result;
        }
        None
    }

    /// Set a graphic's layer to given value.
    pub fn update_graphics_layer(&mut self, graphic_id: usize, layer: usize) {
        if let Some((graphic, _layer, offset)) = self.graphics.remove(&graphic_id) {
            self.graphics.insert(graphic_id, (graphic, layer, offset));
        }
    }

    /// Set a graphic's offset to given value.
    pub fn update_graphics_offset(&mut self, graphic_id: usize, offset: (isize, isize)) {
        if let Some((graphic, layer, _offset)) = self.graphics.remove(&graphic_id) {
            self.graphics.insert(graphic_id, (graphic, layer, offset));
        }
    }

    /// Add an animation to a graphic.
    pub fn add_animation(&mut self, graphic_id: usize, a: Animation) -> Option<usize> {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&graphic_id) {
            let add_result = graphic.add_animation(a);
            self.graphics.insert(graphic_id, (graphic, layer, offset));
            add_result
        } else {
            None
        }
    }

    /// Start an animation for given graphic.
    pub fn start_animation(&mut self, graphic_id: &usize, aid: usize) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(graphic_id) {
            graphic.start_animation(aid, self.time.tick());
        }
    }

    /// Enqueue an animation to a given graphic to be run after other animation that is currently running finishes.
    pub fn enqueue_animation(&mut self, graphic_id: &usize, aid: usize, when: Timestamp) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(graphic_id) {
            graphic.enqueue_animation(aid, self.time.tick() + when);
        }
    }

    /// Start given animation from beginning.
    pub fn restart_animation(&mut self, graphic_id: usize, aid: usize, when: Timestamp) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(&graphic_id) {
            graphic.restart_animation(aid, when);
        }
    }

    /// Pause given animation on current frame
    pub fn pause_animation(&mut self, graphic_id: usize) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(&graphic_id) {
            if let Some(anim_id) = graphic.running_anim {
                graphic.pause_animation(anim_id, Timestamp::now());
            }
        }
    }

    /// Stop running an animation when a particular frame is being displayed.
    pub fn pause_animation_on_frame(&mut self, graphic_id: &usize, fid: usize) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(graphic_id) {
            if let Some(anim_id) = graphic.running_anim {
                graphic.pause_animation_on_frame(anim_id, fid);
            }
            //anim.pause_on_frame(fid);
        }
    }

    /// Stop running an animation.
    pub fn stop_animation(&mut self, graphic_id: &usize) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(graphic_id) {
            graphic.stop_animation();
        }
    }

    /// Update all graphics that run an animation.
    pub fn update_graphics(&mut self) {
        let mut pixels = vec![];
        for (graphic, layer, offset) in self.graphics.values_mut() {
            let mut keep_running = false;
            if graphic.running_anim.is_none() {
                if let Some((anim_id, when)) = graphic.awaiting_anim {
                    if self.time.tick() >= when {
                        graphic.start_animation(anim_id, self.time.tick());
                        graphic.awaiting_anim = None;
                    }
                }
            }
            if let Some(anim_id) = graphic.running_anim {
                keep_running = true;
                if let Some(anim) = graphic.animations.get_mut(&anim_id) {
                    if let Some((frame_id, running)) = anim.update(self.time.tick()) {
                        pixels.push((graphic.set_frame(&frame_id, *offset, false), *layer));
                        keep_running = running;
                    }
                }
            }

            if !keep_running {
                graphic.running_anim = None
            }
        }
        self.update(pixels);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    /// Clear entire screen.
    pub fn clear_screen(&mut self) {
        print!("\x1b[0m\x1b[21;22;23;24;25;26;27;29;37;40m\x1b[1;1Hm");
        println!("\x1b[H{:<1$}", "", self.cols * self.rows);
        self.flush_out();
    }

    /// Clear an area of a screen.
    pub fn clear_area(
        &mut self,
        layer: usize,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
    ) {
        let mut to_clear = Vec::with_capacity(width * height);
        let gplain = Glyph::transparent();
        for x in start_x..start_x + width + 1 {
            for y in start_y..start_y + height + 1 {
                to_clear.push(Pixel::new(x, y, gplain));
            }
        }
        self.update(vec![(to_clear, layer)]);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    /// Get a vector of localized glyphs to be refreshed (or entire screen if using force).
    fn refresh(&mut self, force: bool) -> Vec<(usize, usize, Glyph)> {
        let mut cap = 64;
        if force {
            cap = self.cols * self.rows;
        }
        let mut to_print = Vec::with_capacity(cap);
        for gcake in self.display.array.iter_mut() {
            if gcake.modified || force {
                to_print.push((gcake.col, gcake.row, gcake.get_glyph()));
            }
        }
        to_print
    }

    /// Print all provided Glyphs on screen.
    pub fn print_all(&mut self, glyphs: Vec<(usize, usize, Glyph)>) {
        for (x, y, g) in glyphs {
            self.print(x, y, g);
        }
        self.flush_out();
    }

    fn flush_out(&mut self) {
        self.stdout.lock().flush().expect("Flushing stdout failed.");
        self.c_x = self.cols;
        self.c_y = self.rows;
        self.c_color = Color::white();
        self.c_background = Color::black();
        self.c_blink = false;
        self.c_blink_fast = false;
        self.c_bright = false;
        self.c_dim = false;
        self.c_italic = false;
        self.c_strike = false;
        self.c_transparent = false;
        self.c_reverse = false;
        self.c_underline = false;
    }

    /// Convert a vector of localized glyphs into a vector of strings each representing a line of text.
    pub fn get_text(&mut self, glyphs: Vec<(usize, usize, Glyph)>) -> Vec<String> {
        let mut result = Vec::with_capacity(self.rows);
        let mut line_text = String::new();
        let mut last_line = 10;
        let mut first_glyph = true;
        for (_x, y, glyph) in glyphs {
            if y != last_line {
                if !line_text.is_empty() {
                    result.push(line_text);
                    line_text = String::new();
                    first_glyph = true;
                }
                last_line = y;
            }
            let modifier = self.gformat_out(glyph, first_glyph);
            if !modifier.is_empty() {
                line_text.push_str(&format!("\x1b[{}m{}", modifier, glyph.character));
                // line_text.push_str(&modifier);
                // line_text.push('m');
            } else {
                line_text.push(glyph.character);
            }
            first_glyph = false;
        }
        if !result.is_empty() {
            let mut last = result.pop().unwrap();
            last.push_str("\x1b[0m");
            result.push(last);
        }
        result
    }

    /// Update a pixel to new value.
    pub fn update(&mut self, pixels: Vec<(Vec<Pixel>, usize)>) {
        for (ps, layer) in pixels {
            for p in ps {
                if p.x >= self.cols || p.y >= self.rows {
                    continue;
                }
                let x = p.x; //.saturating_sub(1);
                let y = p.y; //.saturating_sub(1);
                let index = x + (y * self.cols);
                let cake = self.display.array.get_mut(index).expect("WTF?!");
                cake.update(p.g, layer);
            }
        }
    }

    /// Print a glyph on screent in given location.
    pub fn print(&mut self, x: usize, y: usize, glyph: Glyph) {
        if x > self.cols || y > self.rows {
            return;
        }
        let mut formated = String::new();
        if self.c_x != x || self.c_y != y {
            formated.push_str(&format!("\x1b[{};{}H", y, x));
        };
        self.c_x = x + 1;
        self.c_y = y;
        let modifier = self.gformat(glyph);
        if !modifier.is_empty() {
            formated.push_str("\x1b[");
            formated.push_str(&modifier);
            formated.push('m');
        }

        print!("{}{}", formated, glyph.character);
    }

    /// Get a string representing given glyph for writing into a text file.
    fn gformat_out(&mut self, glyph: Glyph, first_glyph: bool) -> String {
        let mut modifier = String::new(); //"\x1b[".to_string();
                                          // let mut add_modifier = false;
                                          // Plain = 0,
        let mut set_plain_on_exit = false;
        let mut push_colors = false;

        if first_glyph {
            //self.c_plain = true;
            self.c_bright = false;
            self.c_dim = false;
            self.c_italic = false;
            self.c_underline = false;
            self.c_blink = false;
            self.c_blink_fast = false;
            self.c_reverse = false;
            self.c_transparent = false;
            self.c_strike = false;
            self.c_color = Color::white();
            self.c_background = Color::black();
            modifier.push_str("0;");
        }
        if self.c_bright && !glyph.bright {
            self.c_bright = false;
            if glyph.dim {
                if !self.c_dim {
                    self.c_dim = true;
                    modifier.push_str("2;");
                }
            } else {
                modifier.push_str("22;");
            }
            push_colors = true;
        } else if !self.c_bright && glyph.bright {
            self.c_bright = true;
            if self.c_dim {
                self.c_dim = false;
                // modifier.push_str("22;");
            }
            modifier.push_str("1;");
            push_colors = true;
        }
        if self.c_dim && !glyph.dim {
            self.c_dim = false;
            if glyph.bright {
                modifier.push_str("1;");
                self.c_bright = true;
            } else {
                modifier.push_str("22;");
            }
            push_colors = true;
        } else if !self.c_dim && glyph.dim {
            self.c_dim = true;
            modifier.push_str("2;");
            push_colors = true;
        }
        if self.c_italic && !glyph.italic {
            self.c_italic = false;
            modifier.push_str("23;");
            push_colors = true;
        } else if !self.c_italic && glyph.italic {
            self.c_italic = true;
            push_colors = true;
            modifier.push_str("3;");
        }
        if self.c_underline && !glyph.underline {
            self.c_underline = false;
            modifier.push_str("24;");
            push_colors = true;
        } else if !self.c_underline && glyph.underline {
            self.c_underline = true;
            modifier.push_str("4;");
            push_colors = true;
        }
        if self.c_blink && !glyph.blink {
            self.c_blink = false;
            push_colors = true;
        } else if !self.c_blink && glyph.blink {
            self.c_blink = true;
            modifier.push_str("5;");
            push_colors = true;
        }
        if self.c_blink_fast && !glyph.blink_fast {
            self.c_blink_fast = false;
            push_colors = true;
        } else if !self.c_blink_fast && glyph.blink_fast {
            self.c_blink_fast = true;
            modifier.push_str("6;");
            push_colors = true;
        }
        if !glyph.blink && !glyph.blink_fast && (self.c_blink || self.c_blink_fast) {
            modifier.push_str("25;");
            push_colors = true;
        }
        if self.c_reverse && !glyph.reverse {
            self.c_reverse = false;
            modifier.push_str("27;");
            push_colors = true;
        } else if !self.c_reverse && glyph.reverse {
            self.c_reverse = true;
            modifier.push_str("7;");
            push_colors = true;
        }
        if self.c_transparent && !glyph.transparent {
            self.c_transparent = false;
            push_colors = true;
            modifier.push_str("28;");
        } else if !self.c_transparent && glyph.transparent {
            self.c_transparent = true;
            modifier.push_str("8;");
            push_colors = true;
        }
        if self.c_strike && !glyph.strike {
            self.c_strike = false;
            modifier.push_str("29;");
            push_colors = true;
        } else if !self.c_strike && glyph.strike {
            self.c_strike = true;
            modifier.push_str("9;");
            push_colors = true;
        }
        if self.c_color != glyph.color || push_colors {
            match glyph.color {
                Color::Basic(color) => modifier.push_str(&format!("3{};", color as u8)),
                Color::EightBit(color) => modifier.push_str(&format!("38;5;{};", color)),
                Color::Grayscale(brightness) => modifier.push_str(&format!("38;5;{};", brightness)),
                Color::Truecolor(red, green, blue) => {
                    modifier.push_str(&format!("38;2;{};{};{};", red, green, blue))
                }
            }
            self.c_color = glyph.color;
        };
        if self.c_background != glyph.background || push_colors {
            //modifier.push_str(&format!("4{}", glyph.background as u8));
            match glyph.background {
                Color::Basic(color) => modifier.push_str(&format!("4{}", color as u8)),
                Color::EightBit(color) => modifier.push_str(&format!("48;5;{}", color)),
                Color::Grayscale(brightness) => modifier.push_str(&format!("48;5;{}", brightness)),
                Color::Truecolor(red, green, blue) => {
                    modifier.push_str(&format!("48;2;{};{};{}", red, green, blue))
                }
            }

            self.c_background = glyph.background;
        };
        self.chars_sent += modifier.len();
        if self.chars_sent >= self.chars_refresh {
            self.chars_sent = 0;
            set_plain_on_exit = true;
        }
        if set_plain_on_exit {
            self.c_bright = true;
            self.c_dim = true;
            self.c_italic = true;
            self.c_underline = true;
            self.c_blink = true;
            self.c_blink_fast = true;
            self.c_reverse = true;
            self.c_transparent = true;
            self.c_strike = true;
            self.c_color = Color::white();
            self.c_background = Color::black();
        }
        modifier
    }

    /// Get a string of a given glyph to print on screen.
    fn gformat(&mut self, glyph: Glyph) -> String {
        let mut modifier = String::new(); //"\x1b[".to_string();
                                          // let mut add_modifier = false;
                                          // Plain = 0,
                                          // if self.c_plain && !glyph.plain {
                                          //     self.c_plain = false;
                                          // } else if !self.c_plain && glyph.plain {
                                          //     self.c_plain = true;
                                          //     modifier.push_str("0;");
                                          // }
        if !glyph.bright {
            if self.c_bright {
                self.c_bright = false;
                modifier.push_str("22;");
            }
        } else {
            if self.c_dim {
                self.c_dim = false;
            }
            if !self.c_bright {
                self.c_bright = true;
                modifier.push_str("1;");
            }
        }
        if !glyph.dim {
            if self.c_dim {
                self.c_dim = false;
                modifier.push_str("22;");
            }
        } else {
            if self.c_bright {
                self.c_bright = false;
            }
            if !self.c_dim {
                // println!("<1234");
                self.c_dim = true;
                modifier.push_str("2;");
            }
        }
        if !self.c_dim && !self.c_bright {
            modifier.push_str("22;");
        }
        if !glyph.italic {
            // self.c_italic = false;
            modifier.push_str("23;");
        } else {
            //self.c_italic = true;
            modifier.push_str("3;");
        }
        if !glyph.underline {
            // self.c_underline = false;
            modifier.push_str("24;");
        } else {
            //self.c_underline = true;
            modifier.push_str("4;");
        }
        if !glyph.blink {
            self.c_blink = false;
        } else {
            //if !self.c_blink {
            modifier.push_str("5;");
            self.c_blink = true;
            //}
        }
        if !glyph.blink_fast {
            self.c_blink_fast = false;
        } else {
            //if !self.c_blink_fast {
            self.c_blink_fast = true;
            modifier.push_str("6;");
            //            }
        }
        if !self.c_blink && !self.c_blink_fast {
            modifier.push_str("25;");
        }
        if !glyph.reverse {
            // self.c_reverse = false;
            modifier.push_str("27;");
        } else {
            //self.c_reverse = true;
            modifier.push_str("7;");
        }
        if !glyph.strike {
            // self.c_strike = false;
            modifier.push_str("29;");
        } else {
            //self.c_strike = true;
            modifier.push_str("9;");
        }
        match glyph.color {
            Color::Basic(color) => modifier.push_str(&format!("3{};", color as u8)),
            Color::EightBit(color) => modifier.push_str(&format!("38;5;{};", color)),
            Color::Grayscale(brightness) => modifier.push_str(&format!("38;5;{};", brightness)),
            Color::Truecolor(red, green, blue) => {
                modifier.push_str(&format!("38;2;{};{};{};", red, green, blue))
            }
        }
        match glyph.background {
            Color::Basic(color) => modifier.push_str(&format!("4{}", color as u8)),
            Color::EightBit(color) => modifier.push_str(&format!("48;5;{}", color)),
            Color::Grayscale(brightness) => modifier.push_str(&format!("48;5;{}", brightness)),
            Color::Truecolor(red, green, blue) => {
                modifier.push_str(&format!("48;2;{};{};{}", red, green, blue))
            }
        }

        modifier
    }

    /// Initialize required parameters for library to work as expected.
    pub fn initialize(&mut self) {
        self.termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(self.stdin, TCSANOW, &self.termios)
            .expect("Failed setting modified Termios buffer during initialization.");
        print!("\x1b[?1049h"); // use separate buffer
        print!("\x1b[2J"); // clear screen
        print!("\x1b[?25l"); // disable cursor
    }

    /// Restore original settings of users terminal.
    pub fn cleanup(self) {
        print!("\x1b[?25h"); // enable cursor
        print!("\x1b[2J"); // clear screen
        print!("\x1b[?1049l"); // disable separate buffer
        tcsetattr(self.stdin, TCSANOW, &self.termios_orig)
            .expect("Failed to restore original Termios buffer."); // reset the stdin to
                                                                   // original termios data
    }
}
