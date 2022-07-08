use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::io;
use std::io::Write;
use std::mem::replace;
use std::ops::{Add, AddAssign, Sub};
use std::process::Command;
use std::time::Instant;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

pub struct Manager {
    screen: Screen,
}

impl Manager {
    pub fn new(cols: Option<usize>, rows: Option<usize>, glyph: Option<Glyph>) -> Self {
        let mut screen = Screen::new(cols, rows, glyph);
        screen.initialize();
        screen.cls();

        Manager { screen }
    }

    pub fn add_animation() {}
    pub fn start_animation(anim_id: usize) {}
    pub fn pause_animation(anim_id: usize) {}
    pub fn stop_animation(anim_id: usize) {}
    pub fn restart_animation(anim_id: usize) {}
    fn update_animations() {}

    pub fn cls() {}
    pub fn cla() {}
    pub fn new_display(keep_existing: bool) {}
    pub fn restore_display(display_id: usize, keep_existing: bool) {}

    pub fn add_graphic(gr: usize, layer: usize, offset: (usize, usize)) {}
    pub fn set_graphic(ids: (usize, usize)) {}
    pub fn delete_graphic(gid: usize) {}

    pub fn terminate(self) {
        self.screen.cleanup();
    }
}

pub struct Screen {
    pub rows: usize,
    pub cols: usize,
    // display should support transparent Glyphs inside Animations
    // should be Vec containing a structure with layered Glyphs occupying given pixel
    // last non-transparent Glyph for each pixel should be returned.
    // it should be known which pixels on what layer belong to given id of animation
    // it should be possible to move animation from one layer to another
    // a flag on every pixel should notify if a given pixel should be printed
    pub display: Vec<GlyphCake>,
    shelve: HashMap<
        //mgr
        usize,
        (
            Vec<GlyphCake>,
            HashMap<usize, (Animation, usize, (usize, usize))>,
            HashMap<usize, (Graphic, usize, (usize, usize))>,
        ),
    >,
    shelve_id: usize,                                               //mgr
    time: Timestamp,                                                //mgr
    next_anim_id: usize,                                            //mgr
    animations: HashMap<usize, (Animation, usize, (usize, usize))>, //mgr
    graphics: HashMap<usize, (Graphic, usize, (usize, usize))>,     //mgr
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
        let termios = Termios::from_fd(stdin).unwrap();
        let new_termios = termios.clone(); // make a mutable copy of termios
                                           // that we will modify
        let c_x = final_cols;
        let c_y = final_rows;
        let mut dglyph = Glyph::default();
        if glyph.is_some() {
            dglyph = glyph.unwrap();
        }
        let mut display = Vec::with_capacity(final_cols * final_rows);
        for j in 1..final_rows + 1 {
            for i in 1..final_cols + 1 {
                display.push(GlyphCake::new(i, j, Some(dglyph.clone()), 0));
            }
        }
        Screen {
            rows: final_rows,
            cols: final_cols,
            display,
            shelve: HashMap::new(),
            shelve_id: 0,
            time: Timestamp::now(),
            next_anim_id: 0,
            animations: HashMap::with_capacity(5),
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
            c_strike: dglyph.strike,
        }
    }
    // TODO: Functionality to re/store animations and their states
    pub fn new_display(mut self, keep_existing: bool) -> (Screen, Option<usize>) {
        let mut new_display = Vec::with_capacity(self.cols * self.rows);
        let dglyph = Glyph::default();
        for j in 1..self.rows + 1 {
            for i in 1..self.cols + 1 {
                new_display.push(GlyphCake::new(i, j, Some(dglyph.clone()), 0));
            }
        }
        let mut return_id = None;
        if keep_existing {
            let old_display = self.display;
            let current_time = self.time.tick();
            self.animations
                .values_mut()
                .for_each(|(a, _l, (_x, _y))| a.pause(current_time));
            let old_animations = self.animations.drain().collect();
            let old_graphics = self.graphics.drain().collect();
            self.shelve
                .insert(self.shelve_id, (old_display, old_animations, old_graphics));
            return_id = Some(self.shelve_id);
            self.shelve_id += 1;
        }
        self.display = new_display;
        self.cls();
        (self, return_id)
    }
    pub fn restore_display(
        mut self,
        display_id: usize,
        keep_existing: bool,
    ) -> (Screen, Option<usize>) {
        let (shelved_display, mut shelved_animations, shelved_graphics) = self
            .shelve
            .remove(&display_id)
            .expect(&format!("No display with id {} is shelved", display_id));
        let current_time = self.time.tick();
        shelved_animations
            .values_mut()
            .for_each(|(a, _l, (_x, _y))| a.start(current_time));
        let mut return_id = None;
        if keep_existing {
            self.shelve
                .insert(display_id, (self.display, self.animations, self.graphics));
            return_id = Some(display_id);
        }
        self.display = shelved_display;
        self.animations = shelved_animations;
        self.graphics = shelved_graphics;
        let to_print = self.refresh(true);
        self.print_all(to_print);
        (self, return_id)
    }

    pub fn add_graphic(&mut self, g: Graphic, layer: usize, offset: (usize, usize)) -> usize {
        let graph_id = self.next_anim_id;
        self.next_anim_id += 1;
        self.graphics.insert(graph_id, (g, layer, offset));
        graph_id
    }

    pub fn delete_graphic(&mut self, id: &usize) {
        let mut clear_info = None;
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(id) {
            clear_info = Some((*layer, offset.0, offset.1, graphic.cols, graphic.rows));
            //            results.push((graphic.set_graphic(&0, *offset, true), *layer));
        }
        if let Some(c) = clear_info {
            self.cla(c.0, c.1, c.2, c.3, c.4);
        }
        // let skipped = self.update(results);
        // self.cls();
        // println!("Skipped: {}", skipped);
        // let to_print = self.refresh(false);
        // self.print_all(to_print);
    }

    pub fn set_graphic(&mut self, ids: (&usize, &usize)) {
        let mut results = Vec::new();
        if let Some((graphic, layer, offset)) = self.graphics.get_mut(ids.0) {
            results.push((graphic.set_graphic(ids.1, *offset, false), *layer));
        }
        self.update(results);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    pub fn add_animation(&mut self, a: Animation, layer: usize, offset: (usize, usize)) -> usize {
        let anim_id = self.next_anim_id;
        self.next_anim_id += 1;
        self.animations.insert(anim_id, (a, layer, offset));
        anim_id
    }

    pub fn start_animation(&mut self, id: &usize) {
        if let Some((anim, _layer, _offset)) = self.animations.get_mut(id) {
            anim.start(self.time.tick());
        }
    }

    pub fn restart_animation(&mut self, id: &usize) {
        if let Some((anim, _layer, _offset)) = self.animations.get_mut(id) {
            anim.restart(self.time.tick());
        }
    }

    pub fn pause_animation(&mut self, id: &usize) {
        if let Some((anim, _layer, _offset)) = self.animations.get_mut(id) {
            anim.pause(self.time.tick());
        }
    }

    pub fn stop_animation(&mut self, id: &usize) {
        if let Some((anim, _layer, _offset)) = self.animations.get_mut(id) {
            anim.stop();
        }
    }

    pub fn update_animations(&mut self) {
        let mut pixels = vec![];
        for (_id, (anim, layer, off_set)) in &mut self.animations {
            if let Some(pxls) = anim.update(self.time.tick()) {
                let mut n_ps = Vec::with_capacity(pxls.len());
                for mut p in pxls {
                    p.offset(*off_set);
                    n_ps.push(p);
                }
                pixels.push((n_ps, layer.clone()));
            }
        }
        //        for (ps, layer) in pixels {
        //            self.update(ps, *layer);
        //        }
        self.update(pixels);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    pub fn cls(&mut self) {
        self.stdout.lock().flush().unwrap();
        // println!(
        //     "\x1b[37;40mColor: {} Bg: {}",
        //     self.c_color as u8, self.c_background as u8
        // );
        print!("\x1b[0m\x1b[21;22;23;24;25;26;27;29;37;40m\x1b[1;1Hm");
        //for _i in 1..self.rows + 1 {
        println!("\x1b[H{:<1$}", "", self.cols * self.rows);
        //}
        // print!("\x1b[H\x1b[37;40m{:<1$}", "", self.cols * self.rows);
        self.stdout.lock().flush().unwrap();
        self.c_background = Color::Black;
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
        let gplain = Glyph::plain();
        for x in start_x..start_x + width + 1 {
            for y in start_y..start_y + height + 1 {
                to_clear.push(Pixel::new(x, y, true, gplain));
            }
        }
        let mut to_update = Vec::with_capacity(1);
        to_update.push((to_clear, layer));
        self.update(to_update);
        let to_print = self.refresh(false);
        self.print_all(to_print);
    }

    pub fn refresh(&mut self, force: bool) -> Vec<(usize, usize, Glyph)> {
        let mut to_print = Vec::with_capacity(64);
        for gcake in self.display.iter_mut() {
            if gcake.modified || force {
                //println!("Adding glyph {} {}", gcake.col, gcake.row);
                to_print.push((gcake.col, gcake.row, gcake.get_glyph()));
            }
        }
        to_print
    }

    pub fn rectangle(
        &mut self,
        glyph: Glyph,
        start_x: usize,
        start_y: usize,
        width: usize,
        lenght: usize,
    ) -> Vec<Pixel> {
        let mut rectangle = Vec::new();
        for j in start_y..start_y + lenght {
            for i in start_x..start_x + width {
                rectangle.push(Pixel::new(i, j, true, glyph.clone()));
            }
        }
        rectangle
    }

    pub fn print_all(&mut self, glyphs: Vec<(usize, usize, Glyph)>) {
        for (x, y, g) in glyphs {
            self.print(x, y, g);
        }
        self.stdout.lock().flush().unwrap();
    }

    pub fn update(&mut self, pixels: Vec<(Vec<Pixel>, usize)>) {
        //let mut skipped = 0;
        //self.cls();
        //println!("Deleting {}", pixels.get(0).unwrap().0.len());

        for (ps, layer) in pixels {
            for p in ps {
                if p.x > self.cols || p.y > self.rows {
                    // skipped += 1;
                    continue;
                }
                let x = p.x.saturating_sub(1);
                let y = p.y.saturating_sub(1);
                let index = x + (y * self.cols);
                let cake = self.display.get_mut(index).expect("WTF?!");
                cake.update(p.g, layer);
            }
        }
        // if skipped > 0 {
        //     println!("Skipped: {}", skipped);
        // }
        // skipped
    }

    pub fn print(&mut self, x: usize, y: usize, glyph: Glyph) {
        if x > self.cols || y > self.rows {
            return;
        }
        let mut formated = String::new();
        if self.c_x != x || self.c_y != y {
            formated.push_str(&format!("\x1b[{};{}H", y, x));
            self.c_x = x + 1;
            self.c_y = y;
        };
        let modifier = self.gformat(glyph);
        if !modifier.is_empty() {
            formated.push_str("\x1b[");
            formated.push_str(&modifier);
            formated.push('m');
        }

        print!("{}{}", formated, glyph.character);
    }

    fn gformat(&mut self, glyph: Glyph) -> String {
        let mut modifier = String::new(); //"\x1b[".to_string();
                                          // let mut add_modifier = false;
                                          // Plain = 0,
        if self.c_plain && !glyph.plain {
            self.c_plain = false;
        } else if !self.c_plain && glyph.plain {
            self.c_plain = true;
            modifier.push_str("0;");
        }
        if self.c_bright && !glyph.bright {
            self.c_bright = false;
            modifier.push_str("01;");
        } else if !self.c_bright && glyph.bright {
            self.c_bright = true;
            modifier.push_str("1;");
        }
        if self.c_dim && !glyph.dim {
            self.c_dim = false;
            modifier.push_str("22;");
        } else if !self.c_dim && glyph.dim {
            self.c_dim = true;
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
            modifier.push_str("25;");
        } else if !self.c_blink && glyph.blink {
            self.c_blink = true;
            modifier.push_str("5;");
        }
        if self.c_blink_fast && !glyph.blink_fast {
            self.c_blink_fast = false;
            modifier.push_str("26;");
        } else if !self.c_blink_fast && glyph.blink_fast {
            self.c_blink_fast = true;
            modifier.push_str("6;");
        }
        if self.c_reverse && !glyph.reverse {
            self.c_reverse = false;
            modifier.push_str("27;");
        } else if !self.c_reverse && glyph.reverse {
            self.c_reverse = true;
            modifier.push_str("7;");
        }
        if self.c_strike && !glyph.strike {
            self.c_strike = false;
            modifier.push_str("29;");
        } else if !self.c_strike && glyph.strike {
            self.c_strike = true;
            modifier.push_str("9;");
        }
        if self.c_color != glyph.color {
            modifier.push_str(&format!("3{};", glyph.color as u8));
            self.c_color = glyph.color;
        };
        //if self.c_background != glyph.background {
        modifier.push_str(&format!("4{}", glyph.background as u8));
        self.c_background = glyph.background;
        //};
        modifier
    }

    pub fn initialize(&mut self) {
        self.termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(self.stdin, TCSANOW, &mut self.termios).unwrap();
        print!("\x1b[?1049h"); // use separate buffer
        print!("\x1b[2J"); // clear screen
        print!("\x1b[?25l"); // disable cursor
    }

    pub fn cleanup(self) {
        print!("\x1b[?25h"); // enable cursor
        print!("\x1b[2J"); // clear screen
        print!("\x1b[?1049l"); // disable separate buffer
        tcsetattr(self.stdin, TCSANOW, &self.termios_orig).unwrap(); // reset the stdin to
                                                                     // original termios data
    }
}

#[derive(PartialOrd, Debug, Clone, Copy)]
pub struct Timestamp(u64, u32, Instant);
impl Timestamp {
    pub fn now() -> Self {
        Timestamp(0, 0, Instant::now())
    }
    pub fn tick(&mut self) -> Self {
        let now = Instant::now();
        let dif = now - self.2;
        *self += Timestamp(dif.as_secs(), dif.subsec_millis(), now);
        *self
    }
    pub fn new(sec: u64, msec: u32) -> Self {
        Timestamp(sec, msec, Instant::now())
    }
}
impl PartialEq for Timestamp {
    fn eq(&self, other: &Timestamp) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Add for Timestamp {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut next_one = self.1 + other.1;
        let next_zero = self.0 + other.0 + (next_one / 1000) as u64;
        next_one %= 1000;
        Self(next_zero, next_one, Instant::now())
    }
}

impl AddAssign for Timestamp {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}

impl Sub for Timestamp {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let my_msec = self.0 * 1000 + self.1 as u64;
        let other_msec = other.0 * 1000 + other.1 as u64;
        if other_msec > my_msec {
            return Self(0, 0, Instant::now());
        }
        let sub_msec = my_msec - other_msec;
        let next_one = (sub_msec % 1000) as u32;
        let next_zero = sub_msec / 1000;
        Self(next_zero, next_one, Instant::now())
    }
}

pub struct Graphic {
    rows: usize,
    cols: usize,
    next_lib_id: usize,
    glyphs: Vec<Glyph>,
    library: HashMap<usize, Vec<Glyph>>,
}

impl Graphic {
    pub fn new(
        cols: usize,
        rows: usize,
        glyphs: Option<Vec<Glyph>>,
        library: Option<HashMap<usize, Vec<Glyph>>>,
    ) -> Self {
        let g = if glyphs.is_some() {
            glyphs.unwrap()
        } else {
            vec![Glyph::default(); rows * cols]
        };
        let mut next_lib_id = 0;
        let l = if library.is_some() {
            let lib = library.unwrap();
            next_lib_id = lib.keys().max().unwrap() + 1;
            lib
        } else {
            HashMap::new()
        };
        Graphic {
            rows,
            cols,
            next_lib_id,
            glyphs: g,
            library: l,
        }
    }
    pub fn add_to_library(&mut self, item: Vec<Glyph>) -> Option<usize> {
        let mut result = None;
        if item.len() == self.rows * self.cols {
            self.library.insert(self.next_lib_id, item);
            result = Some(self.next_lib_id);
            self.next_lib_id += 1;
        }
        result
    }

    pub fn get(&self, offset: (usize, usize)) -> Vec<Pixel> {
        let mut result = Vec::with_capacity(self.rows * self.cols);
        for (i, glyph) in self.glyphs.iter().cloned().enumerate() {
            result.push(Pixel::new(
                1 + offset.0 + (i % self.cols),
                1 + offset.1 + (i / self.cols),
                true,
                glyph,
            ));
        }
        result
    }

    pub fn set_graphic(&mut self, id: &usize, offset: (usize, usize), empty: bool) -> Vec<Pixel> {
        if empty {
            let size = self.cols * self.rows;
            let gplain = Glyph::plain();
            let mut result = Vec::with_capacity(size);
            for i in 0..size {
                result.push(Pixel::new(
                    1 + offset.0 + (i % self.cols),
                    1 + offset.1 + (i / self.cols),
                    true,
                    gplain.clone(),
                ));
            }
            return result;
        }
        let mut changed = Vec::with_capacity(self.cols);
        if let Some(glyphs) = self.library.get(id) {
            for (i, glyph) in glyphs.into_iter().enumerate() {
                if *glyph != *self.glyphs.get(i).unwrap() {
                    self.glyphs.insert(i, glyph.clone());
                    changed.push(Pixel::new(
                        1 + offset.0 + (i % self.cols),
                        1 + offset.1 + (i / self.cols),
                        true,
                        glyph.clone(),
                    ));
                }
            }
        }
        changed
    }
}

pub struct Animation {
    current_frame: usize,
    next_frame: usize,
    running: bool,
    looping: bool,
    frames: HashMap<usize, Vec<Pixel>>,
    ordering: Vec<(usize, Timestamp)>,
    ord_max: usize,
    trigger_time: Timestamp,
}

impl Animation {
    pub fn new(
        frames: HashMap<usize, Vec<Pixel>>,
        running: bool,
        looping: bool,
        ordering: Vec<(usize, Timestamp)>,
        start_time: Timestamp,
    ) -> Animation {
        let ord_max = ordering.len() - 1;
        Animation {
            current_frame: 0,
            next_frame: 0,
            running,
            looping,
            frames,
            ordering,
            ord_max,
            trigger_time: start_time,
        }
    }

    pub fn start(&mut self, t: Timestamp) {
        self.trigger_time = t + self.trigger_time;
        self.running = true;
    }

    pub fn restart(&mut self, t: Timestamp) {
        self.trigger_time = t + self.trigger_time;
        // self.current_frame = 0;
        self.next_frame = self.ordering[0].0;
        self.running = true;
    }

    pub fn pause(&mut self, t: Timestamp) {
        self.trigger_time = self.trigger_time - t;
        self.running = false;
    }

    pub fn stop(&mut self) {
        self.trigger_time = Timestamp::now();
        self.current_frame = 0;
        self.next_frame = 0;
        self.running = false;
    }

    pub fn update(&mut self, dtime: Timestamp) -> Option<Vec<Pixel>> {
        if dtime >= self.trigger_time && self.running {
            let frame = self.frames.get(&self.current_frame).unwrap();
            let (current_frame, delta_time) = self.ordering[self.next_frame];
            self.current_frame = current_frame;
            self.trigger_time += delta_time;
            if self.next_frame == self.ord_max {
                if self.looping {
                    self.next_frame = 0;
                }
            } else {
                self.next_frame += 1;
            }
            Some(frame.to_vec().clone())
        } else {
            None
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    x: usize,
    y: usize,
    print: bool,
    g: Glyph,
}

impl Pixel {
    pub fn new(x: usize, y: usize, print: bool, g: Glyph) -> Pixel {
        Pixel { x, y, print, g }
    }
    pub fn set_xy(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
    pub fn set_print(&mut self, print: bool) {
        self.print = print;
    }
    pub fn offset(&mut self, offset: (usize, usize)) {
        self.x += offset.0;
        self.y += offset.1;
    }
}

pub struct GlyphCake {
    col: usize,
    row: usize,
    pub glyphs: Vec<Option<Glyph>>,
    top_layer: usize,
    modified: bool,
}

impl GlyphCake {
    pub fn new(col: usize, row: usize, glyph: Option<Glyph>, layer: usize) -> Self {
        let mut glyphs = vec![None; layer];
        let modified = false;
        glyphs.insert(layer, glyph);
        GlyphCake {
            col,
            row,
            glyphs,
            top_layer: layer,
            modified,
        }
    }

    pub fn update(&mut self, glyph: Glyph, layer: usize) {
        let what_to_insert = if glyph.plain { None } else { Some(glyph) };
        // println!("Inserting: {:?}", what_to_insert);
        if layer > self.top_layer {
            for i in self.top_layer + 1..layer + 1 {
                self.glyphs.insert(i, None);
            }
        }
        //println!("layer: {}, what: {:?}", layer, what_to_insert);
        let _old = replace(&mut self.glyphs[layer], what_to_insert);
        if layer >= self.top_layer {
            self.top_layer = layer;
            self.modified = true;
            if what_to_insert.is_none() {
                self.decrease_top_layer();
            }
        }
    }

    pub fn decrease_top_layer(&mut self) {
        let mut current_glyph = self.glyphs.get(self.top_layer).unwrap();
        while current_glyph.is_none() && self.top_layer > 0 {
            self.top_layer -= 1;
            self.glyphs.pop();
            current_glyph = self.glyphs.get(self.top_layer).unwrap();
        }
        if current_glyph.is_none() {
            self.glyphs.insert(0, Some(Glyph::plain()));
        }
    }

    pub fn get_glyph(&mut self) -> Glyph {
        self.modified = false;
        let glyph = self.glyphs.get(self.top_layer);
        if glyph.is_none() || glyph.unwrap().is_none() {
            return Glyph::plain();
        };

        glyph.expect("WHat?").expect("DaF..").clone()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Glyph {
    pub character: char,
    pub color: Color,
    pub background: Color,
    pub plain: bool,
    pub bright: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub blink_fast: bool,
    pub reverse: bool,
    pub strike: bool,
}

impl Glyph {
    pub fn new(
        character: char,
        color: Color,
        background: Color,
        plain: bool,
        bright: bool,
        dim: bool,
        italic: bool,
        underline: bool,
        blink: bool,
        blink_fast: bool,
        reverse: bool,
        strike: bool,
    ) -> Self {
        Glyph {
            character,
            color,
            background,
            plain,
            bright,
            dim,
            italic,
            underline,
            blink,
            blink_fast,
            reverse,
            strike,
        }
    }
    pub fn plain() -> Self {
        Glyph {
            character: ' ',
            color: Color::White,
            background: Color::Black,
            plain: true,
            bright: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            blink_fast: false,
            reverse: false,
            strike: false,
        }
    }
    pub fn set_char(&mut self, character: char) {
        self.character = character;
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }
    pub fn set_blink(&mut self, blink: bool) {
        self.blink = blink;
    }
}
impl Default for Glyph {
    fn default() -> Self {
        Glyph {
            character: ' ',
            color: Color::White,
            background: Color::Black,
            plain: false,
            bright: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            blink_fast: false,
            reverse: false,
            strike: false,
        }
    }
}

fn ask_os_for_rows_and_cols() -> (usize, usize) {
    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
        .collect();
    let rows = match Command::new("tput")
        .arg("lines")
        .env_clear()
        .envs(&filtered_env)
        .output()
    {
        Ok(data) => {
            let output = String::from_utf8(data.stdout).unwrap();
            let number = usize::from_str_radix(output.trim(), 10);
            match number {
                Ok(a_number) => a_number,
                Err(e) => {
                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to determine lines count from {}, using defaults\n{}", output, e);
                    35
                }
            }
        }
        Err(e) => {
            eprintln!(
                "\x1b[97;41;5mERR\x1b[m Unable to determine lines count, using defaults\n{:?}",
                e
            );
            35
        }
    };

    let cols = match Command::new("tput")
        .arg("cols")
        .env_clear()
        .envs(&filtered_env)
        .output()
    {
        Ok(data) => {
            let output = String::from_utf8(data.stdout).unwrap();
            let number = usize::from_str_radix(output.trim(), 10);
            match number {
                Ok(a_number) => a_number,
                Err(e) => {
                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to determine cols count from {}, using defaults\n{}", output, e);
                    80
                }
            }
        }
        Err(e) => {
            eprintln!(
                "\x1b[97;41;5mERR\x1b[m Unable to determine cols count, using defaults\n{:?}",
                e
            );
            80
        }
    };
    (rows, cols)
}

#[cfg(test)]
mod tests {
    use super::Glyph;
    use super::GlyphCake;
    use super::Timestamp;
    #[test]
    fn overflow_ms_to_sec() {
        let t0 = Timestamp::new(0, 0);
        let t1 = Timestamp::new(0, 1000);
        assert_eq!(t0 + t1, Timestamp::new(1, 0));
    }
    #[test]
    fn add_sec_to_sec() {
        let t0 = Timestamp::new(1, 500);
        let t1 = Timestamp::new(2, 700);
        assert_eq!(t0 + t1, Timestamp::new(4, 200));
    }
    #[test]
    fn sub_sec_from_sec() {
        let t0 = Timestamp::new(1, 500);
        let t1 = Timestamp::new(1, 400);
        assert_eq!(t0 - t1, Timestamp::new(0, 100));
    }
    #[test]
    fn no_neg_time() {
        let t0 = Timestamp::new(1, 500);
        let t1 = Timestamp::new(2, 400);
        assert_eq!(t0 - t1, Timestamp::new(0, 0));
    }
    #[test]
    fn glyph_cake() {
        let g1 = Glyph::new(
            '1',
            crate::Color::White,
            crate::Color::Red,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        );
        let g2 = Glyph::new(
            '2',
            crate::Color::Blue,
            crate::Color::Yellow,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        );
        let mut gc = GlyphCake::new(1, 1, Some(g1), 2);
        //println!("Start: {:?}", gc.glyphs);
        gc.update(g2, 10);
        //println!("Update1: {:?}", gc.glyphs);
        assert_eq!(gc.get_glyph().character, '2');
        gc.update(Glyph::plain(), 10);
        println!("Update2: {:?}", gc.glyphs);
        assert_eq!(gc.glyphs.len(), 3);
        assert_eq!(gc.get_glyph().character, '1');
    }
}

pub fn message_box(
    title: Option<String>,
    content: String,
    glyph: Glyph,
    start_x: usize,
    start_y: usize,
    width: usize,
    lenght: usize,
) -> Vec<Pixel> {
    let mut mbox = Vec::new();
    let mut cgl = glyph.clone();
    cgl.set_char('╭');
    mbox.push(Pixel::new(start_x, start_y, true, cgl.clone()));
    let mut i = 1;
    if let Some(name) = title {
        for c in name.chars() {
            if i > width.saturating_sub(2) {
                break;
            }
            cgl.set_char(c);
            mbox.push(Pixel::new(start_x + i, start_y, true, cgl.clone()));
            i += 1;
        }
    }
    cgl.set_char('─');
    for i in start_x + i..start_x + width - 1 {
        mbox.push(Pixel::new(i, start_y, true, cgl.clone()));
    }
    cgl.set_char('╮');
    mbox.push(Pixel::new(start_x + width - 1, start_y, true, cgl.clone()));
    let mut text = content.split_whitespace();
    let mut word = text.next();
    for j in start_y + 1..start_y + lenght - 1 {
        cgl.set_char('│');
        mbox.push(Pixel::new(start_x, j, true, cgl.clone()));
        i = 2;
        mbox.push(Pixel::new(start_x + 1, j, true, glyph.clone()));
        if let Some(mut content) = word {
            while content.len() < width.saturating_sub(i + 1) {
                for c in content.chars() {
                    cgl.set_char(c);
                    mbox.push(Pixel::new(start_x + i, j, true, cgl.clone()));
                    i += 1;
                }
                mbox.push(Pixel::new(start_x + i, j, true, glyph.clone()));
                i += 1;
                word = text.next();
                if let Some(help) = word {
                    content = help;
                } else {
                    content = "";
                }
            }
            for g in start_x + i..start_x + width - 1 {
                mbox.push(Pixel::new(g, j, true, glyph.clone()));
            }
        } else {
            for i in start_x + 1..start_x + width - 1 {
                mbox.push(Pixel::new(i, j, true, glyph.clone()));
            }
        }
        cgl.set_char('│');
        mbox.push(Pixel::new(start_x + width - 1, j, true, cgl.clone()));
    }
    cgl.set_char('╰');
    mbox.push(Pixel::new(start_x, start_y + lenght - 1, true, cgl.clone()));
    cgl.set_char('─');
    for i in start_x + 1..start_x + width - 1 {
        mbox.push(Pixel::new(i, start_y + lenght - 1, true, cgl.clone()));
    }
    cgl.set_char('╯');
    mbox.push(Pixel::new(
        start_x + width - 1,
        start_y + lenght - 1,
        true,
        cgl.clone(),
    ));
    mbox
}
