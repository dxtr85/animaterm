use animaterm::{
    prelude::*,
    utilities::{message_box, progress_bar},
};
use std::collections::HashMap;
use std::default::Default;
use std::env;
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

    let gl = Glyph::default();
    let (gr, pid) = build_graphic(130, 10);
    let gid = mgr.add_graphic(gr, 0, (3, 15));
    mgr.set_graphic(gid, pid, true);

    let pbid = mgr.add_graphic(build_progress_bar(cols - 4), 1, (2, rows - 2));
    mgr.set_graphic(pbid, 0, true);
    mgr.start_animation(pbid, 0);
    let mut mbox_created = false;

    let mut keep_running = true;
    let c_pgup = vec![27, 91, 53, 59, 53, 126];
    let c_pgdn = vec![27, 91, 54, 59, 53, 126];
    let a_pgup = vec![27, 91, 53, 59, 51, 126];
    let a_pgdn = vec![27, 91, 54, 59, 51, 126];
    while keep_running {
        let mut c: usize = 1;
        let mut r: usize = 1;
        if let Some(key) = mgr.read_key() {
            match key {
                Key::PgUp | Key::u => {
                    mgr.move_graphic(0, 3, (0, 0));
                }
                Key::PgDn | Key::d => {
                    mgr.move_graphic(0, 0, (0, 0));
                }
                Key::Alt_Unicode(c_pgup) => {
                    mgr.move_graphic(1, 4, (0, 0));
                }
                Key::Ctrl_u => {
                    mgr.move_graphic(1, 4, (0, 0));
                }
                Key::Alt_Unicode(c_pgdn) => {
                    mgr.move_graphic(1, 1, (0, 0));
                }
                Key::Ctrl_d => {
                    mgr.move_graphic(1, 1, (0, 0));
                }
                Key::Alt_Unicode(a_pgup) => {
                    mgr.move_graphic(2, 5, (0, 0));
                }
                Key::Alt_u => {
                    mgr.move_graphic(2, 5, (0, 0));
                }
                Key::Alt_Unicode(a_pgdn) => {
                    mgr.move_graphic(2, 2, (0, 0));
                }
                Key::Alt_d => {
                    mgr.move_graphic(2, 2, (0, 0));
                }
                Key::Alt_Up | Key::Alt_k => {
                    mgr.move_graphic(2, 2, (0, -1));
                }
                Key::Ctrl_Up | Key::Ctrl_k => {
                    mgr.move_graphic(1, 1, (0, -1));
                }
                Key::Up | Key::k => {
                    mgr.stop_animation(gid);
                    mgr.move_graphic(gid, 0, (0, -1));
                    mgr.set_graphic(gid, 0, true);
                    //mgr.set_graphic(pbid, 0, true);
                    c += 1;
                }
                Key::Alt_Down | Key::Alt_j => {
                    mgr.move_graphic(2, 2, (0, 1));
                }
                Key::Ctrl_Down | Key::Enter => {
                    mgr.move_graphic(1, 1, (0, 1));
                }
                Key::Down | Key::j => {
                    mgr.pause_animation_on_frame(pbid, 100);
                    mgr.move_graphic(gid, 0, (0, 1));
                    mgr.set_graphic(gid, pid, true);
                    //mgr.set_graphic(pbid, 1, true);
                    r += 1;
                }
                Key::Alt_Left | Key::Alt_h => {
                    mgr.move_graphic(2, 2, (-1, 0));
                }
                Key::Ctrl_Left | Key::Backspace => {
                    mgr.move_graphic(1, 1, (-1, 0));
                }
                Key::Left | Key::h => {
                    mgr.move_graphic(gid, 0, (-1, 0));
                    mgr.start_animation(gid, 0);
                    mgr.start_animation(pbid, 0);
                    //mgr.set_graphic(pbid, 2, true);
                    c.saturating_sub(1);
                }
                Key::Alt_Right | Key::Alt_l => {
                    mgr.move_graphic(2, 2, (1, 0));
                }
                Key::Ctrl_Right | Key::Ctrl_l => {
                    mgr.move_graphic(1, 1, (1, 0));
                }
                Key::Right | Key::l => {
                    mgr.move_graphic(gid, 0, (1, 0));
                    //mgr.start_animation(anim_id);
                    //mgr.set_graphic(pbid, 3, true);
                    r.saturating_sub(1);
                }
                Key::Tab => {
                    if !mbox_created {
                        let mbid = mgr.add_graphic(
                            build_mbox(60, 20, "La ku ka ra cza ga wi ga ba ga da da da ja nie".to_string(),
                            "lastBuildDate: Tue, 12 Apr 2022 07:52:44 GMT
* generator: Oddmuse
* copyright: This work is licensed to you under version 2 of the GNU General Public License. Alternatively, you may choose to receive this work under
 any other license that grants the right to use, copy, modify, and distribute the work, as long as that license imposes the restriction that derivative
 works have to grant the same rights and impose the same restriction. For example, you may choose to receive this work under the GNU Free
 Documentation License, the CreativeCommons ShareAlike License, the XEmacs manual license, or similar licenses.
* license: https://creativecommons.org/licenses/sa/1.0/
* license: https://www.gnu.org/copyleft/fdl.html
* license: https://www.gnu.org/copyleft/gpl.html".to_string()                        ),
                        2,
                        (20, 10),
                    );
                        mgr.set_graphic(mbid, 0, true);
                        mbox_created = true;
                    }
                }
                Key::Escape | Key::Q | Key::Ctrl_q => {
                    keep_running = false;
                    break;
                }
                Key::Ctrl_a => {
                    mgr.start_animation(gid, 0);
                }
                Key::Ctrl_b => {
                    mgr.stop_animation(gid);
                }
                Key::Insert => {
                    // mgr.pause_animation(anim_id);
                    mgr.set_graphic(gid, 2, false);
                }
                Key::Delete => {
                    // mgr.stop_animation(anim_id);
                    c = 1;
                    r = 1;
                }
                Key::Home => {
                    //mgr.restart_animation(anim_id);
                    mgr.set_glyph(gid, Glyph::default(), c, r);
                    mgr.move_graphic(gid, 0, (0, 0));
                    mgr.empty_frame(gid);
                }
                _ => {
                    println!("You pressed: {:?}", key);
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
    // Unknown,
}

enum WhatToParse {
    Name,
    Number,
    // Text,
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
    let mut number; // = None;
                    // let text: Option<String> = None;
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
                    // println!("Parsing for name: {}", name.unwrap());
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
            } // WhatToParse::Text => {
              //     println!("Parsing for text: {}", arg);
              // }
        }
    }
    arguments
}

fn build_graphic(cols: usize, rows: usize) -> (Graphic, usize) {
    let start_frame = 0;
    let mut library = HashMap::with_capacity(2);
    library.insert(
        start_frame,
        vec![
            Glyph::new(
                '\u{2580}',
                animaterm::Color::new_8bit(0, 5, 0),
                animaterm::Color::new_8bit(0, 0, 5),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            );
            rows * cols
        ],
    );
    let mut animations = HashMap::new();
    animations.insert(
        0,
        Animation::new(
            false,
            true,
            vec![(1, Timestamp::new(0, 500)), (0, Timestamp::new(0, 500))],
            Timestamp::now(),
        ),
    );

    let mut gr = Graphic::new(cols, rows, start_frame, library, Some(animations));
    let pid = gr
        .add_to_library(vec![
            Glyph::new(
                '\u{2580}',
                animaterm::Color::new_truecolor(0, 255, 255),
                animaterm::Color::new_truecolor(0, 0, 255),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            );
            rows * cols
        ])
        .unwrap();
    (gr, pid)
}

fn build_mbox(cols: usize, rows: usize, title: String, content: String) -> Graphic {
    message_box(
        Some(title),
        content,
        Glyph::new(
            ' ',
            animaterm::Color::new_gray(22),
            animaterm::Color::new_gray(0),
            false,
            false,
            false,
            false,
            false,
            false,
            true,
            false,
        ),
        cols,
        rows,
    )
}

fn build_progress_bar(length: usize) -> Graphic {
    let glf = Glyph::new(
        '\u{2588}',
        animaterm::Color::new(ColorName::Red),
        animaterm::Color::new(ColorName::White),
        false,
        true,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    progress_bar(
        length,
        Glyph::plain(),
        glf,
        Some(vec![
            Glyph::new(
                '\u{258F}',
                animaterm::Color::red(),
                animaterm::Color::cyan(),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            Glyph::new(
                '\u{258E}',
                animaterm::Color::red(),
                animaterm::Color::cyan(),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            Glyph::new(
                '\u{258D}',
                animaterm::Color::red(),
                animaterm::Color::cyan(),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            Glyph::new(
                '\u{258C}',
                animaterm::Color::red(),
                animaterm::Color::cyan(),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            Glyph::new(
                '\u{258B}',
                animaterm::Color::red(),
                animaterm::Color::cyan(),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            Glyph::new(
                '\u{258A}',
                animaterm::Color::red(),
                animaterm::Color::cyan(),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            Glyph::new(
                '\u{2589}',
                animaterm::Color::new_truecolor(128, 0, 0),
                animaterm::Color::cyan(),
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
        ]),
    )
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
