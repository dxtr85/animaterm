use animaterm::{message_box, progress_bar, Animation, Color, Glyph, Graphic, Manager, Timestamp};
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::env;
use std::io;
use std::io::Read;
// use std::io::Write;
// use std::ops::{Shl, Shr};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

static ROWS_MIN: usize = 4;
static COLS_MIN: usize = 5;

fn main() {
    let args = parse_arguments();
    let cols = args.cols;
    let rows = args.rows;
    verify_cols_and_rows(cols, rows);
    let gl = Glyph::default();

    let (mut mgr, keys) = Manager::new(cols, rows, None);
    let (cols, rows) = mgr.screen_size();
    let anim_id = mgr.add_animation(build_animation_one(gl, cols, rows), 0, (0, 0));
    let anim2_id = mgr.add_animation(build_animation_two(gl, cols, rows), 0, (0, 0));
    let anim3_id = mgr.add_animation(build_animation_three(gl, cols, rows), 0, (0, 0));

    let (gr, pid) = build_graphic(130, 10, None, None);
    sleep(Duration::from_secs(1));
    let gid = mgr.add_graphic(gr, 2, (3, 15));

    let mut keep_running = true;
    let mut key_iter = keys.into_iter();
    while keep_running {
        while let Some(key) = key_iter.next() {
            match key {
                27 => {
                    keep_running = false;
                    break;
                }
                97 => mgr.set_graphic(gid, pid),
                98 => mgr.set_graphic(gid, pid + 1),
                99 => {
                    mgr.delete_graphic(3);
                    //     mgr.add_graphic(
                    //         progress_bar(
                    //             20,
                    //             Glyph::default(),
                    //             Glyph::new(
                    //                 '\u{2580}',
                    //                 Color::White,
                    //                 Color::Red,
                    //                 false,
                    //                 true,
                    //                 false,
                    //                 false,
                    //                 false,
                    //                 false,
                    //                 false,
                    //                 false,
                    //             ),
                    //             None,
                    //         ),
                    //         2,
                    //         (0, 0),
                    //     )
                }
                100 => mgr.start_animation(anim_id),
                101 => mgr.pause_animation(anim_id),
                102 => mgr.stop_animation(anim_id),
                103 => mgr.restart_animation(anim_id),
                104 => mgr.start_animation(anim2_id),
                105 => mgr.pause_animation(anim2_id),
                106 => mgr.stop_animation(anim2_id),
                107 => mgr.restart_animation(anim2_id),
                108 => mgr.start_animation(anim3_id),
                109 => mgr.pause_animation(anim3_id),
                110 => mgr.stop_animation(anim3_id),
                111 => mgr.restart_animation(anim3_id),
                _ => continue,
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
        gl.set_background(c);
        gl.set_color(colors[(i + 1) % 7]);
        ordering.push(((i + 1) % 7, Timestamp::new(0, 200)));
        frames.insert(
            i,
            message_box(
                Some("\u{2660} \u{2665} \u{2666} \u{2663} ???? Mesyd?? in a baaaato".to_string()),
                "    Bardzo niespodziewane wie??ci przekazuje po Grand Prix Azerbejd??anu
dziennikarz Joe Saward. Wed??ug jego doniesie??, do Formu??y 1 mo??e wr??ci??
Antonio Giovinazzi. Wszyscy za spraw?? decyzji Ferrari.

Antonio Giovinazzi odszed?? z Formu??y 1 pod koniec ubieg??ego roku, trac??c miejsce w
Alfie Romeo, gdzie pojawili si?? Valtteri Bottas i Guanyu Zhou. W??och znalaz?? miejsce
w ekipie Dragon Penske, jednej z najs??abszych w stawce i po 9 rundach sezonu
zajmuje ostatnie miejsce z zerowym dorobkiem punktowym.

Antonio by?? niedawno pytany o swoj?? przysz??o????, jednak powiedzia??, ??e jest ona
otwarta i nie wie, czy zostanie w Formule E.

Giovinazzi w Formule 1 pe??ni rol?? kierowcy rezerwowego Ferrari i do???? cz??sto
pojawia si?? na wy??cigach. Pracuje tak??e sporo ??? r??wnie?? podczas weekend??w
wy??cigowych ??? w symulatorze ekipy.

 Podobno ??wietne wy??cigowe ustawienia Ferrari w Australii by??y zas??ug??
 ??wietnej pracy w symulatorze w Maranello. 

 Sp??jrzmy zatem, kto jest Stigiem z symulatora:#F1 #F1pl #elevenf1
 pic.twitter.com/hRVqGgWYp1

 ??? Powr??t Roberta (@powrotroberta) April 21, 2022

Obecnie W??osi nie maj?? swojego kierowcy w F1 i dla Ferrari wa??ne by??oby zmienienie
tego. Ekipa z Maranello nie ma ju?? du??ych wp??yw??w w Alfie Romeo i nie mo??e
decydowa?? o obsadzie foteli w Hinwil. Inaczej jest jednak w Haasie, kt??ry w ostatnich
latach mocno zacie??ni?? wsp????prac?? z Ferrari.

Jak pisze w swoim blogu po Grand Prix Azerbejd??anu Joe Saward, Haas szuka
kierowcy do swojej ekipy za Micka Schumachera, notuj??cego fatalne wyniki.

???Amerykanie mogliby by?? zainteresowani Oscarem Piastrim, ale prawda jest taka, ??e
Ferrari ma g??os w sprawie obsady drugiego kierowcy Haasa i jako ??e Mick
Schumacher nie spisuje si?? dobrze, m??wi si??, ??e Haas prawdopodobnie sko??czy w
2023 roku z rezerwowym kierowc?? Ferrari, Antonio Giovinazzim??? ??? pisze Saward.

Nie by??aby to raczej du??a zmiana jako??ciowa, gdy?? Giovinazzi w swoich startach w F1
spisywa?? si?? s??abo. W 62 wy??cigach zdoby?? 21 punkt??w, a jego partner zespo??owy,
Kimi Raikkonen w 60 startach zebra?? 57 punkt??w, a przecie?? najlepsze lata ma ju?? za
sob??.

Ferrari nie ma obecnie mocnej akademii juniorskiej co by??o r??wnie?? powodem
przed??u??enia kontraktu z Carlosem Sainzem do 2024 roku. To szansa dla
Giovinazziego je??eli chce jeszcze pojecha?? w F1."
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
    gl.set_color(Color::Blue);
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
        gl.set_background(c);
        ordering.push(((i + 1) % 7, Timestamp::new(0, 100)));
        frames.insert(
            i,
            message_box(
                Some("\u{2660} \u{2665} \u{2666} \u{2663} awend??ed sewenfold".to_string()),
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

fn build_animation_three(mut gl: Glyph, cols: usize, rows: usize) -> Animation {
    let mut frames = HashMap::new();
    let mut ordering = vec![];
    gl.set_color(Color::White);
    gl.set_background(Color::Blue);
    gl.set_blink(true);
    ordering.push((1, Timestamp::new(0, 100)));
    frames.insert(
        0,
        message_box(
            Some("??I".to_string()),
            "J U u u u u u u u".to_string(),
            gl.clone(),
            1,
            1,
            6,
            rows.saturating_sub(5),
        ),
    );
    gl.set_background(Color::Red);
    ordering.push((2, Timestamp::new(0, 100)));
    frames.insert(
        1,
        message_box(
            Some("JU".to_string()),
            "?? I i i i i i i i".to_string(),
            gl.clone(),
            1,
            1,
            6,
            rows.saturating_sub(5),
        ),
    );
    ordering.push((3, Timestamp::new(0, 100)));
    frames.insert(
        2,
        message_box(
            Some("??I".to_string()),
            "J U u u u u u u u".to_string(),
            gl.clone(),
            cols.saturating_sub(5),
            1,
            6,
            rows.saturating_sub(5),
        ),
    );
    gl.set_background(Color::Blue);
    ordering.push((0, Timestamp::new(0, 100)));
    frames.insert(
        3,
        message_box(
            Some("JU".to_string()),
            "?? I i i i i i i i".to_string(),
            gl.clone(),
            cols.saturating_sub(5),
            1,
            6,
            rows.saturating_sub(5),
        ),
    );
    Animation::new(frames, true, true, ordering, Timestamp::new(0, 500))
}

fn build_graphic(
    cols: usize,
    rows: usize,
    glyphs: Option<Vec<Glyph>>,
    library: Option<HashMap<usize, Vec<Glyph>>>,
) -> (Graphic, usize) {
    let mut gr = Graphic::new(cols, rows, glyphs, library, None);
    let pid = gr
        .add_to_library(vec![
            Glyph::new(
                '\u{2580}',
                Color::White,
                Color::Red,
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            );
            1300
        ])
        .unwrap();
    let _pid2 = gr
        .add_to_library(vec![
            Glyph::new(
                '\u{2580}',
                Color::Yellow,
                Color::Blue,
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            );
            1300
        ])
        .unwrap();
    (gr, pid)
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
