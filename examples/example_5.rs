use animaterm::{
    prelude::*,
    utilities::{message_box, progress_bar},
};
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::path::Path;
use std::process::exit;

static ROWS_MIN: usize = 4;
static COLS_MIN: usize = 5;

fn main() {
    let args = parse_arguments();
    let cols = args.cols;
    let rows = args.rows;
    verify_cols_and_rows(cols, rows);
    let mut g = Glyph::default();
    g.set_background(Color::green());
    g.set_char(char::from_u32(9626).unwrap());
    g.set_bright(true);

    let mut mgr = Manager::new(true, cols, rows, Some(g), None, None);
    let (cols, rows) = mgr.screen_size();

    let title = "Navigation help".to_string();
    let text = "CtrlPgUp,CtrlPgDn, -------------------------------------
AltPgUp,AltPgDn, ---------------------------------------
PgUp,PgDn, ---------------------------------------------
U,D, ---------------------------------------------------
CtrlU,CtrlD -------------------------------------------- 
AltU,AltD ----------------------------------------------
AltUp | AltK -------------------------------------------
CtrlUp | CtrlK -----------------------------------------
Up | K -------------------------------------------------
AltDown | AltJ -----------------------------------------
CtrlDown -----------------------------------------------
Down | J -----------------------------------------------
AltLeft | AltH -----------------------------------------
CtrlLeft | Backspace -----------------------------------
Left | H -----------------------------------------------
AltRight | AltL ----------------------------------------
CtrlRight | CtrlL --------------------------------------
Right | L - all above move graphics around the screen --
----------- and up/down layers. ------------------------
Tab - show a message box -------------------------------
Enter - type some text and hit Enter again -------------
        "
    .to_string();
    let mbox = message_box(Some(title), text, Glyph::default(), 60, 23);
    let mbid = mgr.add_graphic(mbox, 4, (1, 10)).unwrap();
    mgr.set_graphic(mbid, 0, true);
    let (gr, pid) = build_graphic(130, 10);
    let gr_layer = 1;
    let gid;
    let result = mgr.add_graphic(gr, gr_layer, (3, 15));
    if let Some(id) = result {
        gid = id;
    } else {
        eprintln!("Did not receive first graphic id");
        exit(2);
    }
    mgr.set_graphic(gid, pid, true);

    let pbid;
    let mut pb_layer = 3;
    let result = mgr.add_graphic(
        build_progress_bar(cols - 4),
        pb_layer,
        (2, (rows - 2) as isize),
    );
    if let Some(id) = result {
        pbid = id;
    } else {
        eprintln!("Did not receive progress bar graphic id");
        exit(2);
    }
    mgr.set_graphic(pbid, 0, true);
    mgr.start_animation(pbid, 0);
    let mut mbox_created = false;
    let mut mbox_id = 100;
    let mut mbox_layer = 2;
    if Path::new("index.txg").exists() {
        if let Some(id) =
            mgr.add_graphic(Graphic::from_file("index.txg").unwrap(), mbox_layer, (1, 0))
        {
            mgr.move_graphic(id, 2, (-1, 0));
            mbox_id = id;
            mbox_layer = 4;
        }
    } else {
        eprintln!("\x1b[97;41;5mERR\x1b[m Unable to load index.txg. Run this example from within top examples directory!");
        eprintln!("\x1b[97;41;5mERR\x1b[m Or copy *.txf and index.txg from there to current dir.");
        exit(1);
    }

    loop {
        if let Some(key) = mgr.read_key() {
            match key {
                Key::AltUnicode(uni) => match (uni[2], uni[4]) {
                    (53, 53) => {
                        //CtrlPgUp
                        mgr.move_graphic(1, 4, (0, 0));
                    }
                    (54, 53) => {
                        //CtrlPgDn
                        mgr.move_graphic(1, 1, (0, 0));
                    }
                    (53, 51) => {
                        //AltPgUp
                        mgr.move_graphic(2, 5, (0, 0));
                    }
                    (54, 51) => {
                        //AltPgDn
                        mgr.move_graphic(2, 2, (0, 0));
                    }
                    _ => {}
                },
                Key::PgUp | Key::U => {
                    mgr.move_graphic(0, 3, (0, 0));
                }
                Key::PgDn | Key::D => {
                    mgr.move_graphic(0, 0, (0, 0));
                }
                Key::CtrlU => {
                    pb_layer = 4;
                    mgr.move_graphic(pbid, pb_layer, (0, 0));
                }
                Key::CtrlD => {
                    mgr.move_graphic(pbid, pb_layer, (0, 0));
                }
                Key::AltU => {
                    mbox_layer = 5;
                    mgr.move_graphic(mbox_id, mbox_layer, (0, 0));
                }
                Key::AltD => {
                    mbox_layer = 2;
                    mgr.move_graphic(mbox_id, mbox_layer, (0, 0));
                }
                Key::AltUp | Key::AltK => {
                    mgr.move_graphic(mbox_id, mbox_layer, (0, -1));
                }
                Key::CtrlUp | Key::CtrlK => {
                    mgr.move_graphic(pbid, pb_layer, (0, -1));
                }
                Key::Up | Key::K => {
                    mgr.stop_animation(gid);
                    mgr.move_graphic(gid, 1, (0, -1));
                    mgr.set_graphic(gid, 1, true);
                }
                Key::AltDown | Key::AltJ => {
                    mgr.move_graphic(mbox_id, mbox_layer, (0, 1));
                }
                Key::CtrlDown => {
                    mgr.move_graphic(pbid, pb_layer, (0, 1));
                }
                Key::Down | Key::J => {
                    mgr.pause_animation_on_frame(pbid, 100);
                    mgr.move_graphic(gid, 1, (0, 1));
                    mgr.set_graphic(gid, pid, true);
                }
                Key::AltLeft | Key::AltH => {
                    mgr.move_graphic(mbox_id, mbox_layer, (-1, 0));
                }
                Key::CtrlLeft | Key::Backspace => {
                    mgr.move_graphic(pbid, pb_layer, (-1, 0));
                }
                Key::Left | Key::H => {
                    mgr.move_graphic(gid, 1, (-1, 0));
                    mgr.start_animation(gid, 0);
                    mgr.start_animation(pbid, 0);
                }
                Key::AltRight | Key::AltL => {
                    mgr.move_graphic(mbox_id, mbox_layer, (1, 0));
                }
                Key::CtrlRight | Key::CtrlL => {
                    mgr.move_graphic(pbid, pb_layer, (1, 0));
                }
                Key::Right | Key::L => {
                    mgr.move_graphic(gid, 1, (1, 0));
                }
                Key::Tab => {
                    mbox_layer += 1;
                    if !mbox_created {
                        if let  Some(mbid) = mgr.add_graphic(
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
                        mbox_layer,
                        (0, 0),
                    ){
                        mgr.set_graphic(mbid, 0, true);
                            mbox_created = true;
                            mbox_id = mbid;
                        }
                    }
                }
                Key::CtrlA => {
                    mgr.start_animation(gid, 0);
                }
                Key::CtrlB => {
                    mgr.stop_animation(gid);
                }
                Key::Insert => {
                    mgr.set_graphic(gid, 2, false);
                }
                Key::Delete => {}
                Key::Home => {
                    mgr.set_glyph(gid, Glyph::default(), 1, 1);
                    mgr.move_graphic(gid, 1, (0, 0));
                    mgr.empty_frame(gid);
                }
                Key::Escape | Key::Q | Key::CtrlQ => {
                    break;
                }
                Key::Enter => {
                    if !mbox_created {
                        mbox_layer += 1;
                        if let Some(tid) = mgr.add_graphic(
                            build_mbox(
                                40,
                                1,
                                "Please enter title and hit Enter".to_string(),
                                String::new(),
                            ),
                            mbox_layer,
                            (cols as isize / 2 - 20, rows as isize / 2),
                        ) {
                            mgr.set_graphic(tid, 0, true);
                            let line = mgr.read_line();
                            if let Some(mbid) = mgr.add_graphic(
                                build_mbox(
                                    60,
                                    20,
                                    line,
                                    "Hit Enter to type content in.".to_string(),
                                ),
                                mbox_layer,
                                (0, 0),
                            ) {
                                mgr.delete_graphic(tid);
                                mbox_id = mbid;
                                mgr.set_graphic(mbid, 0, true);
                                mbox_created = true;
                            }
                        }
                    } else {
                        let mut x = 1;
                        let mut y = 1;
                        mbox_created = false;
                        let mut rev = Glyph::default();
                        rev.set_reverse(true);
                        for i in 1..31 {
                            mgr.set_glyph(mbox_id, rev, i, 1);
                        }
                        loop {
                            if let Some(c) = mgr.read_char() {
                                if c == '\t' {
                                    break;
                                }
                                if c as u8 == 8 || c as u8 == 127 {
                                    x -= 1;
                                    if x == 0 {
                                        if y > 1 {
                                            y -= 1;
                                            x = 58;
                                        } else {
                                            x = 1;
                                        }
                                    }
                                    mgr.set_glyph(mbox_id, Glyph::default(), x, y);
                                    continue;
                                }
                                if c == '\n' {
                                    break;
                                }

                                mgr.set_glyph(mbox_id, Glyph::default_with_char(c), x, y);
                                x += 1;
                                if x > 58 {
                                    x = 1;
                                    y += 1;
                                }
                            }
                        }
                    }
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
            animaterm::Color::new_gray(0),
            animaterm::Color::new_gray(22),
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
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
        false,
        false,
    );
    progress_bar(
        length,
        Glyph::transparent(),
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
                false,
                false,
            ),
            // Glyph::new(
            //     '\u{2589}',
            //     animaterm::Color::new_truecolor(128, 0, 0),
            //     animaterm::Color::cyan(),
            //     false,
            //     true,
            //     false,
            //     false,
            //     false,
            //     false,
            //     false,
            //     false,
            //     false,
            //     false,
            // ),
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
