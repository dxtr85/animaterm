use animaterm::{str_to_key, Key};
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::exit;

static ROWS_MIN: usize = 29;
static COLS_MIN: usize = 84;

pub struct Arguments {
    pub rows: Option<usize>,
    pub cols: Option<usize>,
    pub colors_offset: Option<(isize, isize)>,
    pub backgrounds_offset: Option<(isize, isize)>,
    pub styles_offset: Option<(isize, isize)>,
    pub glyphs_offset: Option<(isize, isize)>,
    pub workspace_offset: Option<(isize, isize)>,
    pub workspace_size: Option<(usize, usize)>,
    pub config_file: Option<String>,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub glyphs: Option<String>,
    pub bindings: Bindings,
}

pub struct Bindings {
    pub colors_left: Vec<Key>,
    pub colors_right: Vec<Key>,
    pub colors_far_right: Vec<Key>,
    pub colors_far_left: Vec<Key>,
    pub colors_top: Vec<Key>,
    pub colors_up: Vec<Key>,
    pub colors_down: Vec<Key>,
    pub colors_bottom: Vec<Key>,
    pub colors_invisible: Vec<Key>,
    pub colors_visible: Vec<Key>,
    pub backgrounds_left: Vec<Key>,
    pub backgrounds_right: Vec<Key>,
    pub backgrounds_far_right: Vec<Key>,
    pub backgrounds_far_left: Vec<Key>,
    pub backgrounds_top: Vec<Key>,
    pub backgrounds_up: Vec<Key>,
    pub backgrounds_down: Vec<Key>,
    pub backgrounds_bottom: Vec<Key>,
    pub backgrounds_invisible: Vec<Key>,
    pub backgrounds_visible: Vec<Key>,
    pub glyphs_left: Vec<Key>,
    pub glyphs_right: Vec<Key>,
    pub glyphs_up: Vec<Key>,
    pub glyphs_down: Vec<Key>,
    pub glyphs_select: Vec<Key>,
    pub glyphs_prev: Vec<Key>,
    pub glyphs_next: Vec<Key>,
    pub glyphs_home: Vec<Key>,
    pub glyphs_end: Vec<Key>,
    pub workspace_left: Vec<Key>,
    pub workspace_right: Vec<Key>,
    pub workspace_up: Vec<Key>,
    pub workspace_down: Vec<Key>,
    pub workspace_line_start: Vec<Key>,
    pub workspace_line_end: Vec<Key>,
    pub workspace_select_color: Vec<Key>,
    pub workspace_select_background: Vec<Key>,
    pub workspace_select_glyph: Vec<Key>,
    pub workspace_erase: Vec<Key>,
    pub style_up: Vec<Key>,
    pub style_down: Vec<Key>,
    pub style_enable: Vec<Key>,
    pub style_disable: Vec<Key>,
    pub print_graphic: Vec<Key>,
    pub print_screen: Vec<Key>,
    pub exit: Vec<Key>,
}

impl Default for Bindings {
    fn default() -> Self {
        Bindings {
            colors_left: vec![Key::ShiftLeft],
            colors_right: vec![Key::ShiftRight],
            colors_far_right: vec![Key::CtrlShiftRight],
            colors_far_left: vec![Key::CtrlShiftLeft],
            colors_top: vec![Key::CtrlShiftUp],
            colors_up: vec![Key::ShiftUp],
            colors_down: vec![Key::ShiftDown],
            colors_bottom: vec![Key::CtrlShiftDown],
            colors_invisible: vec![Key::I],
            colors_visible: vec![Key::ShiftI],
            backgrounds_left: vec![Key::AltLeft],
            backgrounds_right: vec![Key::AltRight],
            backgrounds_far_right: vec![Key::AltCtrlRight],
            backgrounds_far_left: vec![Key::AltCtrlLeft],
            backgrounds_top: vec![Key::AltCtrlUp],
            backgrounds_up: vec![Key::AltUp],
            backgrounds_down: vec![Key::AltDown],
            backgrounds_bottom: vec![Key::AltCtrlDown],
            backgrounds_invisible: vec![Key::AltI],
            backgrounds_visible: vec![Key::AltShiftI],
            glyphs_left: vec![Key::CtrlLeft],
            glyphs_right: vec![Key::CtrlRight],
            glyphs_up: vec![Key::CtrlUp],
            glyphs_down: vec![Key::CtrlDown],
            glyphs_select: vec![Key::Space],
            glyphs_prev: vec![Key::PgUp],
            glyphs_next: vec![Key::PgDn],
            glyphs_home: vec![Key::Home],
            glyphs_end: vec![Key::End],
            workspace_left: vec![Key::Left],
            workspace_right: vec![Key::Right],
            workspace_up: vec![Key::Up],
            workspace_down: vec![Key::Down],
            workspace_line_start: vec![Key::CtrlA],
            workspace_line_end: vec![Key::CtrlE],
            workspace_select_color: vec![Key::C],
            workspace_select_background: vec![Key::B],
            workspace_select_glyph: vec![Key::G],
            workspace_erase: vec![Key::Delete],
            style_up: vec![Key::AltShiftUp],
            style_down: vec![Key::AltShiftDown],
            style_enable: vec![Key::AltShiftRight],
            style_disable: vec![Key::AltShiftLeft],
            print_graphic: vec![Key::AltP],
            print_screen: vec![Key::AltCtrlP],
            exit: vec![Key::Escape],
        }
    }
}
impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            rows: None,
            cols: None,
            colors_offset: None,
            backgrounds_offset: None,
            styles_offset: None,
            glyphs_offset: None,
            workspace_offset: None,
            workspace_size: None,
            config_file: None,
            input_file: None,
            output_file: None,
            glyphs: None,
            bindings: Bindings::default(),
        }
    }
}

enum ArgType {
    Rows,
    Cols,
    ColorsOffset,
    BackgroundsOffset,
    StylesOffset,
    GlyphsOffset,
    WorkspaceOffset,
    WorkspaceSize,
    ConfigFile,
    InputFile,
    OutputFile,
    Glyphs,
}

enum WhatToParse {
    Name,
    Number,
    NumberPair,
}

pub fn parse_arguments() -> Arguments {
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
                "{} [--argument [value]]",
                program_name.split("/").last().unwrap()
            );
            println!("\n Optional arguments:");
            println!(" --help - print this message");
            println!(" --config_file <path to config file> - load config from file");
            println!(
                " --rows <number> - how many rows should the screen consist of (at least {})",
                ROWS_MIN
            );
            println!(
                " --cols <number> - how many columns should be in each line (at least {})",
                COLS_MIN
            );
            println!(
                " --colors_offset <number>x<number> - where should Colors window be placed (i.e 0x0)"
            );
            println!(
                " --backgrounds_offset <number>x<number> - where should Backgrounds window be placed"
            );
            println!(" --styles_offset <number>x<number> - where should Styles window be placed");
            println!(" --glyphs_offset <number>x<number> - where should Glyphs window be placed");
            println!(
                " --workspace_offset <number>x<number> - where should Workspace window be placed"
            );
            println!(
                " --workspace_size <number>x<number> - Width and Height of Workspace's interior (i.e 20x10)"
            );
            println!(" --input_file <file_name> - Read a frame into workspace from file");
            println!(" --output_file <file_name> - Write a workspace frame into file");
            println!(
                " --glyphs <filename> - index file containing filenames with glyph definitions, each filename in separate line");
            exit(0)
        }
        match what_to_parse {
            WhatToParse::Name => {
                if arg.starts_with("--") {
                    what_to_parse = WhatToParse::Number;
                    name = match &arg[2..] {
                        "rows" => Some(ArgType::Rows),
                        "cols" => Some(ArgType::Cols),
                        "colors_offset" => {
                            what_to_parse = WhatToParse::NumberPair;
                            Some(ArgType::ColorsOffset)
                        }
                        "backgrounds_offset" => {
                            what_to_parse = WhatToParse::NumberPair;
                            Some(ArgType::BackgroundsOffset)
                        }
                        "styles_offset" => {
                            what_to_parse = WhatToParse::NumberPair;
                            Some(ArgType::StylesOffset)
                        }
                        "glyphs_offset" => {
                            what_to_parse = WhatToParse::NumberPair;
                            Some(ArgType::GlyphsOffset)
                        }
                        "workspace_offset" => {
                            what_to_parse = WhatToParse::NumberPair;
                            Some(ArgType::WorkspaceOffset)
                        }
                        "workspace_size" => {
                            what_to_parse = WhatToParse::NumberPair;
                            Some(ArgType::WorkspaceSize)
                        }
                        "config_file" => {
                            what_to_parse = WhatToParse::Name;
                            Some(ArgType::ConfigFile)
                        }
                        "input_file" => {
                            what_to_parse = WhatToParse::Name;
                            Some(ArgType::InputFile)
                        }
                        "output_file" => {
                            what_to_parse = WhatToParse::Name;
                            Some(ArgType::OutputFile)
                        }
                        "glyphs" => {
                            what_to_parse = WhatToParse::Name;
                            Some(ArgType::Glyphs)
                        }
                        &_ => {
                            eprintln!("\x1b[97;41;5mERR\x1b[m Unexpected argument name: {}", arg);
                            exit(1);
                        }
                    };
                } else {
                    match &name {
                        &Some(ArgType::Glyphs) => {
                            arguments.glyphs = Some(arg.trim().to_owned());
                        }
                        &Some(ArgType::ConfigFile) => {
                            arguments.config_file = Some(arg.trim().to_owned());
                        }
                        &Some(ArgType::InputFile) => {
                            arguments.input_file = Some(arg.trim().to_owned());
                        }
                        &Some(ArgType::OutputFile) => {
                            arguments.output_file = Some(arg.trim().to_owned());
                        }
                        _ => {
                            eprintln!(
                        "\x1b[97;41;5mERR\x1b[m Expected argument name (e.g. --cols), got: {}",
                        arg
                    );
                            exit(1);
                        }
                    }
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
                            Some(ArgType::Glyphs) => {
                                eprintln!(
                                "\x1b[97;41;5mERR\x1b[m Bug in parsing code, should read text now");
                                exit(2);
                            }
                            _ => {}
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
            WhatToParse::NumberPair => {
                let trimmed = arg.trim();
                let little_x = 'x';
                let capital_x = 'X';
                let mut split_by = little_x;
                if !trimmed.contains(little_x) {
                    if trimmed.contains(capital_x) {
                        split_by = capital_x;
                    } else {
                        eprintln!(
                            "\x1b[97;41;5mERR\x1b[m Expected integer pair split by x or X (e.g. 12x34 or 56X78), got: {}",
                            arg
                        );
                        exit(1);
                    }
                }
                let splited: Vec<&str> = trimmed.split(split_by).collect();
                if splited.len() != 2 {
                    eprintln!(
                        "\x1b[97;41;5mERR\x1b[m Expected integer pair value (e.g. 12x34), got: {}",
                        arg
                    );
                    exit(1);
                } else {
                    let cols = isize::from_str_radix(splited[0], 10);
                    let rows = isize::from_str_radix(splited[1], 10);
                    //let mut result = None;
                    if cols.is_ok() && rows.is_ok() {
                        let result = Some((cols.clone().unwrap(), rows.clone().unwrap()));
                        match &name {
                            Some(ArgType::ColorsOffset) => arguments.colors_offset = result,
                            Some(ArgType::BackgroundsOffset) => {
                                arguments.backgrounds_offset = result
                            }
                            Some(ArgType::StylesOffset) => arguments.styles_offset = result,
                            Some(ArgType::GlyphsOffset) => arguments.glyphs_offset = result,
                            Some(ArgType::WorkspaceOffset) => arguments.workspace_offset = result,
                            Some(ArgType::WorkspaceSize) => {
                                arguments.workspace_size =
                                    Some((cols.unwrap() as usize, rows.unwrap() as usize))
                            }
                            Some(ArgType::Rows) | Some(ArgType::Glyphs) => {
                                eprintln!(
                                "\x1b[97;41;5mERR\x1b[m Bug in parsing code, should read text now"
                            );
                                exit(2);
                            }
                            _ => {}
                        }
                    } else {
                        eprintln!(
                        "\x1b[97;41;5mERR\x1b[m Unable to parse integer pair value (e.g. 12x34), got: {}",
                        arg
                    );
                        exit(1);
                    }
                }
                what_to_parse = WhatToParse::Name;
            }
        }
    }
    arguments
}

pub fn read_config_file<P>(filename: &P) -> Arguments
where
    P: AsRef<Path> + AsRef<str> + Display,
{
    let mut args = Arguments::default();
    let hash = '#';
    if let Ok(file) = File::open(filename) {
        let mut read_string = String::with_capacity(1024);
        let mut br = io::BufReader::new(file);
        if br.read_to_string(&mut read_string).is_ok() {
            // let cs = 0;
            for line in read_string.lines() {
                if line.starts_with(hash) || line.is_empty() {
                    continue;
                } else {
                    parse_line(&mut args, line.trim());
                }
            }
        }
        args
    } else {
        eprintln!(
            "\x1b[97;41;5mERR\x1b[m Unable to open config file: {}",
            filename
        );
        exit(1)
    }
}

fn parse_line(args: &mut Arguments, line: &str) {
    let splited: Vec<&str> = line.split_ascii_whitespace().collect();
    args.config_file = None;
    match splited[0] {
        "rows" => {
            args.rows = usize::from_str_radix(splited[1], 10).ok();
        }
        "cols" => {
            args.cols = usize::from_str_radix(splited[1], 10).ok();
        }
        "colors_offset" => {
            args.colors_offset = offset_from_str(splited[1]);
        }
        "backgrounds_offset" => {
            args.backgrounds_offset = offset_from_str(splited[1]);
        }
        "styles_offset  " => {
            args.styles_offset = offset_from_str(splited[1]);
        }
        "glyphs_offset  " => {
            args.glyphs_offset = offset_from_str(splited[1]);
        }
        "workspace_offset" => {
            args.workspace_offset = offset_from_str(splited[1]);
        }
        "workspace_size" => {
            if let Some((c, r)) = offset_from_str(splited[1]) {
                args.workspace_size = Some((c as usize, r as usize));
            }
        }
        "input_file" => {
            args.input_file = Some(String::from(splited[1]));
        }
        "output_file" => {
            args.output_file = Some(String::from(splited[1]));
        }
        "glyphs" => {
            args.glyphs = Some(String::from(splited[1]));
        }
        "colors_left" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_left = keys;
            }
        }
        "colors_right" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_right = keys;
            }
        }
        "colors_far_right" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_far_right = keys;
            }
        }
        "colors_far_left" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_far_left = keys;
            }
        }
        "colors_top" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_top = keys;
            }
        }
        "colors_up" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_up = keys;
            }
        }
        "colors_down" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_down = keys;
            }
        }
        "colors_bottom" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_bottom = keys;
            }
        }
        "colors_invisible" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_invisible = keys;
            }
        }
        "colors_visible" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.colors_visible = keys;
            }
        }
        "backgrounds_left" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_left = keys;
            }
        }
        "backgrounds_right" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_right = keys;
            }
        }
        "backgrounds_far_right" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_far_right = keys;
            }
        }
        "backgrounds_far_left" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_far_left = keys;
            }
        }
        "backgrounds_top" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_top = keys;
            }
        }
        "backgrounds_up" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_up = keys;
            }
        }
        "backgrounds_down" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_down = keys;
            }
        }
        "backgrounds_bottom" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_bottom = keys;
            }
        }
        "backgrounds_invisible" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_invisible = keys;
            }
        }
        "backgrounds_visible" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.backgrounds_visible = keys;
            }
        }
        "glyphs_left" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_left = keys;
            }
        }
        "glyphs_right" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_right = keys;
            }
        }
        "glyphs_up" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_up = keys;
            }
        }
        "glyphs_down" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_down = keys;
            }
        }
        "glyphs_select" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_select = keys;
            }
        }
        "glyphs_prev" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_prev = keys;
            }
        }
        "glyphs_next" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_next = keys;
            }
        }
        "glyphs_home" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_home = keys;
            }
        }
        "glyphs_end" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.glyphs_end = keys;
            }
        }
        "workspace_left" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_left = keys;
            }
        }
        "workspace_right" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_right = keys;
            }
        }
        "workspace_up" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_up = keys;
            }
        }
        "workspace_down" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_down = keys;
            }
        }
        "workspace_line_start" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_line_start = keys;
            }
        }
        "workspace_line_end" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_line_end = keys;
            }
        }
        "workspace_select_color" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_select_color = keys;
            }
        }
        "workspace_select_background" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_select_background = keys;
            }
        }
        "workspace_select_glyph" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_select_glyph = keys;
            }
        }
        "workspace_erase" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.workspace_erase = keys;
            }
        }
        "style_up" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.style_up = keys;
            }
        }
        "style_down" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.style_down = keys;
            }
        }
        "style_enable" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.style_enable = keys;
            }
        }
        "style_disable" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.style_disable = keys;
            }
        }
        "print_graphic" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.print_graphic = keys;
            }
        }
        "print_screen" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.print_screen = keys;
            }
        }
        "exit" => {
            let mut keys = vec![];
            for s in splited.into_iter().skip(1) {
                if let Some(key) = str_to_key(s) {
                    keys.push(key);
                }
            }
            if keys.len() > 0 {
                args.bindings.exit = keys;
            }
        }

        _ => {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unrecognized config line: {}", line);
            exit(1)
        }
    }
    // println!("Got {}: {}", splited[0], splited[1]);
}

fn offset_from_str(text: &str) -> Option<(isize, isize)> {
    let trimmed = text.trim();
    let little_x = 'x';
    let capital_x = 'X';
    let mut split_by = little_x;
    if !trimmed.contains(little_x) {
        if trimmed.contains(capital_x) {
            split_by = capital_x;
        } else {
            eprintln!(
                            "\x1b[97;41;5mERR\x1b[m Expected integer pair split by x or X (e.g. 12x34 or 56X78), got: {}",
                            text
                        );
            exit(1);
        }
    }
    let splited: Vec<&str> = trimmed.split(split_by).collect();
    if splited.len() != 2 {
        eprintln!(
            "\x1b[97;41;5mERR\x1b[m Expected integer pair value (e.g. 12x34), got: {}",
            text
        );
        exit(1);
    } else {
        let cols = isize::from_str_radix(splited[0], 10);
        let rows = isize::from_str_radix(splited[1], 10);
        if let Ok(c) = cols {
            if let Ok(r) = rows {
                Some((c, r))
            } else {
                None
            }
        } else {
            None
        }
    }
}
pub fn verify_cols_and_rows(cols: Option<usize>, rows: Option<usize>) {
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
