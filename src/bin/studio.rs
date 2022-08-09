use animaterm::prelude::*;
use animaterm::utilities::{progress_bar, text_to_frame, wrap_border_around};
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::mem::replace;
use std::process::exit;

static ROWS_MIN: usize = 4;
static COLS_MIN: usize = 5;

fn main() {
    let args = parse_arguments();
    let cols = args.cols;
    let rows = args.rows;
    verify_cols_and_rows(cols, rows);
    let mut mgr = Manager::new(true, cols, rows, None);
    let (cols, rows) = mgr.screen_size();
    let mut keep_running = true;
    let mut selector = build_selector();
    let sid = mgr.add_graphic(selector, 1, (0, 7));
    let gmid = mgr.add_graphic(
        build_glyph_matrix(vec![
            9472, 9488, 9504, 9520, 9536, 9552, 9568, 9584, 9600, 9616,
        ]),
        0,
        (0, 7),
    );
    let mut curr_col_tab = 3;
    let col_sel_id = mgr.add_graphic(build_color_selector(Some("Color")), 0, (1, 0));
    let mut curr_bg_tab = 1;
    let bg_sel_id = mgr.add_graphic(build_color_selector(Some("Background")), 0, (64, 0));
    let matrix_cols = 60;
    let matrix_rows = 10;
    let mut glyph = Glyph::default();
    glyph.set_color(Color::black());
    glyph.set_background(Color::white());
    let basic_sel_id = mgr.add_graphic(
        build_basic_colors_graphic(glyph, Glyph::default()),
        2,
        (4, 3),
    );
    let mut basic_selected_color = 0;
    glyph = Glyph::default();
    mgr.set_graphic(basic_sel_id, basic_selected_color, false);
    mgr.set_invisible(basic_sel_id, true);
    let did = mgr.add_graphic(build_empty_matrix(matrix_cols, matrix_rows), 0, (19, 7));
    let mut vc_cursor = 0;
    let vc_id = mgr.add_graphic(
        Graphic::from_texts(
            1,
            vec![
                ("\u{25C6}    ", glyph),
                ("  \u{25C6}  ", glyph),
                ("   \u{25C6} ", glyph),
                ("    \u{25C6}", glyph),
            ],
        ),
        1,
        (2, 1),
    );
    mgr.set_graphic(vc_id, vc_cursor, true);
    let glyph2 = glyph.clone();
    glyph.set_color(Color::red());
    let pb1t_id = mgr.add_graphic(
        Graphic::from_texts(6, vec![("Red   ", glyph), ("Bright", glyph2)]),
        1,
        (4, 3),
    );
    mgr.set_graphic(pb1t_id, 0, true);
    glyph.set_color(Color::green());
    let pb2t_id = mgr.add_graphic(Graphic::from_text(6, "Green", glyph), 1, (4, 4));
    mgr.set_graphic(pb2t_id, 0, true);
    glyph.set_color(Color::blue());
    let pb3t_id = mgr.add_graphic(Graphic::from_text(6, "Blue", glyph), 1, (4, 5));
    mgr.set_graphic(pb3t_id, 0, true);
    glyph.set_color(Color::white());
    let pb1_id = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            None,
        ),
        1,
        (10, 3),
    );
    let pb2_id = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            None,
        ),
        1,
        (10, 4),
    );
    let pb3_id = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            None,
        ),
        1,
        (10, 5),
    );
    mgr.set_graphic(pb1_id, 24, true);
    mgr.set_graphic(pb2_id, 31, true);
    mgr.set_graphic(pb3_id, 16, true);
    mgr.set_graphic(sid, 0, true);
    mgr.set_graphic(gmid, 0, true);
    mgr.set_graphic(did, 0, true);
    mgr.set_graphic(col_sel_id, curr_col_tab, true);
    mgr.set_graphic(bg_sel_id, curr_bg_tab, true);

    let mut c = 2;
    let mut r = 2;
    let mut glyph_under_cursor = Glyph::default();
    mgr.get_glyph(did, c, r);
    let result = mgr.read_result();
    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
        glyph_under_cursor = glyph;
    }

    let mut mc = 2;
    let mut mr = 2;
    let g = Glyph::new(
        '\u{2588}',
        Color::blue(),
        Color::white(),
        false,
        false,
        false,
        false,
        true,
        false,
        false,
        false,
    );
    mgr.set_glyph(did, g, c, r);
    while keep_running {
        if let Some(key) = mgr.read_key() {
            match key {
                Key::Right => {
                    if mc < 17 {
                        mc += 1;
                        mgr.move_graphic(sid, 2, (1, 0));
                    }
                }
                Key::Shift_Right => {
                    if vc_cursor == 0 {
                        curr_col_tab += 1;
                        if curr_col_tab > 3 {
                            curr_col_tab = 0;
                        }
                        mgr.set_graphic(col_sel_id, curr_col_tab, true);
                        match curr_col_tab {
                            0 => {
                                mgr.set_invisible(pb1t_id, true);
                                mgr.set_invisible(pb2t_id, true);
                                mgr.set_invisible(pb3t_id, true);
                                mgr.set_invisible(pb1_id, true);
                                mgr.set_invisible(pb2_id, true);
                                mgr.set_invisible(pb3_id, true);
                                mgr.set_invisible(basic_sel_id, false);
                            }
                            1 => {
                                //Grayscale
                                mgr.set_graphic(pb1t_id, 1, true);
                                mgr.set_invisible(pb1t_id, false);
                                mgr.set_invisible(pb1_id, false);
                                mgr.set_invisible(basic_sel_id, true);
                            }
                            2 => {
                                //8-bit
                                mgr.set_graphic(pb1t_id, 0, true);
                                mgr.set_invisible(pb1t_id, false);
                                mgr.set_invisible(pb2t_id, false);
                                mgr.set_invisible(pb3t_id, false);
                                mgr.set_invisible(pb1_id, false);
                                mgr.set_invisible(pb2_id, false);
                                mgr.set_invisible(pb3_id, false);
                            }
                            3 => { //Truecolor
                            }
                            _ => {
                                continue;
                            }
                        }
                    } else {
                        if vc_cursor == 1 && curr_col_tab == 0 {
                            //Basic
                            basic_selected_color += 1;
                            if basic_selected_color > 7 {
                                basic_selected_color = 0;
                            }
                            mgr.set_graphic(basic_sel_id, basic_selected_color, true);
                        }
                    }
                }
                Key::Shift_Up => {
                    if vc_cursor == 0 {
                        if curr_col_tab == 0 || curr_col_tab == 1 {
                            vc_cursor = 1;
                        } else {
                            vc_cursor = 3;
                        }
                    } else {
                        vc_cursor -= 1;
                    }
                    mgr.set_graphic(vc_id, vc_cursor, true);
                }
                Key::Shift_Down => {
                    if vc_cursor == 3 {
                        vc_cursor = 0;
                    } else {
                        vc_cursor += 1;
                        if vc_cursor > 1 && (curr_col_tab == 0 || curr_col_tab == 1) {
                            vc_cursor = 0;
                        }
                    }
                    mgr.set_graphic(vc_id, vc_cursor, true);
                }
                Key::Alt_Right => {
                    curr_bg_tab += 1;
                    if curr_bg_tab > 3 {
                        curr_bg_tab = 0;
                    }
                    mgr.set_graphic(bg_sel_id, curr_bg_tab, true);
                }
                Key::Shift_Left => {
                    if vc_cursor == 0 {
                        if curr_col_tab == 0 {
                            curr_col_tab = 3;
                        } else {
                            curr_col_tab -= 1;
                        }
                        mgr.set_graphic(col_sel_id, curr_col_tab, true);
                        match curr_col_tab {
                            0 => {
                                mgr.set_invisible(basic_sel_id, false);
                                mgr.set_invisible(pb1t_id, true);
                                mgr.set_invisible(pb2t_id, true);
                                mgr.set_invisible(pb3t_id, true);
                                mgr.set_invisible(pb1_id, true);
                                mgr.set_invisible(pb2_id, true);
                                mgr.set_invisible(pb3_id, true);
                            } //TODO basic color select visible
                            1 => {
                                //Grayscale
                                mgr.set_graphic(pb1t_id, 1, true);
                                mgr.set_invisible(pb1t_id, false);
                                mgr.set_invisible(pb1_id, false);
                                mgr.set_invisible(pb2t_id, true);
                                mgr.set_invisible(pb3t_id, true);
                                mgr.set_invisible(pb2_id, true);
                                mgr.set_invisible(pb3_id, true);
                            } //TODO basic color invisible
                            2 => {
                                //8-bit
                                mgr.set_graphic(pb1t_id, 0, true);
                            }
                            3 => {
                                //Truecolor
                                mgr.set_invisible(basic_sel_id, true);
                                mgr.set_invisible(pb1t_id, false);
                                mgr.set_invisible(pb2t_id, false);
                                mgr.set_invisible(pb3t_id, false);
                                mgr.set_invisible(pb1_id, false);
                                mgr.set_invisible(pb2_id, false);
                                mgr.set_invisible(pb3_id, false);
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }
                Key::Alt_Left => {
                    if curr_bg_tab == 0 {
                        curr_bg_tab = 3;
                    } else {
                        curr_bg_tab -= 1;
                    }
                    mgr.set_graphic(bg_sel_id, curr_bg_tab, true);
                }
                //Key::Shift_Up
                Key::Ctrl_Right => {
                    let mut pos_changed = false;
                    if c < matrix_cols + 1 {
                        mgr.set_glyph(did, glyph_under_cursor, c, r);
                        pos_changed = true;
                        c += 1;
                    } else {
                        if r < matrix_rows + 1 {
                            mgr.set_glyph(did, glyph_under_cursor, c, r);
                            pos_changed = true;
                            c = 2;
                            r += 1;
                        }
                    }
                    if pos_changed {
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }
                Key::Left => {
                    if mc > 2 {
                        mc -= 1;
                        mgr.move_graphic(sid, 2, (-1, 0))
                    }
                }
                Key::Ctrl_Left => {
                    let mut pos_changed = false;
                    if c > 2 {
                        mgr.set_glyph(did, glyph_under_cursor, c, r);
                        pos_changed = true;

                        c -= 1;
                    } else {
                        if r > 2 {
                            pos_changed = true;
                            mgr.set_glyph(did, glyph_under_cursor, c, r);
                            c = matrix_cols + 1;
                            r -= 1;
                        }
                    }
                    if pos_changed {
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }
                Key::Up => {
                    if mr > 2 {
                        mr -= 1;
                        mgr.move_graphic(sid, 2, (0, -1))
                    }
                }
                Key::Ctrl_Up => {
                    if r > 2 {
                        mgr.set_glyph(did, glyph_under_cursor, c, r);
                        r -= 1;
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }
                Key::Down => {
                    if mr < 11 {
                        mr += 1;
                        mgr.move_graphic(sid, 2, (0, 1))
                    }
                }
                Key::Ctrl_Down => {
                    if r < matrix_rows + 1 {
                        mgr.set_glyph(did, glyph_under_cursor, c, r);
                        r += 1;
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }
                Key::Space => {
                    mgr.start_animation(sid, 0);
                    mgr.get_glyph(gmid, mc, mr);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                        mgr.set_glyph(did, glyph, c, r);
                        if c < matrix_cols + 1 {
                            c += 1;
                        } else if r < matrix_rows + 1 {
                            c = 2;
                            r += 1;
                        } else {
                            c = 2;
                            r = 2;
                        }
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }
                Key::Backspace => {
                    mgr.set_glyph(did, Glyph::default(), c, r);
                    if c > 2 {
                        c -= 1
                    } else {
                        r -= 1;
                        c = 5;
                    }
                    mgr.set_glyph(did, g, c, r);
                }
                Key::Escape | Key::Q | Key::Ctrl_q => {
                    keep_running = false;
                    break;
                }
                Key::i => mgr.set_invisible(col_sel_id, true),
                Key::I => mgr.set_invisible(col_sel_id, false),
                Key::Alt_i => mgr.set_invisible(bg_sel_id, true),
                Key::Alt_I => mgr.set_invisible(bg_sel_id, false),
                _ => {
                    continue;
                }
            }
        }
    }
    mgr.terminate();
}

struct Arguments {
    rows: Option<usize>,
    cols: Option<usize>,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            rows: None,
            cols: None,
        }
    }
}

enum ArgType {
    Rows,
    Cols,
}

enum WhatToParse {
    Name,
    Number,
}

fn parse_arguments() -> Arguments {
    let mut arguments = Arguments::default();
    let mut what_to_parse = WhatToParse::Name;
    let mut args = env::args();
    let mut program_name = args.next().unwrap();
    if let Some(value) = program_name.split("/").last() {
        program_name = value.to_string();
    };
    let mut name = None;
    let mut number;
    for arg in args {
        if arg == "--help" {
            println!("Usage:");
            println!(
                "{} --argument value",
                program_name.split("/").last().unwrap()
            );
            println!("\n Optional arguments:");
            println!(" --help - print this message");
            println!(
                " --rows <number> - how many rows should the screen consist of (at least {})",
                ROWS_MIN
            );
            println!(
                " --cols <number> - how many columns should be in each line (at least {})",
                COLS_MIN
            );
            exit(0)
        }
        match what_to_parse {
            WhatToParse::Name => {
                if arg.starts_with("--") {
                    name = match &arg[2..] {
                        "rows" => Some(ArgType::Rows),
                        "cols" => Some(ArgType::Cols),
                        &_ => None,
                    };
                    what_to_parse = WhatToParse::Number;
                } else {
                    eprintln!(
                        "\x1b[97;41;5mERR\x1b[m Expected argument name (e.g. --argument), got: {}",
                        arg
                    );
                    exit(1);
                }
            }
            WhatToParse::Number => {
                let parsed_number = usize::from_str_radix(arg.trim(), 10);
                match parsed_number {
                    Ok(a_number) => {
                        number = Some(a_number);
                        match &name {
                            Some(ArgType::Cols) => arguments.cols = number,
                            Some(ArgType::Rows) => arguments.rows = number,
                            None => {}
                        }
                    }
                    Err(_e) => {
                        eprintln!(
                            "\x1b[97;41;5mERR\x1b[m Expected integer value (e.g. 42), got: {}",
                            arg
                        );
                        exit(1);
                    }
                }
                what_to_parse = WhatToParse::Name;
                println!("Parsing for number: {}", number.unwrap());
            }
        }
    }
    arguments
}

fn verify_cols_and_rows(cols: Option<usize>, rows: Option<usize>) {
    if let Some(a_rows) = rows {
        if a_rows < ROWS_MIN {
            eprintln!(
                "\x1b[97;41;5mERR\x1b[m Min rows: {}, you provided: {}",
                ROWS_MIN, a_rows
            );
            exit(1)
        }
    }
    if let Some(a_cols) = cols {
        if a_cols < COLS_MIN {
            eprintln!(
                "\x1b[97;41;5mERR\x1b[m Min cols: {}, you provided: {}",
                COLS_MIN, a_cols
            );
            exit(1)
        }
    }
}

fn build_selector() -> Graphic {
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
    let mut gr = Graphic::new(3, 3, 0, library, Some(anims));
    gr
}

fn build_glyph_matrix(start_points: Vec<u32>) -> Graphic {
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
    let rows = start_points.len();
    let mut frame = Vec::with_capacity(cols * rows);
    let mut g = Glyph::default();
    g.set_color(Color::new_truecolor(48, 250, 13));
    g.set_background(Color::new_8bit(0, 5, 5));
    for sp in start_points {
        for i in 0..cols {
            g.set_char(char::from_u32(sp + i as u32).unwrap());
            //(sp + i as u32) as char
            frame.push(g.clone());
        }
    }
    library.insert(0, wrap_border_around(frame, cols, border, Some("Glyphs")));
    let mut gr = Graphic::new(cols + 2, rows + 2, 0, library, None);
    gr
}

fn build_empty_matrix(cols: usize, rows: usize) -> Graphic {
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
    let mut g = Glyph::default();
    let mut frame = vec![g; cols * rows];
    library.insert(
        0,
        wrap_border_around(frame, cols, border, Some("Workspace")),
    );
    let mut gr = Graphic::new(cols + 2, rows + 2, 0, library, None);
    gr
}

fn build_color_selector(title: Option<&str>) -> Graphic {
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
    let cols = 60;
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
    let mut gr = Graphic::new(cols + 2, rows + 2, 0, library, None);
    gr
}

fn build_basic_colors_graphic(mut selected: Glyph, mut deselected: Glyph) -> Graphic {
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

    let mut gr = Graphic::new(32, 2, 0, library, None);
    gr
}
