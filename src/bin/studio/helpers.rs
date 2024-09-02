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
            ("\u{25C6}         ", glyph),
            (" \u{25C6}        ", glyph),
            ("  \u{25C6}       ", glyph),
            ("   \u{25C6}      ", glyph),
            ("    \u{25C6}     ", glyph),
            ("     \u{25C6}    ", glyph),
            ("      \u{25C6}   ", glyph),
            ("       \u{25C6}  ", glyph),
            ("        \u{25C6} ", glyph),
            ("         \u{25C6}", glyph),
        ],
    );
    library.insert(
        0,
        wrap_border_around(vec![Glyph::default(); 16 * 10], 16, border, Some("Style")),
    );
    let style_window = Graphic::new(18, 12, 0, library, None);

    let plain = Graphic::from_texts(
        11,
        vec![("Plain      ", deselected), ("Plain      ", selected)],
    );
    let bright = Graphic::from_texts(
        11,
        vec![("Bright     ", deselected), ("Bright     ", selected)],
    );
    let dim = Graphic::from_texts(
        11,
        vec![("Dim        ", deselected), ("Dim        ", selected)],
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
    let transparent = Graphic::from_texts(
        11,
        vec![("Transparent", deselected), ("Transparent", selected)],
    );
    let strike = Graphic::from_texts(
        11,
        vec![("Strike     ", deselected), ("Strike     ", selected)],
    );
    let mut result = Vec::with_capacity(10);
    result.push(style_window);
    result.push(selection);
    result.push(plain);
    result.push(bright);
    result.push(dim);
    result.push(italic);
    result.push(underline);
    result.push(blink);
    result.push(blinkfast);
    result.push(reverse);
    result.push(transparent);
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
        let _old = replace(&mut frame[i + 2], *t);
    }
    let mut library = HashMap::new();
    library.insert(0, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("Grayscale ", glyph));
    glyph.set_reverse(false);
    text.append(&mut text_to_frame("8-bit TrueColor", glyph));
    for (i, t) in text.iter().enumerate() {
        let _old = replace(&mut frame[i + 2], *t);
    }
    library.insert(1, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic Grayscale ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("8-bit ", glyph));
    glyph.set_reverse(false);
    text.append(&mut text_to_frame("TrueColor ", glyph));
    for (i, t) in text.iter().enumerate() {
        let _old = replace(&mut frame[i + 2], *t);
    }
    library.insert(2, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic Grayscale 8-bit ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("Truecolor ", glyph));
    for (i, t) in text.iter().enumerate() {
        let _old = replace(&mut frame[i + 2], *t);
    }
    library.insert(3, wrap_border_around(frame, cols, border, title));
    Graphic::new(cols + 2, rows + 2, 0, library, None)
}

pub fn build_workspace_matrix(cols: usize, rows: usize, graphic: Option<Graphic>) -> Graphic {
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
    let mut frame = vec![g; cols * rows];
    if let Some(graphic) = graphic {
        if let Ok(existing_frame) = graphic.get_frame(graphic.current_frame) {
            frame = existing_frame;
        }
    }
    library.insert(
        0,
        wrap_border_around(
            frame,
            cols,
            border,
            Some(&format!("Workspace {}x{}", cols, rows)),
        ),
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
    let comma = ',';
    let plus = '+';
    let mut library = HashMap::new();
    let cols = 16;
    let rows = 10;
    let mut g = Glyph::cyan();
    g.set_color(Color::black());
    let mut avail_index = 0;
    for file in glyph_files {
        if let Ok(file_name) = file {
            if file_name.trim().starts_with('#') {
                continue;
            }
            if let Ok(file) = File::open(&file_name) {
                g.set_char(' ');
                let mut frame = vec![g; cols * rows];
                let mut glyph_codes_lines = vec![];
                for line in io::BufReader::new(file).lines() {
                    if let Ok(line) = line {
                        if line.trim().starts_with('#') {
                            continue;
                        }
                        let mut glyph_codes = vec![];
                        // a_number -- takes given number and following numbers up to cols
                        // a_number,b_number -- takes only explicit numbers
                        // a_number+followers -- takes given numbers plus up to followers count
                        if line.contains(comma) {
                            for number_text in line.split(comma) {
                                if number_text.contains(plus) {
                                    let num_count: Vec<&str> = number_text.split(plus).collect();
                                    if num_count.len() != 2 {
                                        eprint!(
                                            "Unable to parse Glyph line entry {} from {}",
                                            number_text, file_name
                                        );
                                        continue;
                                    }
                                    if let Ok(number) = num_count[0].parse::<u32>() {
                                        if let Ok(count) = num_count[1].parse::<u32>() {
                                            for n in number..number + count {
                                                glyph_codes.push(n);
                                            }
                                        } else {
                                            eprint!(
                                                "Unable to parse Glyph line entry {} from {}",
                                                number_text, file_name
                                            );
                                        }
                                    } else {
                                        eprint!(
                                            "Unable to parse Glyph line entry {} from {}",
                                            number_text, file_name
                                        );
                                    }
                                } else {
                                    // no plus
                                    if let Ok(number) = number_text.parse::<u32>() {
                                        glyph_codes.push(number);
                                    } else {
                                        eprint!(
                                            "Unable to parse Glyph line entry {} from {}",
                                            number_text, file_name
                                        );
                                    }
                                }
                            }
                        } else if line.contains(plus) {
                            let num_count: Vec<&str> = line.split(plus).collect();
                            if num_count.len() != 2 {
                                eprint!(
                                    "Unable to parse Glyph line entry {} from {}",
                                    line, file_name
                                );
                                continue;
                            }
                            if let Ok(number) = num_count[0].parse::<u32>() {
                                if let Ok(count) = num_count[1].parse::<u32>() {
                                    for n in number..number + count {
                                        glyph_codes.push(n);
                                    }
                                } else {
                                    eprint!(
                                        "Unable to parse Glyph line entry {} from {}",
                                        line, file_name
                                    );
                                }
                            } else {
                                eprint!(
                                    "Unable to parse Glyph line entry {} from {}",
                                    line, file_name
                                );
                            }
                        } else if let Ok(number) = line.parse::<u32>() {
                            // no comma
                            for n in number..number + cols as u32 {
                                glyph_codes.push(n);
                            }
                        } else {
                            eprint!(
                                "Unable to parse Glyph line entry {} from {}",
                                line, file_name
                            );
                        }
                        if glyph_codes.len() > 0 {
                            glyph_codes_lines.push(glyph_codes);
                        }
                    }
                }
                let mut next_to_replace = 0;
                for code_line in glyph_codes_lines.into_iter().take(rows) {
                    let mut added = 0;
                    for code in code_line.into_iter().take(cols) {
                        g.set_char(char::from_u32(code as u32).unwrap());
                        //(sp + i as u32) as char
                        let _old = replace(&mut frame[next_to_replace], g.clone());
                        next_to_replace += 1;
                        added += 1;
                    }
                    next_to_replace += cols - added;
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
    let mut library: HashMap<usize, Vec<Glyph>> = HashMap::with_capacity(24);
    let color = Color::white();
    let mut background = Color::black();
    let gt = Glyph::transparent();
    let mut h = Glyph::new(
        '\u{2500}', color, background, false, true, false, false, false, false, false, false,
        false, false,
    );
    let mut v = Glyph::new(
        '\u{2502}', color, background, false, true, false, false, false, false, false, false,
        false, false,
    );
    let mut cr = Glyph::new(
        '\u{253C}', color, background, false, true, false, false, false, false, false, false,
        false, false,
    );
    for i in 0..24 {
        let c = Color::new_gray(i);
        h.set_background(c);
        v.set_background(c);
        cr.set_background(c);
        library.insert(i as usize, vec![cr, h, cr, v, gt, v, cr, h, cr]);
    }

    background = Color::yellow();
    h.set_background(background);
    v.set_background(background);
    h.set_color(background);
    v.set_color(background);
    let lu = Glyph::new(
        '\u{2518}', color, background, false, true, false, false, false, false, false, false,
        false, false,
    );
    let ru = Glyph::new(
        '\u{2514}', color, background, false, true, false, false, false, false, false, false,
        false, false,
    );
    let ld = Glyph::new(
        '\u{2510}', color, background, false, true, false, false, false, false, false, false,
        false, false,
    );
    let rd = Glyph::new(
        '\u{250C}', color, background, false, true, false, false, false, false, false, false,
        false, false,
    );
    library.insert(24, vec![rd, h, ld, v, gt, v, ru, h, lu]);

    // TODO: two frames for indicating macro recording phase
    background = Color::red();
    let r = Glyph::new(
        'R', color, background, false, true, false, false, false, false, false, false, false, false,
    );
    let e = Glyph::new(
        'E', color, background, false, true, false, false, false, false, false, false, false, false,
    );
    let c = Glyph::new(
        'C', color, background, false, true, false, false, false, false, false, false, false, false,
    );
    let m = Glyph::new(
        '*', color, background, false, true, false, false, false, false, false, false, false, false,
    );
    let l = Glyph::new(
        'O', color, background, false, true, false, false, false, false, false, false, false, false,
    );
    library.insert(25, vec![r, e, c, m, gt, m, m, m, m]);
    library.insert(26, vec![m, m, m, m, gt, m, r, e, c]);
    library.insert(27, vec![r, e, c, l, gt, l, l, l, l]);
    library.insert(28, vec![l, l, l, l, gt, l, r, e, c]);
    let frame_time = Timestamp::new(0, 40);
    let anim_default = Animation::new(
        true,
        true,
        vec![
            (0, frame_time),
            (1, frame_time),
            (2, frame_time),
            (3, frame_time),
            (4, frame_time),
            (5, frame_time),
            (6, frame_time),
            (7, frame_time),
            (8, frame_time),
            (9, frame_time),
            (10, frame_time),
            (11, frame_time),
            (12, frame_time),
            (13, frame_time),
            (14, frame_time),
            (15, frame_time),
            (16, frame_time),
            (17, frame_time),
            (18, frame_time),
            (19, frame_time),
            (20, frame_time),
            (21, frame_time),
            (22, frame_time),
            (23, frame_time),
            (22, frame_time),
            (21, frame_time),
            (20, frame_time),
            (19, frame_time),
            (18, frame_time),
            (17, frame_time),
            (16, frame_time),
            (15, frame_time),
            (14, frame_time),
            (13, frame_time),
            (12, frame_time),
            (11, frame_time),
            (10, frame_time),
            (9, frame_time),
            (8, frame_time),
            (7, frame_time),
            (6, frame_time),
            (5, frame_time),
            (4, frame_time),
            (3, frame_time),
            (2, frame_time),
            (1, frame_time),
            (0, frame_time),
        ],
        Timestamp::now(),
    );
    let anim_select = Animation::new(
        false,
        false,
        vec![(24, Timestamp::new(0, 100)), (0, Timestamp::new(0, 1))],
        Timestamp::now(),
    );
    let mut anims = HashMap::new();
    anims.insert(0, anim_default);
    anims.insert(1, anim_select);
    Graphic::new(3, 3, 0, library, Some(anims))
}

pub fn build_glyph_selector() -> Graphic {
    let mut library: HashMap<usize, Vec<Glyph>> = HashMap::with_capacity(10);
    //let color = Color::white();
    //let mut background = Color::black();
    let gt = Glyph::transparent();
    let r = Glyph::red();
    let o = Glyph::orange();
    let y = Glyph::yellow();
    let g = Glyph::green();
    let b = Glyph::blue();
    let i = Glyph::indigo();
    let v = Glyph::violet();
    let w = Glyph::white();
    library.insert(0, vec![r, w, v, o, gt, g, y, b, i]);
    library.insert(1, vec![o, r, w, y, gt, v, b, i, g]);
    library.insert(2, vec![y, o, r, g, gt, w, i, g, v]);
    library.insert(3, vec![g, y, o, b, gt, r, g, v, w]);
    library.insert(4, vec![b, g, y, i, gt, o, v, w, r]);
    library.insert(5, vec![i, b, g, v, gt, y, w, r, o]);
    library.insert(6, vec![v, i, b, w, gt, b, r, o, y]);
    library.insert(7, vec![w, v, i, r, gt, i, o, y, b]);
    library.insert(8, vec![r, r, r, r, gt, r, r, r, r]);
    library.insert(9, vec![o, o, o, o, gt, o, o, o, o]);
    library.insert(10, vec![y, y, y, y, gt, y, y, y, y]);
    library.insert(11, vec![b, b, b, b, gt, b, b, b, b]);
    library.insert(12, vec![i, i, i, i, gt, i, i, i, i]);
    library.insert(13, vec![g, g, g, g, gt, g, g, g, g]);
    library.insert(14, vec![v, v, v, v, gt, v, v, v, v]);
    let mut frame_time = Timestamp::new(0, 100);
    let anim_default = Animation::new(
        true,
        true,
        vec![
            (0, frame_time),
            (1, frame_time),
            (2, frame_time),
            (3, frame_time),
            (4, frame_time),
            (5, frame_time),
            (6, frame_time),
            (7, frame_time),
        ],
        Timestamp::now(),
    );
    frame_time = Timestamp::new(0, 50);
    let anim_select = Animation::new(
        false,
        false,
        vec![
            (8, frame_time),
            (9, frame_time),
            (10, frame_time),
            (11, frame_time),
            (12, frame_time),
            (13, frame_time),
            (14, frame_time),
        ],
        Timestamp::now(),
    );
    let mut anims = HashMap::new();
    anims.insert(0, anim_default);
    anims.insert(1, anim_select);
    Graphic::new(3, 3, 0, library, Some(anims))
}
