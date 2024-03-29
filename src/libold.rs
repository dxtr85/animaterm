use std::collections::HashMap;
use std::env;
use std::time::Instant;
//use std::fmt::Display;
use std::default::Default;
use std::io;
use std::io::Write;
use std::ops::{Add, AddAssign};
use std::process::Command;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

pub struct Screen {
    pub lines: usize,
    pub cols: usize,
    display: HashMap<(usize, usize), Pixel>,
    time: Timestamp,
    animations: Vec<Animation>,
    to_print: Vec<(usize, usize)>,
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
    pub fn new(lines: Option<usize>, cols: Option<usize>, glyph: Option<Glyph>) -> Self {
        let (new_lines, new_cols) = ask_os_for_rows_and_cols();
        let final_lines = if lines.is_none() {
            new_lines
        } else {
            lines.unwrap()
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
        let c_y = final_lines;
        if glyph.is_none() {
            let glyph = Glyph::default();
            let mut display = HashMap::with_capacity(final_cols * final_lines);
            for j in 1..final_lines + 1 {
                for i in 1..final_cols + 1 {
                    display.insert((i, j), Pixel::new(i, j, true, glyph.clone()));
                }
            }
            Screen {
                lines: final_lines,
                cols: final_cols,
                display: display,
                time: Timestamp::now(),
                animations: vec![],
                to_print: vec![],
                stdin,
                stdout: io::stdout(),
                termios_orig: termios,
                termios: new_termios,
                c_color: Color::White,
                c_background: Color::Black,
                c_x,
                c_y,
                c_plain: false,
                c_bright: false,
                c_dim: false,
                c_italic: false,
                c_underline: false,
                c_blink: false,
                c_blink_fast: false,
                c_reverse: false,
                c_strike: false,
            }
        } else {
            let glyph = glyph.unwrap();
            println!("Filling display with: {}", glyph.character);
            let mut display = HashMap::with_capacity(final_cols * final_lines);
            for j in 1..final_lines + 1 {
                for i in 1..final_cols + 1 {
                    display.insert((i, j), Pixel::new(i, j, true, glyph.clone()));
                }
            }
            Screen {
                lines: final_lines,
                cols: final_cols,
                display: display,
                time: Timestamp::now(),
                animations: vec![],
                to_print: vec![],
                stdin,
                stdout: io::stdout(),
                termios_orig: termios,
                termios: new_termios,
                c_color: glyph.color,
                c_background: glyph.background,
                c_x,
                c_y,
                c_plain: glyph.plain,
                c_bright: glyph.bright,
                c_dim: glyph.dim,
                c_italic: glyph.italic,
                c_blink: glyph.blink,
                c_blink_fast: glyph.blink_fast,
                c_underline: glyph.underline,
                c_reverse: glyph.reverse,
                c_strike: glyph.strike,
            }
        }
    }

    pub fn add_animation(&mut self, a: Animation) -> usize {
        self.animations.push(a);
        0
    }

    pub fn update_animations(&mut self) {
        let mut pixels = vec![];
        for anim in &mut self.animations {
            if let Some(mut pxls) = anim.update(self.time.tick()) {
                pixels.append(&mut pxls);
            }
        }
        self.update(&pixels);
        self.refresh();
    }

    pub fn cls(&self) {
        print!("\x1b[H{:<1$}\x1b[1;1H", "", self.cols * self.lines);
        // print!("\x1b[H\x1b[37;40m{:<1$}", "", self.cols * self.lines);
        self.stdout.lock().flush().unwrap();
    }

    pub fn refresh(&mut self) {
        // println!("Refreshing {} pixels", self.to_print.len());
        for (x, y) in self.to_print.clone() {
            let pixel = self.display.get(&(x, y)).unwrap().clone();
            // println!("To refr: {:?}", pixel);
            let mut formated = String::new();
            if self.c_x != x || self.c_y != y {
                // print!("Będę zmieniał pozycję");
                formated.push_str(&format!("\x1b[{};{}H", y, x));
                self.c_x = x + 1;
                self.c_y = y;
            };
            let modifier = self.gformat(&(pixel.x, pixel.y));
            // print!("Teraz formated: {:?}", &formated);
            if !modifier.is_empty() {
                formated.push_str("\x1b[");
                formated.push_str(&modifier);
                formated.push('m');
            }

            // print!("F: {:?}, M: {:?}", &formated[2..], &modifier[2..]);
            print!("{}{}", formated, pixel.g.character);
            self.stdout.lock().flush().unwrap(); // print!("Done");
        }
        self.to_print = vec![];
        self.stdout.lock().flush().unwrap();
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

    pub fn message_box(
        &mut self,
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
                if i > width - 2 {
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
                while content.len() < width - i - 1 {
                    for c in content.chars() {
                        cgl.set_char(c);
                        mbox.push(Pixel::new(start_x + i, j, true, cgl.clone()));
                        i += 1;
                    }
                    //cgl.set_char(' ');
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

    // pub fn print_all(&mut self, pixels: &Vec<Pixel>) {
    //     for p in pixels {
    //         self.print(p);
    //     }
    // }

    pub fn update(&mut self, pixels: &Vec<Pixel>) {
        // println!("Updating {} pixels", pixels.len());
        for p in pixels {
            let x = p.x;
            let y = p.y;
            self.display.insert((x, y), *p);
            self.to_print.push((x, y));
        }
    }

    pub fn print(&mut self, pixel: &Pixel) {
        if pixel.x > self.cols || pixel.y > self.lines {
            return;
        }
        self.display.insert((pixel.x, pixel.y), pixel.clone());
        // print!("Drukuje: {:?}", pixel);
        let mut formated = String::new();
        if self.c_x != pixel.x || self.c_y != pixel.y {
            // print!("Będę zmieniał pozycję");
            formated.push_str(&format!("\x1b[{};{}H", pixel.y, pixel.x));
            self.c_x = pixel.x + 1;
            self.c_y = pixel.y;
        };
        let modifier = self.gformat(&(pixel.x, pixel.y));
        //print!("Teraz formated: {:?}", &modifier);
        if !modifier.is_empty() {
            formated.push_str("\x1b[");
            formated.push_str(&modifier);
            formated.push('m');
        }

        // print!("F: {:?}, M: {:?}", &formated[2..], &modifier[2..]);
        print!("{}{}", formated, pixel.g.character);
        self.stdout.lock().flush().unwrap(); // print!("Done");
    }

    fn gformat(&mut self, pos: &(usize, usize)) -> String {
        let glyph = self.display.get(pos).unwrap().g;
        // print!("{:?}", glyph.background);
        let mut modifier = String::new(); //"\x1b[".to_string();
                                          // let mut add_modifier = false;
                                          // Plain = 0,
        if self.c_plain && !glyph.plain {
            // print!("Było plain");
            self.c_plain = false;
        } else if !self.c_plain && glyph.plain {
            // print!("Nie było plain");
            self.c_plain = true;
            modifier.push_str("0;");
            //add_modifier = true;
        }
        // Bright = 1,
        if self.c_bright && !glyph.bright {
            // print!("Było brajt");
            self.c_bright = false;
            modifier.push_str("21;");
            //add_modifier = true;
        } else if !self.c_bright && glyph.bright {
            // print!("Nie było brajt");
            self.c_bright = true;
            modifier.push_str("1;");
            //add_modifier = true;
        }
        // Dim = 2,
        if self.c_dim && !glyph.dim {
            // print!("Było dim");
            self.c_dim = false;
            modifier.push_str("22;");
            //add_modifier = true;
        } else if !self.c_dim && glyph.dim {
            // print!("Nie było dim");
            self.c_dim = true;
            modifier.push_str("2;");
            //add_modifier = true;
        }
        // Italic = 3,
        if self.c_italic && !glyph.italic {
            // print!("Było italic");
            self.c_italic = false;
            modifier.push_str("23;");
            //add_modifier = true;
        } else if !self.c_italic && glyph.italic {
            // print!("Nie było italic");
            self.c_italic = true;
            modifier.push_str("3;");
            //add_modifier = true;
        }
        // Underline = 4,
        if self.c_underline && !glyph.underline {
            // print!("Było underline");
            self.c_underline = false;
            modifier.push_str("24;");
            //add_modifier = true;
        } else if !self.c_underline && glyph.underline {
            // print!("Nie było underline");
            self.c_underline = true;
            modifier.push_str("4;");
            //add_modifier = true;
        }
        // Blink = 5,
        if self.c_blink && !glyph.blink {
            // print!("Było blink");
            self.c_blink = false;
            modifier.push_str("25;");
            //add_modifier = true;
        } else if !self.c_blink && glyph.blink {
            // print!("Nie było blink");
            self.c_blink = true;
            modifier.push_str("5;");
            //add_modifier = true;
        }
        // BlinkFast = 6,
        if self.c_blink_fast && !glyph.blink_fast {
            // print!("Było blink_fast");
            self.c_blink_fast = false;
            modifier.push_str("26;");
            //add_modifier = true;
        } else if !self.c_blink_fast && glyph.blink_fast {
            // print!("Nie było blink_fast");
            self.c_blink_fast = true;
            modifier.push_str("6;");
            //add_modifier = true;
        }
        // Reverse = 7,
        if self.c_reverse && !glyph.reverse {
            // print!("Było reverse");
            self.c_reverse = false;
            modifier.push_str("27;");
            //add_modifier = true;
        } else if !self.c_reverse && glyph.reverse {
            // print!("Nie było reverse");
            self.c_reverse = true;
            modifier.push_str("7;");
            //add_modifier = true;
        }
        // Strike = 9,
        if self.c_strike && !glyph.strike {
            // print!("Było strike");
            self.c_strike = false;
            modifier.push_str("29;");
            //add_modifier = true;
        } else if !self.c_strike && glyph.strike {
            // print!("Nie było strike");
            self.c_strike = true;
            modifier.push_str("9;");
            //add_modifier = true;
        }
        //print!("{:?}:{:?}", self.c_color, glyph.color);
        if self.c_color != glyph.color {
            //print!("Zmieniam kolor");
            modifier.push_str(&format!("3{};", glyph.color as u8));
            self.c_color = glyph.color;
            //add_modifier = true;
        };
        if self.c_background != glyph.background {
            //print!("Zmieniam tło");
            modifier.push_str(&format!("4{}", glyph.background as u8));
            self.c_background = glyph.background;
            //add_modifier = true;
        };
        modifier
    }

    pub fn initialize(&mut self) {
        self.termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(self.stdin, TCSANOW, &mut self.termios).unwrap();
        print!("\x1b[?1049h"); // use separate buffer
        print!("\x1b[2J"); // clear screen
        print!("\x1b[?25l"); // disable cursor
    }

    pub fn cleanup(&self) {
        print!("\x1b[?25h"); // enable cursor
        print!("\x1b[2J"); // clear screen
        print!("\x1b[?1049l"); // disable separate buffer
        tcsetattr(self.stdin, TCSANOW, &self.termios_orig).unwrap(); // reset the stdin to
                                                                     // original termios data
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
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
impl Add for Timestamp {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut next_one = self.1 + other.1;
        let mut next_zero = self.0 + other.0;
        if next_one > 999 {
            next_zero += (next_one / 1000) as u64;
            next_one %= 1000;
        }
        Self(next_zero, next_one, Instant::now())
    }
}

impl AddAssign for Timestamp {
    fn add_assign(&mut self, o: Self) {
        let new_self = *self + o;
        *self = new_self;
    }
}

pub struct Animation {
    current_frame: usize,
    next_frame: usize,
    looping: bool,
    frames: HashMap<usize, Vec<Pixel>>,
    ordering: Vec<(usize, Timestamp)>,
    ord_max: usize,
    trigger_time: Timestamp,
}

impl Animation {
    pub fn new(
        frames: HashMap<usize, Vec<Pixel>>,
        looping: bool,
        ordering: Vec<(usize, Timestamp)>,
        start_time: Timestamp,
    ) -> Animation {
        let ord_max = ordering.len() - 1;
        Animation {
            current_frame: 0,
            next_frame: 0,
            looping,
            frames,
            ordering,
            ord_max,
            trigger_time: start_time,
        }
    }

    pub fn update(&mut self, dtime: Timestamp) -> Option<Vec<Pixel>> {
        if dtime >= self.trigger_time {
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

pub enum Style {
    Plain = 0,
    Bright = 1,
    Dim = 2,
    Italic = 3,
    Underline = 4,
    Reverse = 7,
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
// impl Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.longitude, self.latitude)
//     }
// }

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
}

#[derive(Copy, Clone, Debug)]
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
    pub fn set_char(&mut self, character: char) {
        self.character = character;
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }
}
impl Default for Glyph {
    fn default() -> Self {
        Glyph {
            character: ' ',
            color: Color::White,
            background: Color::Green,
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
            // println!("OK {}", &output);
            let number = usize::from_str_radix(output.trim(), 10);
            match number {
                Ok(a_number) => {
                    // println!("OK {}", a_number);
                    a_number
                }
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
    use super::Timestamp;
    #[test]
    fn overflow_ms_to_sec() {
        let t0 = Timestamp(0, 0);
        let t1 = Timestamp(0, 1000);
        assert_eq!(t0 + t1, Timestamp(1, 0));
    }
    #[test]
    fn add_sec_to_sec() {
        let t0 = Timestamp(1, 500);
        let t1 = Timestamp(2, 700);
        assert_eq!(t0 + t1, Timestamp(4, 200));
    }
}
