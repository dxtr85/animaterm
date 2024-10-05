use animaterm::{prelude::*, utilities::message_box};

fn main() {
    let mut mgr = Manager::new(true, None, None, None, None, None);
    let mut input = Input::new(&mut mgr);
    input.show(&mut mgr);
    loop {
        if let Some(ch) = mgr.read_char() {
            if let Some(key) = map_private_char_to_key(ch) {
                match key {
                    Key::Up => input.move_cursor(Direction::Up, &mut mgr),
                    Key::Down => input.move_cursor(Direction::Down, &mut mgr),
                    Key::Left => input.move_cursor(Direction::Left, &mut mgr),
                    Key::Right => input.move_cursor(Direction::Right, &mut mgr),
                    Key::Home => input.move_to_line_start(&mut mgr),
                    Key::End => input.move_to_line_end(&mut mgr),
                    Key::Delete => input.delete(&mut mgr),
                    Key::AltB => {
                        input.move_cursor(Direction::Left, &mut mgr);
                        input.move_cursor(Direction::Left, &mut mgr);
                        input.move_cursor(Direction::Left, &mut mgr);
                        input.move_cursor(Direction::Left, &mut mgr);
                    }
                    Key::AltF => {
                        input.move_cursor(Direction::Right, &mut mgr);
                        input.move_cursor(Direction::Right, &mut mgr);
                        input.move_cursor(Direction::Right, &mut mgr);
                        input.move_cursor(Direction::Right, &mut mgr);
                    }
                    Key::AltQ => input.toggle_style(Style::Bright, &mut mgr),
                    Key::AltW => input.toggle_style(Style::Transparent, &mut mgr),
                    Key::AltE => input.toggle_style(Style::Italic, &mut mgr),
                    Key::AltR => input.toggle_style(Style::Underline, &mut mgr),
                    Key::AltT => input.toggle_style(Style::Blink, &mut mgr),
                    Key::AltY => input.toggle_style(Style::Blinkfast, &mut mgr),
                    Key::AltU => input.toggle_style(Style::Reverse, &mut mgr),
                    Key::AltI => input.toggle_style(Style::Strike, &mut mgr),
                    _other => {}
                }
            } else if ch == '\u{1b}' {
                break;
            } else if ch == '\u{7f}' {
                input.backspace(&mut mgr);
            } else if ch == '\u{1}' {
                input.move_to_line_start(&mut mgr);
            } else if ch == '\u{5}' {
                input.move_to_line_end(&mut mgr);
            } else if ch == '\u{a}' {
                input.move_cursor(Direction::Down, &mut mgr);
                input.move_to_line_start(&mut mgr);
            } else if ch == '\u{b}' {
                input.remove_chars_from_cursor_to_end(&mut mgr);
            } else if ch == '\u{e}' {
                input.move_cursor(Direction::Down, &mut mgr);
            } else if ch == '\u{10}' {
                input.move_cursor(Direction::Up, &mut mgr);
            } else if ch == '\u{2}' {
                input.move_cursor(Direction::Left, &mut mgr);
            } else if ch == '\u{6}' {
                input.move_cursor(Direction::Right, &mut mgr);
            } else {
                input.insert(&mut mgr, ch);
            }
        }
    }
    mgr.terminate();
}

pub struct Input {
    pub g_id: usize,
    text: Vec<String>,
    cursor_position: (usize, usize),
    max_position: (usize, usize),
}

impl Input {
    pub fn new(mgr: &mut Manager) -> Self {
        let (cols, rows) = mgr.screen_size();
        let m_box = message_box(
            Some(
                "Type in text, ESC: exit, Alt+[Q|W|E|R|T|Y|U|I]: modify selected char".to_string(),
            ),
            String::new(),
            Glyph::plain(),
            cols,
            rows,
        );
        let g_id = mgr.add_graphic(m_box, 0, (0, 0)).unwrap();
        Input {
            g_id,
            text: vec![String::new(); rows],
            cursor_position: (2, 1),
            max_position: (cols - 2, rows - 2),
        }
    }
    pub fn show(&mut self, mgr: &mut Manager) {
        mgr.move_graphic(self.g_id, 2, (0, 0));
        mgr.set_graphic(self.g_id, 0, true);
    }
    pub fn hide(&mut self, mgr: &mut Manager) {
        mgr.move_graphic(self.g_id, 0, (0, 0));
    }
    pub fn insert(&mut self, mgr: &mut Manager, ch: char) {
        let len = self.text[self.cursor_position.1].chars().count();
        if len >= self.max_position.0 - 2 {
            return;
        }
        let glyph = if ch == '\n' {
            Glyph::plain()
        } else {
            Glyph::char(ch)
        };
        if self.cursor_position.0 - 2 == len {
            self.text[self.cursor_position.1].push(ch);
        } else {
            let mut chars = self.text[self.cursor_position.1].chars();
            let mut new_string = String::with_capacity(len);
            for _i in 0..self.cursor_position.0 - 2 {
                if let Some(char) = chars.next() {
                    new_string.push(char);
                } else {
                    new_string.push(' ');
                }
            }
            new_string.push(ch);
            let mut i = 1;
            for char in chars {
                new_string.push(char);
                mgr.set_glyph(
                    self.g_id,
                    Glyph::char(char),
                    i + self.cursor_position.0,
                    self.cursor_position.1,
                );
                i += 1;
            }
            self.text[self.cursor_position.1] = new_string;
        }
        mgr.set_glyph(
            self.g_id,
            glyph,
            self.cursor_position.0,
            self.cursor_position.1,
        );
        if ch == '\n' {
            if self.cursor_position.1 < self.max_position.1 {
                self.cursor_position.0 = 2;
                self.cursor_position.1 += 1;
            }
        } else {
            self.cursor_position.0 += 1;
            if self.cursor_position.0 > self.max_position.0 {
                self.cursor_position.0 = 2;
                self.cursor_position.1 += 1;
                if self.cursor_position.1 > self.max_position.1 {
                    self.cursor_position.1 = self.max_position.1;
                }
            }
        }
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(true);
            glyph.set_reverse(true);
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
    }

    pub fn backspace(&mut self, mgr: &mut Manager) {
        let glyph = Glyph::plain();
        let mut last_char = false;
        let len = self.text[self.cursor_position.1].chars().count();
        match len.cmp(&(self.cursor_position.0 - 2)) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => {
                mgr.set_glyph(
                    self.g_id,
                    glyph,
                    self.cursor_position.0,
                    self.cursor_position.1,
                );
                last_char = true;
                self.text[self.cursor_position.1].pop();
                mgr.set_glyph(
                    self.g_id,
                    Glyph::plain(),
                    self.cursor_position.0,
                    self.cursor_position.1,
                );
            }
            std::cmp::Ordering::Greater => {
                if self.cursor_position.0 == 2 {
                    return;
                }
                mgr.set_glyph(
                    self.g_id,
                    glyph,
                    self.cursor_position.0,
                    self.cursor_position.1,
                );
                let mut chars = self.text[self.cursor_position.1].chars();
                let mut new_string = String::with_capacity(len);
                for _i in 0..self.cursor_position.0 - 3 {
                    if let Some(char) = chars.next() {
                        new_string.push(char);
                    }
                }
                let _skip = chars.next();
                let mut i = 0;
                for char in chars {
                    new_string.push(char);
                    mgr.set_glyph(
                        self.g_id,
                        Glyph::char(char),
                        self.cursor_position.0 + i - 1,
                        self.cursor_position.1,
                    );
                    i += 1;
                }
                self.text[self.cursor_position.1] = new_string;
                mgr.set_glyph(
                    self.g_id,
                    Glyph::plain(),
                    self.cursor_position.0 + i - 1,
                    self.cursor_position.1,
                );
            }
        }
        self.cursor_position.0 -= 1;
        if self.cursor_position.0 < 2 {
            self.cursor_position.1 -= 1;
            if self.cursor_position.1 == 0 {
                self.cursor_position = (2, 1);
            } else {
                self.cursor_position.0 = self.text[self.cursor_position.1].chars().count() + 2;
            }
        }
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(true);
            glyph.set_reverse(true);
            if last_char {
                glyph.set_char(' ');
            }
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
    }

    pub fn delete(&mut self, mgr: &mut Manager) {
        let next_position = (self.cursor_position.0 + 1, self.cursor_position.1);
        if next_position.0 > self.max_position.0 {
            return;
        }
        let len = self.text[self.cursor_position.1].chars().count();
        match len.cmp(&(next_position.0 - 2)) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => {
                self.text[self.cursor_position.1].pop();
                mgr.set_glyph(
                    self.g_id,
                    Glyph::plain(),
                    self.cursor_position.0,
                    self.cursor_position.1,
                );
            }
            std::cmp::Ordering::Greater => {
                let mut chars = self.text[next_position.1].chars();
                let mut new_string = String::with_capacity(len);
                for _i in 0..self.cursor_position.0 - 2 {
                    if let Some(char) = chars.next() {
                        new_string.push(char);
                    }
                }
                let _skip = chars.next();
                let mut i = next_position.0 - 1;
                for char in chars {
                    new_string.push(char);
                    mgr.set_glyph(self.g_id, Glyph::char(char), i, next_position.1);
                    i += 1;
                }
                mgr.set_glyph(self.g_id, Glyph::plain(), i, next_position.1);
                self.text[self.cursor_position.1] = new_string;
            }
        }
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(true);
            glyph.set_reverse(true);
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
    }
    pub fn remove_chars_from_cursor_to_end(&mut self, mgr: &mut Manager) {
        let mut chars = self.text[self.cursor_position.1].chars();
        let mut new_string = String::with_capacity(self.max_position.0);
        for _i in 0..self.cursor_position.0 - 2 {
            if let Some(char) = chars.next() {
                new_string.push(char);
            } else {
                break;
            }
        }
        let mut glyph = Glyph::plain();
        glyph.set_blink(true);
        glyph.set_reverse(true);
        mgr.set_glyph(
            self.g_id,
            glyph,
            self.cursor_position.0,
            self.cursor_position.1,
        );
        let mut i = 1;
        while chars.next().is_some() {
            mgr.set_glyph(
                self.g_id,
                Glyph::plain(),
                self.cursor_position.0 + i,
                self.cursor_position.1,
            );
            i += 1;
        }
        self.text[self.cursor_position.1] = new_string;
    }
    pub fn move_to_line_start(&mut self, mgr: &mut Manager) {
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(false);
            glyph.set_reverse(false);
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
        self.cursor_position.0 = 2;
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(true);
            glyph.set_reverse(true);
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
    }
    pub fn move_to_line_end(&mut self, mgr: &mut Manager) {
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(false);
            glyph.set_reverse(false);
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
        self.cursor_position.0 = self.text[self.cursor_position.1].chars().count() + 2;
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(true);
            glyph.set_reverse(true);
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
    }
    pub fn toggle_style(&self, style: Style, mgr: &mut Manager) {
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            match style {
                Style::Bright => glyph.set_bright(!glyph.bright),
                Style::Transparent => glyph.set_transparent(!glyph.transparent),
                Style::Italic => glyph.set_italic(!glyph.italic),
                Style::Underline => glyph.set_underline(!glyph.underline),
                Style::Blink => glyph.set_blink(!glyph.blink),
                Style::Blinkfast => glyph.set_blinkfast(!glyph.blink_fast),
                Style::Reverse => glyph.set_reverse(!glyph.reverse),
                Style::Strike => glyph.set_strike(!glyph.strike),
            };
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
    }
    pub fn move_cursor(&mut self, direction: Direction, mgr: &mut Manager) {
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(false);
            glyph.set_reverse(false);
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
        match direction {
            Direction::Up => {
                self.cursor_position.1 -= 1;
                if self.cursor_position.1 < 1 {
                    self.cursor_position.1 = self.max_position.1;
                }
            }
            Direction::Down => {
                self.cursor_position.1 += 1;
                if self.cursor_position.1 > self.max_position.1 {
                    self.cursor_position.1 = 1;
                }
            }
            Direction::Left => {
                self.cursor_position.0 -= 1;
                if self.cursor_position.0 < 2 {
                    self.cursor_position.0 = self.max_position.0;
                }
            }
            Direction::Right => {
                self.cursor_position.0 += 1;
                if self.cursor_position.0 > self.max_position.0 {
                    self.cursor_position.0 = 2;
                }
            }
        }
        mgr.get_glyph(self.g_id, self.cursor_position.0, self.cursor_position.1);
        if let Ok(AnimOk::GlyphRetrieved(_u, mut glyph)) = mgr.read_result() {
            glyph.set_blink(true);
            glyph.set_reverse(true);
            mgr.set_glyph(
                self.g_id,
                glyph,
                self.cursor_position.0,
                self.cursor_position.1,
            );
        }
    }
}
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
pub enum Style {
    Bright,
    Transparent,
    Italic,
    Underline,
    Blink,
    Blinkfast,
    Reverse,
    Strike,
}
