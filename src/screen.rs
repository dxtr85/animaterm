use super::animation::Animation;
use super::color::Color;
use super::display::Display;
use super::graphic::Graphic;
use super::helpers::ask_os_for_rows_and_cols;
use super::pixel::Pixel;
use super::time::Timestamp;
use super::Glyph;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::mem::replace;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

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
    shelve: HashMap<
        usize,
        (
            Display,
            // HashMap<usize, (Animation, usize, (usize, usize))>,
            HashMap<usize, (Graphic, usize, (usize, usize))>,
        ),
    >,
    shelve_id: usize,
    time: Timestamp,
    next_available_id: usize,
    // animations: HashMap<usize, (Animation, usize, (usize, usize))>,
    graphics: HashMap<usize, (Graphic, usize, (usize, usize))>,
    stdin: i32,
    stdout: io::Stdout,
    termios_orig: Termios,
    termios: Termios,
    c_color: Color,
    c_background: Color,
    c_x: usize,
    c_y: usize,
    c_plain: bool,
    c_bright: bool,
    c_dim: bool,
    c_italic: bool,
    c_underline: bool,
    c_blink: bool,
    c_blink_fast: bool,
    c_reverse: bool,
    c_transparent: bool,
    c_strike: bool,
}

impl Screen {
    pub fn new(cols: Option<usize>, rows: Option<usize>, glyph: Option<Glyph>) -> Self {
        let (new_rows, new_cols) = ask_os_for_rows_and_cols();
        let final_rows = if rows.is_none() {
            new_rows
        } else {
            rows.unwrap()
        };
        let final_cols = if cols.is_none() {
            new_cols
        } else {
            cols.unwrap()
        };
        let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work
                       // on /dev/stdin or /dev/tty
        let termios = Termios::from_fd(stdin).expect("Could not get Termios instance from stdin.");
        let new_termios = termios.clone(); // make a mutable copy of termios
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
            c_plain: dglyph.plain,
            c_bright: dglyph.bright,
            c_dim: dglyph.dim,
            c_italic: dglyph.italic,
            c_blink: dglyph.blink,
            c_blink_fast: dglyph.blink_fast,
            c_underline: dglyph.underline,
            c_reverse: dglyph.reverse,
            c_transparent: dglyph.transparent,
            c_strike: dglyph.strike,
        }
    }

    pub fn new_display(&mut self, display_id: usize, keep_existing: bool) -> usize {
        let new_display = Display::new(display_id, Glyph::default(), self.cols, self.rows);
        //let mut return_id = None;
        let old_display = replace(&mut self.display, new_display);
        //let current_time = self.time.tick();
        let old_graphics = self.graphics.drain().collect();
        if keep_existing {
            self.shelve
                .insert(old_display.id, (old_display, old_graphics));
            //return_id = Some(self.shelve_id);
            self.shelve_id += 1;
        }
        self.cls();
        display_id
    }

    pub fn restore_display(&mut self, display_id: usize, keep_existing: bool) -> Option<usize> {
        let mut return_id = None;
        if let Some((shelved_display, shelved_graphics)) = self.shelve.remove(&display_id) {
            return_id = Some(display_id);
            //let current_time = self.time.tick();
            let old_display = replace(&mut self.display, shelved_display);
            let old_graphics = replace(&mut self.graphics, shelved_graphics);
            if keep_existing {
                self.shelve
                    .insert(old_display.id, (old_display, old_graphics));
            }
            let to_print = self.refresh(true);
            self.print_all(to_print);
        }
        return_id
    }

    pub fn add_graphic(&mut self, g: Graphic, layer: usize, offset: (usize, usize)) -> usize {
        let graph_id = self.next_available_id;
        self.next_available_id += 1;
        self.graphics.insert(graph_id, (g, layer, offset));
        graph_id
    }

    pub fn move_graphic(&mut self, gid: usize, layer: usize, offset: (isize, isize)) {
        let mut cl_args = Vec::new();
        let mut new_offset = None;
        let mut frame_id = None;
        if let Some(current_state) = self.graphics.get_mut(&gid) {
            frame_id = Some(current_state.0.current_frame);
            let delta_zero = if offset.0 < 0 {
                (current_state.2 .0).saturating_sub((offset.0).unsigned_abs() as usize)
            } else {
                (current_state.2 .0) + (offset.0 as usize)
            };
            let delta_one = if offset.1 < 0 {
                (current_state.2 .1).saturating_sub((offset.1).unsigned_abs() as usize)
            } else {
                (current_state.2 .1) + (offset.1 as usize)
            };
            new_offset = Some((delta_zero, delta_one));
            if current_state.1 != layer {
                cl_args.push((
                    current_state.1,
                    current_state.2 .0,
                    current_state.2 .1,
                    current_state.0.cols,
                    current_state.0.rows,
                ));
            }
            if offset.0 < 0 {
                cl_args.push((
                    layer,
                    delta_zero + current_state.0.cols,
                    current_state.2 .1,
                    (offset.0).abs() as usize,
                    current_state.0.rows,
                ));
            } else if offset.0 > 0 {
                cl_args.push((
                    layer,
                    current_state.2 .0,
                    current_state.2 .1,
                    offset.0 as usize,
                    current_state.0.rows,
                ));
            }
            if offset.1 < 0 {
                cl_args.push((
                    layer,
                    current_state.2 .0,
                    delta_one + current_state.0.rows,
                    current_state.0.cols,
                    (offset.1).abs() as usize,
                ));
            } else if offset.1 > 0 {
                cl_args.push((
                    layer,
                    current_state.2 .0,
                    current_state.2 .1,
                    current_state.0.cols,
                    offset.1 as usize,
                ));
            }
        }
        for c in cl_args {
            self.cla(c.0, c.1, c.2, c.3, c.4);
        }
        if let Some(no) = new_offset {
            self.update_graphics_layer(gid, layer);
            self.update_graphics_offset(gid, no);
        }
        if let Some(fid) = frame_id {
            self.set_graphic((&gid, &fid), false);
            if let Some((graphic, layer, offset)) = self.graphics.get(&gid) {
                let pixels = graphic.get(*offset);
                self.update(vec![(pixels, *layer)]);
            }
        }
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    pub fn delete_graphic(&mut self, id: &usize) {
        let mut clear_info = None;
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(id) {
            clear_info = Some((*layer, offset.0, offset.1, graphic.cols, graphic.rows));
        }
        if let Some(c) = clear_info {
            self.cla(c.0, c.1, c.2, c.3, c.4);
        }
    }

    pub fn set_graphic(&mut self, ids: (&usize, &usize), force: bool) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(ids.0) {
            results.push((graphic.set_frame(ids.1, *offset, force), *layer));
        }
        self.update(results);
        let to_print = self.refresh(force);
        self.print_all(to_print);
    }

    pub fn print_screen(&mut self) -> Vec<String> {
        let to_print = self.refresh(true);
        self.get_text(to_print)
    }

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

    pub fn print_graphic(&mut self, gid: usize, skip_border: bool) -> Vec<String> {
        let mut to_print = Vec::new();
        if let Some((graphic, _layer, _offset)) = self.graphics.get(&gid) {
            let mut pixels = graphic.get((0, 0));
            if skip_border {
                let max_c = graphic.cols - 1;
                let max_r = graphic.rows;
                pixels = pixels
                    .into_iter()
                    .filter(|p| p.x > 0 && p.x < max_c && p.y > 0 && p.y < max_r)
                    .collect();
            }
            for p in pixels {
                to_print.push((p.x, p.y, p.g));
            }
        }
        self.get_text(to_print)
    }

    pub fn set_graphic_color(&mut self, gid: usize, color: Color) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(&gid) {
            graphic.set_current_frame_color(color);
            let curr_frame = graphic.current_frame;
            results.push((graphic.set_frame(&curr_frame, *offset, true), *layer));
        }
        self.update(results);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    pub fn set_graphic_background(&mut self, gid: usize, color: Color) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(&gid) {
            graphic.set_current_frame_background(color);
            let curr_frame = graphic.current_frame;
            results.push((graphic.set_frame(&curr_frame, *offset, true), *layer));
        }
        self.update(results);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    pub fn set_graphic_style(&mut self, gid: usize, style: Glyph) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(&gid) {
            graphic.set_current_frame_style(style);
            let curr_frame = graphic.current_frame;
            results.push((graphic.set_frame(&curr_frame, *offset, true), *layer));
        }
        self.update(results);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    pub fn set_invisible(&mut self, gid: usize, invisible: bool) {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&gid) {
            let pixels = graphic.set_invisible(invisible, offset);
            let fid = graphic.current_frame;
            self.graphics.insert(gid, (graphic, layer, offset));
            self.set_graphic((&gid, &fid), false);
            self.update(vec![(pixels, layer)]);
            let to_print = self.refresh(false);
            self.print_all(to_print);
        }
    }
    pub fn set_glyph(&mut self, gid: usize, glyph: Glyph, col: usize, row: usize) {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&gid) {
            let pixel = graphic.set_glyph(glyph, col, row, offset);
            let fid = graphic.current_frame;
            self.graphics.insert(gid, (graphic, layer, offset));
            self.set_graphic((&gid, &fid), false);
            self.update(vec![(pixel, layer)]);
            let to_print = self.refresh(false);
            self.print_all(to_print);
        }
    }

    pub fn get_glyph(&mut self, gid: usize, col: usize, row: usize) -> Option<Glyph> {
        if let Some((gr, _l, _o)) = self.graphics.get(&gid) {
            return gr.get_glyph(col, row);
        }
        None
    }
    pub fn empty_frame(&mut self, gid: usize) -> Option<usize> {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&gid) {
            let result = graphic.empty_frame();
            self.graphics.insert(gid, (graphic, layer, offset));
            if result.is_some() {
                self.set_graphic((&gid, &result.unwrap()), true);
            }
            return result;
        }
        None
    }

    pub fn clone_frame(&mut self, gid: usize, frame_id: Option<usize>) -> Option<usize> {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&gid) {
            let mut fr_id = graphic.current_frame;
            if let Some(id) = frame_id {
                fr_id = id;
            }
            let result = graphic.clone_frame(fr_id);
            self.graphics.insert(gid, (graphic, layer, offset));
            if result.is_some() {
                self.set_graphic((&gid, &result.unwrap()), true);
            }
            return result;
        }
        None
    }

    pub fn update_graphics_layer(&mut self, gid: usize, layer: usize) {
        if let Some((graphic, _layer, offset)) = self.graphics.remove(&gid) {
            self.graphics.insert(gid, (graphic, layer, offset));
        }
    }
    pub fn update_graphics_offset(&mut self, gid: usize, offset: (usize, usize)) {
        if let Some((graphic, layer, _offset)) = self.graphics.remove(&gid) {
            self.graphics.insert(gid, (graphic, layer, offset));
        }
    }
    pub fn add_animation(&mut self, graphic_id: usize, a: Animation) -> Option<usize> {
        if let Some((mut graphic, layer, offset)) = self.graphics.remove(&graphic_id) {
            let add_result = graphic.add_animation(a);
            self.graphics.insert(graphic_id, (graphic, layer, offset));
            add_result
        } else {
            None
        }
    }

    pub fn start_animation(&mut self, gid: &usize, aid: usize) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(gid) {
            graphic.start_animation(aid, self.time.tick());
        }
    }

    pub fn enqueue_animation(&mut self, gid: &usize, aid: usize, when: Timestamp) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(gid) {
            graphic.enqueue_animation(aid, self.time.tick() + when);
        }
    }

    // TODO: rewrite this
    pub fn restart_animation(&mut self, gid: usize, aid: usize, when: Timestamp) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(&gid) {
            graphic.restart_animation(aid, when);
        }
    }

    pub fn pause_animation(&mut self, gid: usize) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(&gid) {
            if let Some(anim_id) = graphic.running_anim {
                graphic.pause_animation(anim_id, Timestamp::now());
            }
        }
    }

    pub fn pause_animation_on_frame(&mut self, gid: &usize, fid: usize) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(gid) {
            if let Some(anim_id) = graphic.running_anim {
                graphic.pause_animation_on_frame(anim_id, fid);
            }
            //anim.pause_on_frame(fid);
        }
    }

    pub fn stop_animation(&mut self, gid: &usize) {
        if let Some((graphic, _layer, _offset)) = self.graphics.get_mut(gid) {
            graphic.stop_animation();
        }
    }

    pub fn update_graphics(&mut self) {
        let mut pixels = vec![];
        for (_id, (graphic, layer, offset)) in &mut self.graphics {
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
                    if let Some((frame_id, running)) = anim.new_update(self.time.tick()) {
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

    pub fn cls(&mut self) {
        print!("\x1b[0m\x1b[21;22;23;24;25;26;27;29;37;40m\x1b[1;1Hm");
        println!("\x1b[H{:<1$}", "", self.cols * self.rows);
        self.flush_out();
    }

    pub fn cla(
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
        let mut to_update = Vec::with_capacity(1);
        to_update.push((to_clear, layer));
        self.update(to_update);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

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
        // let ile = to_print.len();
        // if ile > 0 {
        //     println!("To refresh: {}", ile);
        // }
        to_print
    }

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

    pub fn get_text(&mut self, glyphs: Vec<(usize, usize, Glyph)>) -> Vec<String> {
        let mut result = Vec::with_capacity(self.rows);
        let mut line_text = String::new();
        let mut last_line = 10;
        for (_x, y, glyph) in glyphs {
            if y != last_line {
                if line_text.len() > 0 {
                    result.push(line_text);
                    line_text = String::new();
                }
                last_line = y;
            }
            let modifier = self.gformat_old(glyph);
            if !modifier.is_empty() {
                line_text.push_str(&format!("\x1b[{}m{}", modifier, glyph.character));
                // line_text.push_str(&modifier);
                // line_text.push('m');
            } else {
                line_text.push(glyph.character);
            }
        }
        if result.len() > 0 {
            let mut last = result.pop().unwrap();
            last.push_str("\x1b[0m");
            result.push(last);
        }
        result
    }

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

    fn gformat_old(&mut self, glyph: Glyph) -> String {
        let mut modifier = String::new(); //"\x1b[".to_string();
                                          // let mut add_modifier = false;
                                          // Plain = 0,
        if self.c_plain && !glyph.plain {
            self.c_plain = false;
        } else if !self.c_plain && glyph.plain {
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
            modifier.push_str("21;");
            // modifier.push_str("01;");
        } else if !self.c_bright && glyph.bright {
            self.c_bright = true;
            modifier.push_str("1;");
        }
        if self.c_dim && !glyph.dim {
            self.c_dim = false;
            self.c_bright = false;
            modifier.push_str("22;");
        } else if !self.c_dim && glyph.dim {
            self.c_dim = true;
            self.c_bright = false;
            modifier.push_str("2;");
        }
        if self.c_italic && !glyph.italic {
            self.c_italic = false;
            modifier.push_str("23;");
        } else if !self.c_italic && glyph.italic {
            self.c_italic = true;
            modifier.push_str("3;");
        }
        if self.c_underline && !glyph.underline {
            self.c_underline = false;
            modifier.push_str("24;");
        } else if !self.c_underline && glyph.underline {
            self.c_underline = true;
            modifier.push_str("4;");
        }
        if self.c_blink && !glyph.blink {
            self.c_blink = false;
        } else if !self.c_blink && glyph.blink {
            self.c_blink = true;
            modifier.push_str("5;");
        }
        if self.c_blink_fast && !glyph.blink_fast {
            self.c_blink_fast = false;
        } else if !self.c_blink_fast && glyph.blink_fast {
            self.c_blink_fast = true;
            modifier.push_str("6;");
        }
        if !self.c_blink && !self.c_blink_fast {
            modifier.push_str("25;");
        }
        if self.c_reverse && !glyph.reverse {
            self.c_reverse = false;
            modifier.push_str("27;");
        } else if !self.c_reverse && glyph.reverse {
            self.c_reverse = true;
            modifier.push_str("7;");
        }
        if self.c_transparent && !glyph.transparent {
            self.c_transparent = false;
            modifier.push_str("28;");
        } else if !self.c_transparent && glyph.transparent {
            self.c_transparent = true;
            modifier.push_str("8;");
        }
        if self.c_strike && !glyph.strike {
            self.c_strike = false;
            modifier.push_str("29;");
        } else if !self.c_strike && glyph.strike {
            self.c_strike = true;
            modifier.push_str("9;");
        }
        if self.c_color != glyph.color {
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
        if self.c_background != glyph.background {
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
        modifier
    }

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
            }
            // modifier.push_str("01;");
        } else {
            //if !self.c_bright {
            self.c_bright = true;
            self.c_dim = false;
            modifier.push_str("1;");
            //}
        }
        if !glyph.dim {
            if self.c_dim {
                self.c_dim = false;
                //modifier.push_str("2;");
            }
            // modifier.push_str("01;");
        } else {
            //if !self.c_dim {
            self.c_dim = true;
            self.c_bright = false;
            modifier.push_str("2;");
            //}
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
        //self.c_color = glyph.color;
        //if self.c_background != glyph.background {
        //modifier.push_str(&format!("4{}", glyph.background as u8));
        match glyph.background {
            Color::Basic(color) => modifier.push_str(&format!("4{}", color as u8)),
            Color::EightBit(color) => modifier.push_str(&format!("48;5;{}", color)),
            Color::Grayscale(brightness) => modifier.push_str(&format!("48;5;{}", brightness)),
            Color::Truecolor(red, green, blue) => {
                modifier.push_str(&format!("48;2;{};{};{}", red, green, blue))
            }
        }

        //self.c_background = glyph.background;
        //};
        modifier
    }

    pub fn initialize(&mut self) {
        self.termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(self.stdin, TCSANOW, &mut self.termios)
            .expect("Failed setting modified Termios buffer during initialization.");
        print!("\x1b[?1049h"); // use separate buffer
        print!("\x1b[2J"); // clear screen
        print!("\x1b[?25l"); // disable cursor
    }

    pub fn cleanup(self) {
        print!("\x1b[?25h"); // enable cursor
        print!("\x1b[2J"); // clear screen
        print!("\x1b[?1049l"); // disable separate buffer
        tcsetattr(self.stdin, TCSANOW, &self.termios_orig)
            .expect("Failed to restore original Termios buffer."); // reset the stdin to
                                                                   // original termios data
    }
}
