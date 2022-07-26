use animaterm::{
    prelude::*,
    utilities::{message_box, new_message_box, progress_bar},
};
use std::collections::HashMap;
use std::default::Default;
use std::env;
// use std::io;
// use std::io::Read;
// use std::io::Write;
// use std::ops::{Shl, Shr};
use std::process::exit;

static ROWS_MIN: usize = 4;
static COLS_MIN: usize = 5;

fn main() {
    let args = parse_arguments();
    let cols = args.cols;
    let rows = args.rows;
    verify_cols_and_rows(cols, rows);
    let mut mgr = Manager::new(true, cols, rows, None);
    //let mut key_iter = mgr.get_key_receiver().unwrap().into_iter();
    let mut result_iter = mgr.get_result_iter().unwrap();
    let (cols, rows) = mgr.screen_size();
    // let mut results = vec![];

    let gl = Glyph::default();
    // let anim_id = mgr.add_animation(build_animation_one(gl, cols, rows), 0, (0, 0));
    // results.push(result_iter.next());
    // let anim2_id = mgr.add_animation(build_animation_two(gl, cols, rows), 0, (0, 0));
    // results.push(result_iter.next());
    // let anim3_id = mgr.add_animation(build_animation_three(gl, cols, rows), 0, (0, 0));
    // results.push(result_iter.next());
    let (gr, pid) = build_graphic(130, 10);
    let gid = mgr.add_graphic(gr, 2, (3, 15));
    mgr.set_graphic(gid, pid, true);

    let pbid = mgr.add_graphic(build_progress_bar(cols - 4), 2, (2, rows - 2));
    mgr.set_graphic(pbid, 0, true);
    mgr.new_start_animation(pbid, 0);
    let mut mbox_created = false;

    let mut keep_running = true;
    while keep_running {
        let mut c: usize = 1;
        let mut r: usize = 1;
        // while let Some(key) = key_iter.next() {
        if let Some(key) = mgr.read_key() {
            match key {
                Key::Alt_Up | Key::Alt_k => {
                    mgr.move_graphic(2, 3, (0, -1));
                }
                Key::Ctrl_Up | Key::Ctrl_k => {
                    mgr.move_graphic(1, 2, (0, -1));
                }
                Key::Up | Key::k => {
                    mgr.new_stop_animation(gid);
                    mgr.move_graphic(gid, 2, (0, -1));
                    mgr.set_graphic(gid, 0, true);
                    //mgr.set_graphic(pbid, 0, true);
                    c += 1;
                }
                Key::Alt_Down | Key::Alt_j => {
                    mgr.move_graphic(2, 3, (0, 1));
                }
                Key::Ctrl_Down | Key::Enter => {
                    mgr.move_graphic(1, 2, (0, 1));
                }
                Key::Down | Key::j => {
                    mgr.pause_animation_on_frame(pbid, 0, 100);
                    mgr.move_graphic(gid, 2, (0, 1));
                    mgr.set_graphic(gid, pid, true);
                    //mgr.set_graphic(pbid, 1, true);
                    r += 1;
                }
                Key::Alt_Left | Key::Alt_h => {
                    mgr.move_graphic(2, 3, (-1, 0));
                }
                Key::Ctrl_Left | Key::Backspace => {
                    mgr.move_graphic(1, 2, (-1, 0));
                }
                Key::Left | Key::h => {
                    mgr.move_graphic(gid, 2, (-1, 0));
                    mgr.new_start_animation(gid, 0);
                    mgr.new_start_animation(pbid, 0);
                    //mgr.set_graphic(pbid, 2, true);
                    c.saturating_sub(1);
                }
                Key::Alt_Right | Key::Alt_l => {
                    mgr.move_graphic(2, 3, (1, 0));
                }
                Key::Ctrl_Right | Key::Ctrl_l => {
                    mgr.move_graphic(1, 2, (1, 0));
                }
                Key::Right | Key::l => {
                    mgr.move_graphic(gid, 2, (1, 0));
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
                        3,
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
                    mgr.new_start_animation(gid, 0);
                }
                Key::Ctrl_b => {
                    mgr.new_stop_animation(gid);
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
                    mgr.move_graphic(gid, 2, (0, 0));
                    //mgr.empty_frame(gid);
                }
                _ => {
                    println!("You pressed: {:?}", key);
                }
            }
        }
    }
    mgr.terminate();
    //    println!("Added Animation IDs: {:?}", results);
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

fn build_animation_one(mut gl: Glyph, cols: usize, rows: usize) -> Animation {
    let colors = [
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::Red,
    ];
    let mut ordering = Vec::new();
    let mut frames = HashMap::new();
    let mut i = 0;
    for c in colors {
        gl.set_background(animaterm::NewColor::Basic(c));
        gl.set_color(animaterm::NewColor::Basic(colors[(i + 1) % 7]));
        ordering.push(((i + 1) % 7, Timestamp::new(0, 200)));
        frames.insert(
            i,
            message_box(
                Some("\u{2660} \u{2665} \u{2666} \u{2663} 🖕 Mesydż in a baaaato".to_string()),
                "    Bardzo niespodziewane wieści przekazuje po Grand Prix Azerbejdżanu
dziennikarz Joe Saward. Według jego doniesień, do Formuły 1 może wrócić
Antonio Giovinazzi. Wszyscy za sprawą decyzji Ferrari.

Antonio Giovinazzi odszedł z Formuły 1 pod koniec ubiegłego roku, tracąc miejsce w
Alfie Romeo, gdzie pojawili się Valtteri Bottas i Guanyu Zhou. Włoch znalazł miejsce
w ekipie Dragon Penske, jednej z najsłabszych w stawce i po 9 rundach sezonu
zajmuje ostatnie miejsce z zerowym dorobkiem punktowym.

Antonio był niedawno pytany o swoją przyszłość, jednak powiedział, że jest ona
otwarta i nie wie, czy zostanie w Formule E.

Giovinazzi w Formule 1 pełni rolę kierowcy rezerwowego Ferrari i dość często
pojawia się na wyścigach. Pracuje także sporo – również podczas weekendów
wyścigowych – w symulatorze ekipy.

 Podobno świetne wyścigowe ustawienia Ferrari w Australii były zasługą
 świetnej pracy w symulatorze w Maranello. 

 Spójrzmy zatem, kto jest Stigiem z symulatora:#F1 #F1pl #elevenf1
 pic.twitter.com/hRVqGgWYp1

 — Powrót Roberta (@powrotroberta) April 21, 2022

Obecnie Włosi nie mają swojego kierowcy w F1 i dla Ferrari ważne byłoby zmienienie
tego. Ekipa z Maranello nie ma już dużych wpływów w Alfie Romeo i nie może
decydować o obsadzie foteli w Hinwil. Inaczej jest jednak w Haasie, który w ostatnich
latach mocno zacieśnił współpracę z Ferrari.

Jak pisze w swoim blogu po Grand Prix Azerbejdżanu Joe Saward, Haas szuka
kierowcy do swojej ekipy za Micka Schumachera, notującego fatalne wyniki.

„Amerykanie mogliby być zainteresowani Oscarem Piastrim, ale prawda jest taka, że
Ferrari ma głos w sprawie obsady drugiego kierowcy Haasa i jako że Mick
Schumacher nie spisuje się dobrze, mówi się, że Haas prawdopodobnie skończy w
2023 roku z rezerwowym kierowcą Ferrari, Antonio Giovinazzim” – pisze Saward.

Nie byłaby to raczej duża zmiana jakościowa, gdyż Giovinazzi w swoich startach w F1
spisywał się słabo. W 62 wyścigach zdobył 21 punktów, a jego partner zespołowy,
Kimi Raikkonen w 60 startach zebrał 57 punktów, a przecież najlepsze lata ma już za
sobą.

Ferrari nie ma obecnie mocnej akademii juniorskiej co było również powodem
przedłużenia kontraktu z Carlosem Sainzem do 2024 roku. To szansa dla
Giovinazziego jeżeli chce jeszcze pojechać w F1."
                    .to_string(),
                gl.clone(),
                7 + 2 * i,
                1,
                cols.saturating_sub(12 + (4 * i)),
                rows.saturating_sub(5 + (2 * i)),
            ),
        );
        i += 1;
    }
    Animation::new(frames, false, false, ordering, Timestamp::new(0, 100))
}

fn build_animation_two(mut gl: Glyph, cols: usize, rows: usize) -> Animation {
    let mut frames = HashMap::new();
    let mut ordering = vec![];
    let mut i = 0;
    gl.set_color(animaterm::NewColor::Basic(Color::Blue));
    let colors = [
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::Red,
    ];

    for c in colors {
        gl.set_background(animaterm::NewColor::Basic(c));
        ordering.push(((i + 1) % 7, Timestamp::new(0, 100)));
        frames.insert(
            i,
            message_box(
                Some("\u{2660} \u{2665} \u{2666} \u{2663} awendżed sewenfold".to_string()),
                "Dooo pah".to_string(),
                gl.clone(),
                1,
                rows - 4, //(3 * (1 + i)),
                cols,
                5,
            ),
        );
        i += 1;
    }

    Animation::new(frames, true, true, ordering, Timestamp::new(0, 500))
}

fn build_graphic(cols: usize, rows: usize) -> (Graphic, usize) {
    let start_frame = 0;
    let mut library = HashMap::with_capacity(2);
    library.insert(
        start_frame,
        vec![
            Glyph::new(
                '\u{2580}',
                animaterm::NewColor::new8Bit(0, 5, 0),
                animaterm::NewColor::new8Bit(0, 0, 5),
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
            HashMap::new(),
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
                animaterm::NewColor::newTruecolor(0, 255, 255),
                animaterm::NewColor::newTruecolor(0, 0, 255),
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
    new_message_box(
        Some(title),
        content,
        Glyph::new(
            ' ',
            animaterm::NewColor::newGray(22),
            animaterm::NewColor::newGray(0),
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
        animaterm::NewColor::new(Color::Red),
        animaterm::NewColor::new(Color::White),
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
                animaterm::NewColor::red(),
                animaterm::NewColor::cyan(),
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
                animaterm::NewColor::red(),
                animaterm::NewColor::cyan(),
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
                animaterm::NewColor::red(),
                animaterm::NewColor::cyan(),
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
                animaterm::NewColor::red(),
                animaterm::NewColor::cyan(),
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
                animaterm::NewColor::red(),
                animaterm::NewColor::cyan(),
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
                animaterm::NewColor::red(),
                animaterm::NewColor::cyan(),
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
                animaterm::NewColor::newTruecolor(128, 0, 0),
                animaterm::NewColor::cyan(),
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
