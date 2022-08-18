use animaterm::prelude::*;
use animaterm::utilities::{text_to_frame, wrap_border_around};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::mem::replace;

pub fn build_style_graphics(selected: Glyph, deselected: Glyph) -> Vec<Graphic> {
    let border = [
        Glyph::default_with_char('\u{256D}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256E}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2570}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256F}'),
    ];
    let mut library = HashMap::new();
    let glyph = Glyph::default();
    let selection = Graphic::from_texts(
        1,
        vec![
            ("\u{25C6}       ", glyph),
            (" \u{25C6}      ", glyph),
            ("  \u{25C6}     ", glyph),
            ("   \u{25C6}    ", glyph),
            ("    \u{25C6}   ", glyph),
            ("     \u{25C6}  ", glyph),
            ("      \u{25C6} ", glyph),
            ("       \u{25C6}", glyph),
        ],
    );
    library.insert(
        0,
        wrap_border_around(vec![Glyph::default(); 16 * 8], 16, border, Some("Style")),
    );
    let style_window = Graphic::new(18, 10, 0, library, None);

    let transparent = Graphic::from_texts(
        11,
        vec![("Transparent", deselected), ("Transparent", selected)],
    );
    let bright = Graphic::from_texts(
        11,
        vec![("Bright     ", deselected), ("Bright     ", selected)],
    );
    let italic = Graphic::from_texts(
        11,
        vec![("Italic     ", deselected), ("Italic     ", selected)],
    );
    let underline = Graphic::from_texts(
        11,
        vec![("Underline  ", deselected), ("Underline  ", selected)],
    );
    let blink = Graphic::from_texts(
        11,
        vec![("Blink      ", deselected), ("Blink      ", selected)],
    );
    let blinkfast = Graphic::from_texts(
        11,
        vec![("Blink fast ", deselected), ("Blink fast ", selected)],
    );
    let reverse = Graphic::from_texts(
        11,
        vec![("Reverse    ", deselected), ("Reverse    ", selected)],
    );
    let strike = Graphic::from_texts(
        11,
        vec![("Strike     ", deselected), ("Strike     ", selected)],
    );
    let mut result = Vec::with_capacity(10);
    result.push(style_window);
    result.push(selection);
    result.push(transparent);
    result.push(bright);
    result.push(italic);
    result.push(underline);
    result.push(blink);
    result.push(blinkfast);
    result.push(reverse);
    result.push(strike);
    result
}

pub fn build_basic_colors_graphic(mut selected: Glyph, mut deselected: Glyph) -> Graphic {
    let mut library = HashMap::new();

    let mut text = text_to_frame("Black   ", selected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(0, text);

    text = text_to_frame("Black   ", deselected);
    selected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", selected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(1, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    selected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", selected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(2, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    selected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", selected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(3, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    selected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", selected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(4, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    selected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", selected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(5, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    selected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", selected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(6, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    selected.set_color(Color::black());
    text.append(&mut text_to_frame("White   ", selected));
    library.insert(7, text);

    Graphic::new(32, 2, 0, library, None)
}

pub fn build_color_selector(title: Option<&str>) -> Graphic {
    let border = [
        Glyph::default_with_char('\u{256D}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256E}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2570}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256F}'),
    ];
    let cols = 40;
    let rows = 5;
    let mut glyph = Glyph::default();
    glyph.set_reverse(true);
    let mut text = text_to_frame("Basic ", glyph);
    glyph.set_reverse(false);
    text.append(&mut text_to_frame("Grayscale 8-bit TrueColor", glyph));
    let mut frame = vec![Glyph::default(); cols * rows];
    for (i, t) in text.iter().enumerate() {
        replace(&mut frame[i + 2], *t);
    }
    let mut library = HashMap::new();
    library.insert(0, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("Grayscale ", glyph));
    glyph.set_reverse(false);
    text.append(&mut text_to_frame("8-bit TrueColor", glyph));
    for (i, t) in text.iter().enumerate() {
        replace(&mut frame[i + 2], *t);
    }
    library.insert(1, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic Grayscale ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("8-bit ", glyph));
    glyph.set_reverse(false);
    text.append(&mut text_to_frame("TrueColor ", glyph));
    for (i, t) in text.iter().enumerate() {
        replace(&mut frame[i + 2], *t);
    }
    library.insert(2, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic Grayscale 8-bit ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("Truecolor ", glyph));
    for (i, t) in text.iter().enumerate() {
        replace(&mut frame[i + 2], *t);
    }
    library.insert(3, wrap_border_around(frame, cols, border, title));
    Graphic::new(cols + 2, rows + 2, 0, library, None)
}

pub fn build_empty_matrix(cols: usize, rows: usize) -> Graphic {
    let border = [
        Glyph::default_with_char('\u{256D}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256E}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2570}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256F}'),
    ];
    let mut library = HashMap::new();
    let g = Glyph::default();
    let frame = vec![g; cols * rows];
    library.insert(
        0,
        wrap_border_around(frame, cols, border, Some("Workspace")),
    );
    Graphic::new(cols + 2, rows + 2, 0, library, None)
}

pub fn build_glyph_matrix(index_file: Option<String>) -> Graphic {
    let mut glyph_files = vec![];
    if let Some(index_file) = index_file {
        if let Ok(file) = File::open(index_file) {
            for line in io::BufReader::new(file).lines() {
                glyph_files.push(line);
            }
        }
    }
    //    let start_points = vec![9472, 9488, 9504, 9520, 9536, 9552, 9568, 9584, 9600, 9616];
    let border = [
        Glyph::default_with_char('\u{256D}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256E}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2570}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256F}'),
    ];
    let mut library = HashMap::new();
    let cols = 16;
    let rows = 10; //start_points.len();
    let mut g = Glyph::default();
    g.set_color(Color::black());
    g.set_background(Color::cyan());
    let mut avail_index = 0;
    for file in glyph_files {
        if let Ok(file_name) = file {
            if file_name.trim().starts_with('#') {
                continue;
            }
            if let Ok(file) = File::open(&file_name) {
                g.set_char(' ');
                let mut frame = vec![g; cols * rows];
                let mut start_points = vec![];
                for line in io::BufReader::new(file).lines() {
                    if let Ok(line) = line {
                        if line.trim().starts_with('#') {
                            continue;
                        }
                        if let Ok(number) = line.parse::<u32>() {
                            start_points.push(number);
                        }
                    }
                }
                let mut next_to_replace = 0;
                for sp in start_points {
                    for i in 0..cols {
                        g.set_char(char::from_u32(sp + i as u32).unwrap());
                        //(sp + i as u32) as char
                        replace(&mut frame[next_to_replace], g.clone());
                        next_to_replace += 1;
                    }
                }
                let mut name = file_name;
                if name.contains('/') {
                    name = name.split('/').last().unwrap().to_string();
                }

                library.insert(
                    avail_index,
                    wrap_border_around(frame, cols, border, Some(&name)),
                );
                avail_index += 1;
            }
        }
    }
    let mut frame = Vec::with_capacity(cols * rows);
    let start_points = vec![9472, 9488, 9504, 9520, 9536, 9552, 9568, 9584, 9600, 9616];
    for sp in start_points {
        for i in 0..cols {
            g.set_char(char::from_u32(sp + i as u32).unwrap());
            //(sp + i as u32) as char
            //replace(&mut frame[next_to_replace], g.clone());
            frame.push(g);
        }
    }
    library.insert(
        avail_index,
        wrap_border_around(frame, cols, border, Some("default".into())),
    );

    Graphic::new(cols + 2, rows + 2, avail_index, library, None)
}

pub fn build_selector() -> Graphic {
    let mut library = HashMap::with_capacity(2);
    let color = Color::white();
    let mut background = Color::black();
    let gt = Glyph::transparent();
    let mut h = Glyph::new(
        '\u{2500}', color, background, false, true, false, false, false, false, false, false,
    );
    let mut v = Glyph::new(
        '\u{2502}', color, background, false, true, false, false, false, false, false, false,
    );
    let cr = Glyph::new(
        '\u{253C}', color, background, false, true, false, false, false, false, false, false,
    );
    library.insert(0, vec![cr, h, cr, v, gt, v, cr, h, cr]);
    background = Color::yellow();
    h.set_background(background);
    v.set_background(background);
    let lu = Glyph::new(
        '\u{2518}', color, background, false, true, false, false, false, false, false, false,
    );
    let ru = Glyph::new(
        '\u{2514}', color, background, false, true, false, false, false, false, false, false,
    );
    let ld = Glyph::new(
        '\u{2510}', color, background, false, true, false, false, false, false, false, false,
    );
    let rd = Glyph::new(
        '\u{250C}', color, background, false, true, false, false, false, false, false, false,
    );
    library.insert(1, vec![rd, h, ld, v, gt, v, ru, h, lu]);
    let anim_select = Animation::new(
        false,
        false,
        vec![(1, Timestamp::new(0, 100)), (0, Timestamp::new(0, 1))],
        Timestamp::now(),
    );
    let mut anims = HashMap::new();
    anims.insert(0, anim_select);
    Graphic::new(3, 3, 0, library, Some(anims))
}
