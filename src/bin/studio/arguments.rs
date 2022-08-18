use std::env;
use std::path::Path;
use std::process::exit;

static ROWS_MIN: usize = 4;
static COLS_MIN: usize = 5;

pub struct Arguments {
    pub rows: Option<usize>,
    pub cols: Option<usize>,
    pub colors_offset: Option<(usize, usize)>,
    pub backgrounds_offset: Option<(usize, usize)>,
    pub styles_offset: Option<(usize, usize)>,
    pub glyphs_offset: Option<(usize, usize)>,
    pub workspace_offset: Option<(usize, usize)>,
    pub workspace_size: Option<(usize, usize)>,
    // pub input_file: Option<Path>    ,
    // pub output_file: Option<Path>    ,
    pub glyphs: Option<String>,
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
            glyphs: None,
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
            println!(
                " --glyphs <filename> - index file containing filenames with glyph definitions, each in separate line");
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
                    let cols = usize::from_str_radix(splited[0], 10);
                    let rows = usize::from_str_radix(splited[1], 10);
                    let mut result = None;
                    if cols.is_ok() && rows.is_ok() {
                        result = Some((cols.unwrap(), rows.unwrap()));
                        match &name {
                            Some(ArgType::ColorsOffset) => arguments.colors_offset = result,
                            Some(ArgType::BackgroundsOffset) => {
                                arguments.backgrounds_offset = result
                            }
                            Some(ArgType::StylesOffset) => arguments.styles_offset = result,
                            Some(ArgType::GlyphsOffset) => arguments.glyphs_offset = result,
                            Some(ArgType::WorkspaceOffset) => arguments.workspace_offset = result,
                            Some(ArgType::WorkspaceSize) => arguments.workspace_size = result,
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
