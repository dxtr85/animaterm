use super::key::Key;
use std::collections::HashMap;
use std::env;
use std::process::Command;

/// Ask OS how many rows and cols current terminal has.
pub fn ask_os_for_rows_and_cols() -> (usize, usize) {
    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|(k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
        .collect();
    let rows = match Command::new("tput")
        .arg("lines")
        .env_clear()
        .envs(&filtered_env)
        .output()
    {
        Ok(data) => {
            let output = String::from_utf8(data.stdout);
            eprintln!("OS tput lines result: {:?}", output);
            if output.is_ok() {
                let output = output.unwrap();
                let number = output.trim().parse::<usize>();
                match number {
                    Ok(a_number) => a_number,
                    Err(e) => {
                        eprintln!("\x1b[97;41;5mERR\x1b[m Unable to determine lines count from {}, using defaults\n{}", output, e);
                        35
                    }
                }
            } else {
                35
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
            let output = String::from_utf8(data.stdout);
            eprintln!("OS tput cols result: {:?}", output);
            if output.is_ok() {
                let output = output.unwrap();
                let number = output.trim().parse::<usize>();
                match number {
                    Ok(a_number) => a_number,
                    Err(e) => {
                        eprintln!("\x1b[97;41;5mERR\x1b[m Unable to determine cols count from {}, using defaults\n{}", output, e);
                        80
                    }
                }
            } else {
                80
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

/// Helper function used by read_char that converts non-printable
/// user input into Unicode Private Use Area range e000-e107
pub fn map_bytes_to_private_char(bytes: Vec<u8>) -> Option<char> {
    let how_many = bytes.len();
    match how_many {
        2 => {
            match bytes[1] {
                1 => Some('\u{e000}'),   //Key::AltCtrlA,
                2 => Some('\u{e001}'),   // Key::AltCtrlB,
                3 => Some('\u{e002}'),   // Key::AltCtrlC,
                4 => Some('\u{e003}'),   // Key::AltCtrlD,
                5 => Some('\u{e004}'),   // Key::AltCtrlE,
                6 => Some('\u{e005}'),   // Key::AltCtrlF,
                7 => Some('\u{e006}'),   // Key::AltCtrlG,
                8 => Some('\u{e007}'),   // Key::AltCtrlH,
                9 => Some('\u{e008}'),   // Key::AltTab,
                10 => Some('\u{e009}'),  // Key::AltEnter,
                11 => Some('\u{e00a}'),  // Key::AltCtrlK,
                12 => Some('\u{e00b}'),  // Key::AltCtrlL,
                13 => Some('\u{e00c}'),  // Key::AltCtrlM,
                14 => Some('\u{e00d}'),  // Key::AltCtrlN,
                15 => Some('\u{e00e}'),  // Key::AltCtrlO,
                16 => Some('\u{e00f}'),  // Key::AltCtrlP,
                17 => Some('\u{e010}'),  // Key::AltCtrlQ,
                18 => Some('\u{e011}'),  // Key::AltCtrlR,
                19 => Some('\u{e012}'),  // Key::AltCtrlS,
                20 => Some('\u{e013}'),  // Key::AltCtrlT,
                21 => Some('\u{e014}'),  // Key::AltCtrlU,
                22 => Some('\u{e015}'),  // Key::AltCtrlV,
                23 => Some('\u{e016}'),  // Key::AltCtrlW,
                24 => Some('\u{e017}'),  // Key::AltCtrlX,
                25 => Some('\u{e018}'),  // Key::AltCtrlY,
                26 => Some('\u{e019}'),  // Key::AltCtrlZ,
                27 => Some('\u{e01a}'),  // Key::AltEscape,
                28 => Some('\u{e01b}'),  // Key::AltFileSeparator,
                29 => Some('\u{e01c}'),  // Key::AltGroupSeparator,
                30 => Some('\u{e01d}'),  // Key::AltRecordSeparator,
                31 => Some('\u{e01e}'),  // Key::AltUnitSeparator,
                32 => Some('\u{e01f}'),  // Key::AltSpace,
                33 => Some('\u{e020}'),  // Key::AltExclamationMark,
                34 => Some('\u{e021}'),  // Key::AltQuote,
                35 => Some('\u{e022}'),  // Key::AltHash,
                36 => Some('\u{e023}'),  // Key::AltDollar,
                37 => Some('\u{e024}'),  // Key::AltPercent,
                38 => Some('\u{e025}'),  // Key::AltAmpersand,
                39 => Some('\u{e026}'),  // Key::AltApostrophe,
                40 => Some('\u{e027}'),  // Key::AltLeftParen,
                41 => Some('\u{e028}'),  // Key::AltRightParen,
                42 => Some('\u{e029}'),  // Key::AltStar,
                43 => Some('\u{e02a}'),  // Key::AltPlus,
                44 => Some('\u{e02b}'),  // Key::AltComma,
                45 => Some('\u{e02c}'),  // Key::AltDash,
                46 => Some('\u{e02d}'),  // Key::AltPeriod,
                47 => Some('\u{e02e}'),  // Key::AltSlash,
                48 => Some('\u{e02f}'),  // Key::AltZero,
                49 => Some('\u{e030}'),  // Key::AltOne,
                50 => Some('\u{e031}'),  // Key::AltTwo,
                51 => Some('\u{e032}'),  // Key::AltThree,
                52 => Some('\u{e033}'),  // Key::AltFour,
                53 => Some('\u{e034}'),  // Key::AltFive,
                54 => Some('\u{e035}'),  // Key::AltSix,
                55 => Some('\u{e036}'),  // Key::AltSeven,
                56 => Some('\u{e037}'),  // Key::AltEight,
                57 => Some('\u{e038}'),  // Key::AltNine,
                58 => Some('\u{e039}'),  // Key::AltColon,
                59 => Some('\u{e03a}'),  // Key::AltSemicolon,
                60 => Some('\u{e03b}'),  // Key::AltLessThan,
                61 => Some('\u{e03c}'),  // Key::AltEquals,
                62 => Some('\u{e03d}'),  // Key::AltGreaterThan,
                63 => Some('\u{e03e}'),  // Key::AltQuestionMark,
                64 => Some('\u{e03f}'),  // Key::AltAt,
                65 => Some('\u{e040}'),  // Key::AltShiftA,
                66 => Some('\u{e041}'),  // Key::AltShiftB,
                67 => Some('\u{e042}'),  // Key::AltShiftC,
                68 => Some('\u{e043}'),  // Key::AltShiftD,
                69 => Some('\u{e044}'),  // Key::AltShiftE,
                70 => Some('\u{e045}'),  // Key::AltShiftF,
                71 => Some('\u{e046}'),  // Key::AltShiftG,
                72 => Some('\u{e047}'),  // Key::AltShiftH,
                73 => Some('\u{e048}'),  // Key::AltShiftI,
                74 => Some('\u{e049}'),  // Key::AltShiftJ,
                75 => Some('\u{e04a}'),  // Key::AltShiftK,
                76 => Some('\u{e04b}'),  // Key::AltShiftL,
                77 => Some('\u{e04c}'),  // Key::AltShiftM,
                78 => Some('\u{e04d}'),  // Key::AltShiftN,
                79 => Some('\u{e04e}'),  // Key::AltShiftO,
                80 => Some('\u{e04f}'),  // Key::AltShiftP,
                81 => Some('\u{e050}'),  // Key::AltShiftQ,
                82 => Some('\u{e051}'),  // Key::AltShiftR,
                83 => Some('\u{e052}'),  // Key::AltShiftS,
                84 => Some('\u{e053}'),  // Key::AltShiftT,
                85 => Some('\u{e054}'),  // Key::AltShiftU,
                86 => Some('\u{e055}'),  // Key::AltShiftV,
                87 => Some('\u{e056}'),  // Key::AltShiftW,
                88 => Some('\u{e057}'),  // Key::AltShiftX,
                89 => Some('\u{e058}'),  // Key::AltShiftY,
                90 => Some('\u{e059}'),  // Key::AltShiftZ,
                91 => Some('\u{e05a}'),  // Key::AltLeftBracket,
                92 => Some('\u{e05b}'),  // Key::AltBackSlash,
                93 => Some('\u{e05c}'),  // Key::AltRightBracket,
                94 => Some('\u{e05d}'),  // Key::AltCaret,
                95 => Some('\u{e05e}'),  // Key::AltUnderscore,
                96 => Some('\u{e05f}'),  // Key::AltBackTick,
                97 => Some('\u{e060}'),  // Key::AltA,
                98 => Some('\u{e061}'),  // Key::AltB,
                99 => Some('\u{e062}'),  // Key::AltC,
                100 => Some('\u{e063}'), // Key::AltD,
                101 => Some('\u{e064}'), // Key::AltE,
                102 => Some('\u{e065}'), // Key::AltF,
                103 => Some('\u{e066}'), // Key::AltG,
                104 => Some('\u{e067}'), // Key::AltH,
                105 => Some('\u{e068}'), // Key::AltI,
                106 => Some('\u{e069}'), // Key::AltJ,
                107 => Some('\u{e06a}'), // Key::AltK,
                108 => Some('\u{e06b}'), // Key::AltL,
                109 => Some('\u{e06c}'), // Key::AltM,
                110 => Some('\u{e06d}'), // Key::AltN,
                111 => Some('\u{e06e}'), // Key::AltO,
                112 => Some('\u{e06f}'), // Key::AltP,
                113 => Some('\u{e070}'), // Key::AltQ,
                114 => Some('\u{e071}'), // Key::AltR,
                115 => Some('\u{e072}'), // Key::AltS,
                116 => Some('\u{e073}'), // Key::AltT,
                117 => Some('\u{e074}'), // Key::AltU,
                118 => Some('\u{e075}'), // Key::AltV,
                119 => Some('\u{e076}'), // Key::AltW,
                120 => Some('\u{e077}'), // Key::AltX,
                121 => Some('\u{e078}'), // Key::AltY,
                122 => Some('\u{e079}'), // Key::AltZ,
                123 => Some('\u{e07a}'), // Key::AltLeftBrace,
                124 => Some('\u{e07b}'), // Key::AltPipe,
                125 => Some('\u{e07c}'), // Key::AltRightBrace,
                126 => Some('\u{e07d}'), // Key::AltTilde,
                127 => Some('\u{e07e}'), // Key::AltDelete,
                _ => None,
            }
        }
        3 => {
            match bytes[1..3] {
                [79, 80] => Some('\u{e07f}'), // Key::F1,       // 27,79,80
                [79, 81] => Some('\u{e080}'), // Key::F2,       // 27,79,81
                [79, 82] => Some('\u{e081}'), // Key::F3,       // 27,79,82
                [79, 83] => Some('\u{e082}'), // Key::F4,       // 27,79,83
                [91, 65] => Some('\u{e083}'), // Key::Up,       // 27,91,65
                [91, 66] => Some('\u{e084}'), // Key::Down,     // 27,91,66
                [91, 68] => Some('\u{e085}'), // Key::Left,     // 27,91,68
                [91, 67] => Some('\u{e086}'), // Key::Right,    // 27,91,67
                [91, 72] => Some('\u{e087}'), // Key::Home,     // 27,91,72
                [91, 70] => Some('\u{e088}'), // Key::End,      // 27,91,70
                [91, 90] => Some('\u{e089}'), // Key::ShiftTab, // 27,91,90
                _ => None,
            }
        }
        4 => {
            match bytes[2] {
                49 => Some('\u{e087}'), // Key::Home,   // 27,91,49,126
                50 => Some('\u{e08b}'), // Key::Insert, // 27,91,50,126
                51 => Some('\u{e08a}'), // Key::Delete, // 27,91,51.126
                52 => Some('\u{e088}'), // Key::End,    // 27,91,52,126
                53 => Some('\u{e08c}'), // Key::PgUp,   // 27,91,53,126
                54 => Some('\u{e08d}'), // Key::PgDn,   // 27.91.54.126
                _ => None,
            }
        }

        5 => {
            match bytes[2..4] {
                [49, 53] => Some('\u{e08e}'), // Key::F5,   //      27,91,49,53,126
                [49, 55] => Some('\u{e08f}'), // Key::F6,   //      27,91,49,55,126
                [49, 56] => Some('\u{e090}'), // Key::F7,   //      27,91,49,56,126
                [49, 57] => Some('\u{e091}'), // Key::F8,   //      27,91,49,57,126
                [50, 48] => Some('\u{e092}'), // Key::F9,   //      27,91,50,48,126
                [50, 49] => Some('\u{e093}'), // Key::F10,  //      27,91,50,49,126
                [50, 51] => Some('\u{e094}'), // Key::F11,  //      27,91,50,51,126
                [50, 52] => Some('\u{e095}'), // Key::F12,  //      27,91,50,52,126
                [50, 57] => Some('\u{e096}'), // Key::Menu, //      27,91,50,57,126
                _ => None,
            }
        }

        6 => {
            match bytes[4..6] {
                [50, 80] => Some('\u{e097}'), // Key::ShiftF1,          //      27,91,49,59,50,80
                [51, 80] => Some('\u{e098}'), // Key::AltF1,            //      27,91,49,59,51,80
                [52, 80] => Some('\u{e099}'), // Key::AltShiftF1,       //      27,91,49,59,52,80
                [53, 80] => Some('\u{e09a}'), // Key::CtrlF1,           //      27,91,49,59,53,80
                [54, 80] => Some('\u{e09b}'), // Key::CtrlShiftF1,      //      27,91,49,59,54,80
                [55, 80] => Some('\u{e09c}'), // Key::AltCtrlF1,        //      27,91,49,59,55,80
                [56, 80] => Some('\u{e09d}'), // Key::AltCtrlShiftF1,   //      27,91,49,59,56,80
                [50, 81] => Some('\u{e09e}'), // Key::ShiftF2,          //      27,91,49,59,50,81
                [51, 81] => Some('\u{e09f}'), // Key::AltF2,            //      27,91,49,59,51,81
                [52, 81] => Some('\u{e0a0}'), // Key::AltShiftF2,       //      27,91,49,59,52,81
                [53, 81] => Some('\u{e0a1}'), // Key::CtrlF2,           //      27,91,49,59,53,81
                [54, 81] => Some('\u{e0a2}'), // Key::CtrlShiftF2,      //      27,91,49,59,54,81
                [55, 81] => Some('\u{e0a3}'), // Key::AltCtrlF2,        //      27,91,49,59,55,81
                [56, 81] => Some('\u{e0a4}'), // Key::AltCtrlShiftF2,   //      27,91,49,59,56,81
                [50, 82] => Some('\u{e0a5}'), // Key::ShiftF3,          //      27,91,49,59,50,82
                [51, 82] => Some('\u{e0a6}'), // Key::AltF3,            //      27,91,49,59,51,82
                [52, 82] => Some('\u{e0a7}'), // Key::AltShiftF3,       //      27,91,49,59,52,82
                [53, 82] => Some('\u{e0a8}'), // Key::CtrlF3,           //      27,91,49,59,53,82
                [54, 82] => Some('\u{e0a9}'), // Key::CtrlShiftF3,      //      27,91,49,59,54,82
                [55, 82] => Some('\u{e0aa}'), // Key::AltCtrlF3,        //      27,91,49,59,55,82
                [56, 82] => Some('\u{e0ab}'), // Key::AltCtrlShiftF3,   //      27,91,49,59,56,82
                [50, 83] => Some('\u{e0ac}'), // Key::ShiftF4,          //      27,91,49,59,50,83
                [51, 83] => Some('\u{e0ad}'), // Key::AltF4,            //      27,91,49,59,51,83
                [52, 83] => Some('\u{e0ae}'), // Key::AltShiftF4,       //      27,91,49,59,52,83
                [53, 83] => Some('\u{e0af}'), // Key::CtrlF4,           //      27,91,49,59,53,83
                [54, 83] => Some('\u{e0b0}'), // Key::CtrlShiftF4,      //      27,91,49,59,54,83
                [55, 83] => Some('\u{e0b1}'), // Key::AltCtrlF4,        //      27,91,49,59,55,83
                [56, 83] => Some('\u{e0b2}'), // Key::AltCtrlShiftF4,   //      27,91,49,59,56,83
                [50, 68] => Some('\u{e0b3}'), // Key::ShiftLeft,        //      27,91,49,59,50,68
                [51, 68] => Some('\u{e0b4}'), // Key::AltLeft,          //      27,91,49,59,51,68
                [52, 68] => Some('\u{e0b5}'), // Key::AltShiftLeft,     //      27,91,49,59,52,68
                [53, 68] => Some('\u{e0b6}'), // Key::CtrlLeft,         //      27,91,49,59,53,68
                [54, 68] => Some('\u{e0b7}'), // Key::CtrlShiftLeft,    //      27,91,49,59,54,68
                [55, 68] => Some('\u{e0b8}'), // Key::AltCtrlLeft,      //      27,91,49,59,55,68
                [56, 68] => Some('\u{e0b9}'), // Key::AltCtrlShiftLeft, //      27,91,49,59,56,68

                [50, 65] => Some('\u{e0ba}'), // Key::ShiftUp,        //      27,91,49,59,50,65
                [51, 65] => Some('\u{e0bb}'), // Key::AltUp,          //      27,91,49,59,51,65
                [52, 65] => Some('\u{e0bc}'), // Key::AltShiftUp,     //      27,91,49,59,52,65
                [53, 65] => Some('\u{e0bd}'), // Key::CtrlUp,         //      27,91,49,59,53,65
                [54, 65] => Some('\u{e0be}'), // Key::CtrlShiftUp,    //      27,91,49,59,54,65
                [55, 65] => Some('\u{e0bf}'), // Key::AltCtrlUp,      //      27,91,49,59,55,65
                [56, 65] => Some('\u{e0c0}'), // Key::AltCtrlShiftUp, //      27,91,49,59,56,65

                [50, 67] => Some('\u{e0c1}'), // Key::ShiftRight,    //      27,91,49,59,50,67
                [51, 67] => Some('\u{e0c2}'), // Key::AltRight,      //      27,91,49,59,51,67
                [52, 67] => Some('\u{e0c3}'), // Key::AltShiftRight, //      27,91,49,59,52,67
                [53, 67] => Some('\u{e0c4}'), // Key::CtrlRight,     //      27,91,49,59,53,67
                [54, 67] => Some('\u{e0c5}'), // Key::CtrlShiftRight, //      27,91,49,59,54,67
                [55, 67] => Some('\u{e0c6}'), // Key::AltCtrlRight,  //      27,91,49,59,55,67
                [56, 67] => Some('\u{e0c7}'), // Key::AltCtrlShiftRight, //      27,91,49,59,56,67

                [50, 66] => Some('\u{e0c8}'), // Key::ShiftDown,        // 27,91,49,59,50,66
                [51, 66] => Some('\u{e0c9}'), // Key::AltDown,          // 27,91,49,59,51,66
                [52, 66] => Some('\u{e0ca}'), // Key::AltShiftDown,     // 27,91,49,59,52,66
                [53, 66] => Some('\u{e0cb}'), // Key::CtrlDown,         // 27,91,49,59,53,66
                [54, 66] => Some('\u{e0cc}'), // Key::CtrlShiftDown,    // 27,91,49,59,54,66
                [55, 66] => Some('\u{e0cd}'), // Key::AltCtrlDown,      // 27,91,49,59,55,66
                [56, 66] => Some('\u{e0ce}'), // Key::AltCtrlShiftDown, // 27,91,49,59,56,66
                _ => None,
            }
        }

        7 => {
            match bytes[3..6] {
                [49, 53] => Some('\u{e0cf}'), // Key::F5,                  //      27,91,49,53,126
                [53, 59, 50] => Some('\u{e0d0}'), // Key::ShiftF5,         //      27,91,49,53,59,50,126
                [53, 59, 51] => Some('\u{e0d1}'), // Key::AltF5,           //      27,91,49,53,59,51,126
                [53, 59, 52] => Some('\u{e0d2}'), // Key::AltShiftF5,      //      27,91,49,53,59,52,126
                [53, 59, 53] => Some('\u{e0d3}'), // Key::CtrlF5,          //      27,91,49,53,59,53,126
                [53, 59, 54] => Some('\u{e0d4}'), // Key::CtrlShiftF5,     //      27,91,49,53,59,54,126
                [53, 59, 55] => Some('\u{e0d5}'), // Key::AltCtrlF5,       //      27,91,49,53,59,55,126
                [53, 59, 56] => Some('\u{e0d6}'), // Key::AltCtrlShiftF5,  //      27,91,49,53,59,56,126
                [55, 59, 50] => Some('\u{e0d7}'), // Key::ShiftF6,         //      27,91,49,55,59,50,126
                [55, 59, 51] => Some('\u{e0d8}'), // Key::AltF6,           //      27,91,49,55,59,51,126
                [55, 59, 52] => Some('\u{e0d9}'), // Key::AltShiftF6,      //      27,91,49,55,59,52,126
                [55, 59, 53] => Some('\u{e0da}'), // Key::CtrlF6,          //      27,91,49,55,59,53,126
                [55, 59, 54] => Some('\u{e0db}'), // Key::CtrlShiftF6,     //      27,91,49,55,59,54,126
                [55, 59, 55] => Some('\u{e0dc}'), // Key::AltCtrlF6,       //      27,91,49,55,59,55,126
                [55, 59, 56] => Some('\u{e0dd}'), // Key::AltCtrlShiftF6,  //      27,91,49,55,59,56,126
                [56, 59, 50] => Some('\u{e0de}'), // Key::ShiftF7,         //      27,91,49,56,59,50,126
                [56, 59, 51] => Some('\u{e0df}'), // Key::AltF7,           //      27,91,49,56,59,51,126
                [56, 59, 52] => Some('\u{e0e0}'), // Key::AltShiftF7,      //      27,91,49,56,59,52,126
                [56, 59, 53] => Some('\u{e0e1}'), // Key::CtrlF7,          //      27,91,49,56,59,53,126
                [56, 59, 54] => Some('\u{e0e2}'), // Key::CtrlShiftF7,     //      27,91,49,56,59,54,126
                [56, 59, 55] => Some('\u{e0e3}'), // Key::AltCtrlF7,       //      27,91,49,56,59,55,126
                [56, 59, 56] => Some('\u{e0e4}'), // Key::AltCtrlShiftF7,  //      27,91,49,56,59,56,126
                [57, 59, 50] => Some('\u{e0e5}'), // Key::ShiftF8,         //      27,91,49,57,59,50,126
                [57, 59, 51] => Some('\u{e0e6}'), // Key::AltF8,           //      27,91,49,57,59,51,126
                [57, 59, 52] => Some('\u{e0e7}'), // Key::AltShiftF8,      //      27,91,49,57,59,52,126
                [57, 59, 53] => Some('\u{e0e8}'), // Key::CtrlF8,          //      27,91,49,57,59,53,126
                [57, 59, 54] => Some('\u{e0e9}'), // Key::CtrlShiftF8,     //      27,91,49,57,59,54,126
                [57, 59, 55] => Some('\u{e0ea}'), // Key::AltCtrlF8,       //      27,91,49,57,59,55,126
                [57, 59, 56] => Some('\u{e0eb}'), // Key::AltCtrlShiftF8,  //      27,91,49,57,59,56,126
                [48, 59, 50] => Some('\u{e0ec}'), // Key::ShiftF9,         //      27,91,49,48,59,50,126
                [48, 59, 51] => Some('\u{e0ed}'), // Key::AltF9,           //      27,91,49,48,59,51,126
                [48, 59, 52] => Some('\u{e0ee}'), // Key::AltShiftF9,      //      27,91,49,48,59,52,126
                [48, 59, 53] => Some('\u{e0ef}'), // Key::CtrlF9,          //      27,91,49,48,59,53,126
                [48, 59, 54] => Some('\u{e0f0}'), // Key::CtrlShiftF9,     //      27,91,49,48,59,54,126
                [48, 59, 55] => Some('\u{e0f1}'), // Key::AltCtrlF9,       //      27,91,49,48,59,55,126
                [48, 59, 56] => Some('\u{e0f2}'), // Key::AltCtrlShiftF9,  //      27,91,49,48,59,56,126
                [49, 59, 50] => Some('\u{e0f3}'), // Key::ShiftF10,        //      27,91,49,49,59,50,126
                [49, 59, 51] => Some('\u{e0f4}'), // Key::AltF10,          //      27,91,49,49,59,51,126
                [49, 59, 52] => Some('\u{e0f5}'), // Key::AltShiftF10,     //      27,91,49,49,59,52,126
                [49, 59, 53] => Some('\u{e0f6}'), // Key::CtrlF10,         //      27,91,49,49,59,53,126
                [49, 59, 54] => Some('\u{e0f7}'), // Key::CtrlShiftF10,    //      27,91,49,49,59,54,126
                [49, 59, 55] => Some('\u{e0f8}'), // Key::AltCtrlF10,      //      27,91,49,49,59,55,126
                [49, 59, 56] => Some('\u{e0f9}'), // Key::AltCtrlShiftF10, //      27,91,49,49,59,56,126
                [51, 59, 50] => Some('\u{e0fa}'), // Key::ShiftF11,        //      27,91,49,51,59,50,126
                [51, 59, 51] => Some('\u{e0fb}'), // Key::AltF11,          //      27,91,49,51,59,51,126
                [51, 59, 52] => Some('\u{e0fc}'), // Key::AltShiftF11,     //      27,91,49,51,59,52,126
                [51, 59, 53] => Some('\u{e0fd}'), // Key::CtrlF11,         //      27,91,49,51,59,53,126
                [51, 59, 54] => Some('\u{e0fe}'), // Key::CtrlShiftF11,    //      27,91,49,51,59,54,126
                [51, 59, 55] => Some('\u{e0ff}'), // Key::AltCtrlF11,      //      27,91,49,51,59,55,126
                [51, 59, 56] => Some('\u{e100}'), // Key::AltCtrlShiftF11, //      27,91,49,51,59,56,126
                [52, 59, 50] => Some('\u{e101}'), // Key::ShiftF12,        //      27,91,49,52,59,50,126
                [52, 59, 51] => Some('\u{e102}'), // Key::AltF12,          //      27,91,49,52,59,51,126
                [52, 59, 52] => Some('\u{e103}'), // Key::AltShiftF12,     //      27,91,49,52,59,52,126
                [52, 59, 53] => Some('\u{e104}'), // Key::CtrlF12,         //      27,91,49,52,59,53,126
                [52, 59, 54] => Some('\u{e105}'), // Key::CtrlShiftF12,    //      27,91,49,52,59,54,126
                [52, 59, 55] => Some('\u{e106}'), // Key::AltCtrlF12,      //      27,91,49,52,59,55,126
                [52, 59, 56] => Some('\u{e107}'), // Key::AltCtrlShiftF12, //      27,91,49,52,59,56,126
                _ => None,
            }
        }
        _ => None,
    }
}

/// Helper function when user is typing in characters
/// and during that time we want to also read a Key or combination of Keys
pub fn map_private_char_to_key(char: char) -> Option<Key> {
    match char {
        '\u{e000}' => Some(Key::AltCtrlA),
        '\u{e001}' => Some(Key::AltCtrlB),
        '\u{e002}' => Some(Key::AltCtrlC),
        '\u{e003}' => Some(Key::AltCtrlD),
        '\u{e004}' => Some(Key::AltCtrlE),
        '\u{e005}' => Some(Key::AltCtrlF),
        '\u{e006}' => Some(Key::AltCtrlG),
        '\u{e007}' => Some(Key::AltCtrlH),
        '\u{e008}' => Some(Key::AltTab),
        '\u{e009}' => Some(Key::AltEnter),           //
        '\u{e00a}' => Some(Key::AltCtrlK),           //
        '\u{e00b}' => Some(Key::AltCtrlL),           //
        '\u{e00c}' => Some(Key::AltCtrlM),           //
        '\u{e00d}' => Some(Key::AltCtrlN),           //
        '\u{e00e}' => Some(Key::AltCtrlO),           //
        '\u{e00f}' => Some(Key::AltCtrlP),           //
        '\u{e010}' => Some(Key::AltCtrlQ),           //
        '\u{e011}' => Some(Key::AltCtrlR),           //
        '\u{e012}' => Some(Key::AltCtrlS),           //
        '\u{e013}' => Some(Key::AltCtrlT),           //
        '\u{e014}' => Some(Key::AltCtrlU),           //
        '\u{e015}' => Some(Key::AltCtrlV),           //
        '\u{e016}' => Some(Key::AltCtrlW),           //
        '\u{e017}' => Some(Key::AltCtrlX),           //
        '\u{e018}' => Some(Key::AltCtrlY),           //
        '\u{e019}' => Some(Key::AltCtrlZ),           //
        '\u{e01a}' => Some(Key::AltEscape),          //
        '\u{e01b}' => Some(Key::AltFileSeparator),   //
        '\u{e01c}' => Some(Key::AltGroupSeparator),  //
        '\u{e01d}' => Some(Key::AltRecordSeparator), //
        '\u{e01e}' => Some(Key::AltUnitSeparator),   //
        '\u{e01f}' => Some(Key::AltSpace),           //
        '\u{e020}' => Some(Key::AltExclamationMark), // ,
        '\u{e021}' => Some(Key::AltQuote),           // ,
        '\u{e022}' => Some(Key::AltHash),            // ,
        '\u{e023}' => Some(Key::AltDollar),          // ,
        '\u{e024}' => Some(Key::AltPercent),         // ,
        '\u{e025}' => Some(Key::AltAmpersand),       // ,
        '\u{e026}' => Some(Key::AltApostrophe),      // ,
        '\u{e027}' => Some(Key::AltLeftParen),       // ,
        '\u{e028}' => Some(Key::AltRightParen),      // ,
        '\u{e029}' => Some(Key::AltStar),            // ,
        '\u{e02a}' => Some(Key::AltPlus),            // ,
        '\u{e02b}' => Some(Key::AltComma),           // ,
        '\u{e02c}' => Some(Key::AltDash),            // ,
        '\u{e02d}' => Some(Key::AltPeriod),          // ,
        '\u{e02e}' => Some(Key::AltSlash),           // ,
        '\u{e02f}' => Some(Key::AltZero),            // ,
        '\u{e030}' => Some(Key::AltOne),             // ,
        '\u{e031}' => Some(Key::AltTwo),             // ,
        '\u{e032}' => Some(Key::AltThree),           // ,
        '\u{e033}' => Some(Key::AltFour),            // ,
        '\u{e034}' => Some(Key::AltFive),            // ,
        '\u{e035}' => Some(Key::AltSix),             // ,
        '\u{e036}' => Some(Key::AltSeven),           // ,
        '\u{e037}' => Some(Key::AltEight),           // ,
        '\u{e038}' => Some(Key::AltNine),            // ,
        '\u{e039}' => Some(Key::AltColon),           // ,
        '\u{e03a}' => Some(Key::AltSemicolon),       // ,
        '\u{e03b}' => Some(Key::AltLessThan),        // ,
        '\u{e03c}' => Some(Key::AltEquals),          // ,
        '\u{e03d}' => Some(Key::AltGreaterThan),     // ,
        '\u{e03e}' => Some(Key::AltQuestionMark),    // ,
        '\u{e03f}' => Some(Key::AltAt),              // ,
        '\u{e040}' => Some(Key::AltShiftA),          // ,
        '\u{e041}' => Some(Key::AltShiftB),          // ,
        '\u{e042}' => Some(Key::AltShiftC),          // ,
        '\u{e043}' => Some(Key::AltShiftD),          // ,
        '\u{e044}' => Some(Key::AltShiftE),          // ,
        '\u{e045}' => Some(Key::AltShiftF),          // ,
        '\u{e046}' => Some(Key::AltShiftG),          // ,
        '\u{e047}' => Some(Key::AltShiftH),          // ,
        '\u{e048}' => Some(Key::AltShiftI),          // ,
        '\u{e049}' => Some(Key::AltShiftJ),          // ,
        '\u{e04a}' => Some(Key::AltShiftK),          // ,
        '\u{e04b}' => Some(Key::AltShiftL),          // ,
        '\u{e04c}' => Some(Key::AltShiftM),          // ,
        '\u{e04d}' => Some(Key::AltShiftN),          // ,
        '\u{e04e}' => Some(Key::AltShiftO),          // ,
        '\u{e04f}' => Some(Key::AltShiftP),          // ,
        '\u{e050}' => Some(Key::AltShiftQ),          // ,
        '\u{e051}' => Some(Key::AltShiftR),          // ,
        '\u{e052}' => Some(Key::AltShiftS),          // ,
        '\u{e053}' => Some(Key::AltShiftT),          // ,
        '\u{e054}' => Some(Key::AltShiftU),          // ,
        '\u{e055}' => Some(Key::AltShiftV),          // ,
        '\u{e056}' => Some(Key::AltShiftW),          // ,
        '\u{e057}' => Some(Key::AltShiftX),          // ,
        '\u{e058}' => Some(Key::AltShiftY),          // ,
        '\u{e059}' => Some(Key::AltShiftZ),          // ,
        '\u{e05a}' => Some(Key::AltLeftBracket),     // ,
        '\u{e05b}' => Some(Key::AltBackSlash),       // ,
        '\u{e05c}' => Some(Key::AltRightBracket),    // ,
        '\u{e05d}' => Some(Key::AltCaret),           // ,
        '\u{e05e}' => Some(Key::AltUnderscore),      // ,
        '\u{e05f}' => Some(Key::AltBackTick),        // ,
        '\u{e060}' => Some(Key::AltA),               // ,
        '\u{e061}' => Some(Key::AltB),               // ,
        '\u{e062}' => Some(Key::AltC),               // ,
        '\u{e063}' => Some(Key::AltD),               // ,
        '\u{e064}' => Some(Key::AltE),               // ,
        '\u{e065}' => Some(Key::AltF),               // ,
        '\u{e066}' => Some(Key::AltG),               // ,
        '\u{e067}' => Some(Key::AltH),               // ,
        '\u{e068}' => Some(Key::AltI),               // ,
        '\u{e069}' => Some(Key::AltJ),               // ,
        '\u{e06a}' => Some(Key::AltK),               // ,
        '\u{e06b}' => Some(Key::AltL),               // ,
        '\u{e06c}' => Some(Key::AltM),               // ,
        '\u{e06d}' => Some(Key::AltN),               // ,
        '\u{e06e}' => Some(Key::AltO),               // ,
        '\u{e06f}' => Some(Key::AltP),               // ,
        '\u{e070}' => Some(Key::AltQ),               // ,
        '\u{e071}' => Some(Key::AltR),               // ,
        '\u{e072}' => Some(Key::AltS),               // ,
        '\u{e073}' => Some(Key::AltT),               // ,
        '\u{e074}' => Some(Key::AltU),               // ,
        '\u{e075}' => Some(Key::AltV),               // ,
        '\u{e076}' => Some(Key::AltW),               // ,
        '\u{e077}' => Some(Key::AltX),               // ,
        '\u{e078}' => Some(Key::AltY),               // ,
        '\u{e079}' => Some(Key::AltZ),               // ,
        '\u{e07a}' => Some(Key::AltLeftBrace),       // ,
        '\u{e07b}' => Some(Key::AltPipe),            // ,
        '\u{e07c}' => Some(Key::AltRightBrace),      // ,
        '\u{e07d}' => Some(Key::AltTilde),           // ,
        '\u{e07e}' => Some(Key::AltDelete),          // ,
        '\u{e07f}' => Some(Key::F1),                 // ,       // 27,79,80
        '\u{e080}' => Some(Key::F2),                 // ,       // 27,79,81
        '\u{e081}' => Some(Key::F3),                 // ,       // 27,79,82
        '\u{e082}' => Some(Key::F4),                 // ,       // 27,79,83
        '\u{e083}' => Some(Key::Up),                 // ,       // 27,91,65
        '\u{e084}' => Some(Key::Down),               // ,     // 27,91,66
        '\u{e085}' => Some(Key::Left),               // ,     // 27,91,68
        '\u{e086}' => Some(Key::Right),              // ,    // 27,91,67
        '\u{e087}' => Some(Key::Home),               // ,     // 27,91,72
        '\u{e088}' => Some(Key::End),                // ,      // 27,91,70
        '\u{e089}' => Some(Key::ShiftTab),           // , // 27,91,90
        '\u{e08a}' => Some(Key::Delete),             // , // 27,91,51.126
        '\u{e08b}' => Some(Key::Insert),             // , // 27,91,50,126
        '\u{e08c}' => Some(Key::PgUp),               // ,   // 27,91,53,126
        '\u{e08d}' => Some(Key::PgDn),               // ,   // 27.91.54.126
        '\u{e08e}' => Some(Key::F5),                 // ,   //      27,91,49,53,126
        '\u{e08f}' => Some(Key::F6),                 // ,   //      27,91,49,55,126
        '\u{e090}' => Some(Key::F7),                 // ,   //      27,91,49,56,126
        '\u{e091}' => Some(Key::F8),                 // ,   //      27,91,49,57,126
        '\u{e092}' => Some(Key::F9),                 // ,   //      27,91,50,48,126
        '\u{e093}' => Some(Key::F10),                // ,  //      27,91,50,49,126
        '\u{e094}' => Some(Key::F11),                // ,  //      27,91,50,51,126
        '\u{e095}' => Some(Key::F12),                // ,  //      27,91,50,52,126
        '\u{e096}' => Some(Key::Menu),               // , //      27,91,50,57,126
        '\u{e097}' => Some(Key::ShiftF1),            // ,          //      27,91,49,59,50,80
        '\u{e098}' => Some(Key::AltF1),              // ,            //      27,91,49,59,51,80
        '\u{e099}' => Some(Key::AltShiftF1),         // ,       //      27,91,49,59,52,80
        '\u{e09a}' => Some(Key::CtrlF1),             // ,           //      27,91,49,59,53,80
        '\u{e09b}' => Some(Key::CtrlShiftF1),        // ,      //      27,91,49,59,54,80
        '\u{e09c}' => Some(Key::AltCtrlF1),          // ,        //      27,91,49,59,55,80
        '\u{e09d}' => Some(Key::AltCtrlShiftF1),     // ,   //      27,91,49,59,56,80
        '\u{e09e}' => Some(Key::ShiftF2),            // ,          //      27,91,49,59,50,81
        '\u{e09f}' => Some(Key::AltF2),              // ,            //      27,91,49,59,51,81
        '\u{e0a0}' => Some(Key::AltShiftF2),         // ,       //      27,91,49,59,52,81
        '\u{e0a1}' => Some(Key::CtrlF2),             // ,           //      27,91,49,59,53,81
        '\u{e0a2}' => Some(Key::CtrlShiftF2),        // ,      //      27,91,49,59,54,81
        '\u{e0a3}' => Some(Key::AltCtrlF2),          // ,        //      27,91,49,59,55,81
        '\u{e0a4}' => Some(Key::AltCtrlShiftF2),     // ,   //      27,91,49,59,56,81
        '\u{e0a5}' => Some(Key::ShiftF3),            // ,          //      27,91,49,59,50,82
        '\u{e0a6}' => Some(Key::AltF3),              // ,            //      27,91,49,59,51,82
        '\u{e0a7}' => Some(Key::AltShiftF3),         // ,       //      27,91,49,59,52,82
        '\u{e0a8}' => Some(Key::CtrlF3),             // ,           //      27,91,49,59,53,82
        '\u{e0a9}' => Some(Key::CtrlShiftF3),        // ,      //      27,91,49,59,54,82
        '\u{e0aa}' => Some(Key::AltCtrlF3),          // ,        //      27,91,49,59,55,82
        '\u{e0ab}' => Some(Key::AltCtrlShiftF3),     // ,   //      27,91,49,59,56,82
        '\u{e0ac}' => Some(Key::ShiftF4),            // ,          //      27,91,49,59,50,83
        '\u{e0ad}' => Some(Key::AltF4),              // ,            //      27,91,49,59,51,83
        '\u{e0ae}' => Some(Key::AltShiftF4),         // ,       //      27,91,49,59,52,83
        '\u{e0af}' => Some(Key::CtrlF4),             // ,           //      27,91,49,59,53,83
        '\u{e0b0}' => Some(Key::CtrlShiftF4),        // ,      //      27,91,49,59,54,83
        '\u{e0b1}' => Some(Key::AltCtrlF4),          // ,        //      27,91,49,59,55,83
        '\u{e0b2}' => Some(Key::AltCtrlShiftF4),     // ,   //      27,91,49,59,56,83
        '\u{e0b3}' => Some(Key::ShiftLeft),          // ,        //      27,91,49,59,50,68
        '\u{e0b4}' => Some(Key::AltLeft),            // ,          //      27,91,49,59,51,68
        '\u{e0b5}' => Some(Key::AltShiftLeft),       // ,     //      27,91,49,59,52,68
        '\u{e0b6}' => Some(Key::CtrlLeft),           // ,         //      27,91,49,59,53,68
        '\u{e0b7}' => Some(Key::CtrlShiftLeft),      // ,    //      27,91,49,59,54,68
        '\u{e0b8}' => Some(Key::AltCtrlLeft),        // ,      //      27,91,49,59,55,68
        '\u{e0b9}' => Some(Key::AltCtrlShiftLeft),   // , //      27,91,49,59,56,68

        '\u{e0ba}' => Some(Key::ShiftUp), // ,        //      27,91,49,59,50,65
        '\u{e0bb}' => Some(Key::AltUp),   // ,          //      27,91,49,59,51,65
        '\u{e0bc}' => Some(Key::AltShiftUp), // ,     //      27,91,49,59,52,65
        '\u{e0bd}' => Some(Key::CtrlUp),  // ,         //      27,91,49,59,53,65
        '\u{e0be}' => Some(Key::CtrlShiftUp), // ,    //      27,91,49,59,54,65
        '\u{e0bf}' => Some(Key::AltCtrlUp), // ,      //      27,91,49,59,55,65
        '\u{e0c0}' => Some(Key::AltCtrlShiftUp), // , //      27,91,49,59,56,65

        '\u{e0c1}' => Some(Key::ShiftRight), // ,    //      27,91,49,59,50,67
        '\u{e0c2}' => Some(Key::AltRight),   // ,      //      27,91,49,59,51,67
        '\u{e0c3}' => Some(Key::AltShiftRight), // , //      27,91,49,59,52,67
        '\u{e0c4}' => Some(Key::CtrlRight),  // ,     //      27,91,49,59,53,67
        '\u{e0c5}' => Some(Key::CtrlShiftRight), // , //      27,91,49,59,54,67
        '\u{e0c6}' => Some(Key::AltCtrlRight), // ,  //      27,91,49,59,55,67
        '\u{e0c7}' => Some(Key::AltCtrlShiftRight), // , //      27,91,49,59,56,67

        '\u{e0c8}' => Some(Key::ShiftDown), // ,        // 27,91,49,59,50,66
        '\u{e0c9}' => Some(Key::AltDown),   // ,          // 27,91,49,59,51,66
        '\u{e0ca}' => Some(Key::AltShiftDown), // ,     // 27,91,49,59,52,66
        '\u{e0cb}' => Some(Key::CtrlDown),  // ,         // 27,91,49,59,53,66
        '\u{e0cc}' => Some(Key::CtrlShiftDown), // ,    // 27,91,49,59,54,66
        '\u{e0cd}' => Some(Key::AltCtrlDown), // ,      // 27,91,49,59,55,66
        '\u{e0ce}' => Some(Key::AltCtrlShiftDown), // , // 27,91,49,59,56,66
        '\u{e0cf}' => Some(Key::F5),        // ,                  //      27,91,49,53,126
        '\u{e0d0}' => Some(Key::ShiftF5),   // ,         //      27,91,49,53,59,50,126
        '\u{e0d1}' => Some(Key::AltF5),     // ,           //      27,91,49,53,59,51,126
        '\u{e0d2}' => Some(Key::AltShiftF5), // ,      //      27,91,49,53,59,52,126
        '\u{e0d3}' => Some(Key::CtrlF5),    // ,          //      27,91,49,53,59,53,126
        '\u{e0d4}' => Some(Key::CtrlShiftF5), // ,     //      27,91,49,53,59,54,126
        '\u{e0d5}' => Some(Key::AltCtrlF5), // ,       //      27,91,49,53,59,55,126
        '\u{e0d6}' => Some(Key::AltCtrlShiftF5), // ,  //      27,91,49,53,59,56,126
        '\u{e0d7}' => Some(Key::ShiftF6),   // ,         //      27,91,49,55,59,50,126
        '\u{e0d8}' => Some(Key::AltF6),     // ,           //      27,91,49,55,59,51,126
        '\u{e0d9}' => Some(Key::AltShiftF6), // ,      //      27,91,49,55,59,52,126
        '\u{e0da}' => Some(Key::CtrlF6),    // ,          //      27,91,49,55,59,53,126
        '\u{e0db}' => Some(Key::CtrlShiftF6), // ,     //      27,91,49,55,59,54,126
        '\u{e0dc}' => Some(Key::AltCtrlF6), // ,       //      27,91,49,55,59,55,126
        '\u{e0dd}' => Some(Key::AltCtrlShiftF6), // ,  //      27,91,49,55,59,56,126
        '\u{e0de}' => Some(Key::ShiftF7),   // ,         //      27,91,49,56,59,50,126
        '\u{e0df}' => Some(Key::AltF7),     // ,           //      27,91,49,56,59,51,126
        '\u{e0e0}' => Some(Key::AltShiftF7), // ,      //      27,91,49,56,59,52,126
        '\u{e0e1}' => Some(Key::CtrlF7),    // ,          //      27,91,49,56,59,53,126
        '\u{e0e2}' => Some(Key::CtrlShiftF7), // ,     //      27,91,49,56,59,54,126
        '\u{e0e3}' => Some(Key::AltCtrlF7), // ,       //      27,91,49,56,59,55,126
        '\u{e0e4}' => Some(Key::AltCtrlShiftF7), // ,  //      27,91,49,56,59,56,126
        '\u{e0e5}' => Some(Key::ShiftF8),   // ,         //      27,91,49,57,59,50,126
        '\u{e0e6}' => Some(Key::AltF8),     // ,           //      27,91,49,57,59,51,126
        '\u{e0e7}' => Some(Key::AltShiftF8), // ,      //      27,91,49,57,59,52,126
        '\u{e0e8}' => Some(Key::CtrlF8),    // ,          //      27,91,49,57,59,53,126
        '\u{e0e9}' => Some(Key::CtrlShiftF8), // ,     //      27,91,49,57,59,54,126
        '\u{e0ea}' => Some(Key::AltCtrlF8), // ,       //      27,91,49,57,59,55,126
        '\u{e0eb}' => Some(Key::AltCtrlShiftF8), // ,  //      27,91,49,57,59,56,126
        '\u{e0ec}' => Some(Key::ShiftF9),   // ,         //      27,91,49,48,59,50,126
        '\u{e0ed}' => Some(Key::AltF9),     // ,           //      27,91,49,48,59,51,126
        '\u{e0ee}' => Some(Key::AltShiftF9), // ,      //      27,91,49,48,59,52,126
        '\u{e0ef}' => Some(Key::CtrlF9),    // ,          //      27,91,49,48,59,53,126
        '\u{e0f0}' => Some(Key::CtrlShiftF9), // ,     //      27,91,49,48,59,54,126
        '\u{e0f1}' => Some(Key::AltCtrlF9), // ,       //      27,91,49,48,59,55,126
        '\u{e0f2}' => Some(Key::AltCtrlShiftF9), // ,  //      27,91,49,48,59,56,126
        '\u{e0f3}' => Some(Key::ShiftF10),  // ,        //      27,91,49,49,59,50,126
        '\u{e0f4}' => Some(Key::AltF10),    // ,          //      27,91,49,49,59,51,126
        '\u{e0f5}' => Some(Key::AltShiftF10), // ,     //      27,91,49,49,59,52,126
        '\u{e0f6}' => Some(Key::CtrlF10),   // ,         //      27,91,49,49,59,53,126
        '\u{e0f7}' => Some(Key::CtrlShiftF10), // ,    //      27,91,49,49,59,54,126
        '\u{e0f8}' => Some(Key::AltCtrlF10), // ,      //      27,91,49,49,59,55,126
        '\u{e0f9}' => Some(Key::AltCtrlShiftF10), // , //      27,91,49,49,59,56,126
        '\u{e0fa}' => Some(Key::ShiftF11),  // ,        //      27,91,49,51,59,50,126
        '\u{e0fb}' => Some(Key::AltF11),    // ,          //      27,91,49,51,59,51,126
        '\u{e0fc}' => Some(Key::AltShiftF11), // ,     //      27,91,49,51,59,52,126
        '\u{e0fd}' => Some(Key::CtrlF11),   // ,         //      27,91,49,51,59,53,126
        '\u{e0fe}' => Some(Key::CtrlShiftF11), // ,    //      27,91,49,51,59,54,126
        '\u{e0ff}' => Some(Key::AltCtrlF11), // ,      //      27,91,49,51,59,55,126
        '\u{e100}' => Some(Key::AltCtrlShiftF11), // , //      27,91,49,51,59,56,126
        '\u{e101}' => Some(Key::ShiftF12),  // ,        //      27,91,49,52,59,50,126
        '\u{e102}' => Some(Key::AltF12),    // ,          //      27,91,49,52,59,51,126
        '\u{e103}' => Some(Key::AltShiftF12), // ,     //      27,91,49,52,59,52,126
        '\u{e104}' => Some(Key::CtrlF12),   // ,         //      27,91,49,52,59,53,126
        '\u{e105}' => Some(Key::CtrlShiftF12), // ,    //      27,91,49,52,59,54,126
        '\u{e106}' => Some(Key::AltCtrlF12), // ,      //      27,91,49,52,59,55,126
        '\u{e107}' => Some(Key::AltCtrlShiftF12), // , //      27,91,49,52,59,56,126
        _ => None,
    }
}

/// Convert numerical value received from keyboard into it's Key representation.
pub fn map_key_to_char(key: &Key) -> Option<char> {
    match *key {
        Key::Space => Some(' '),
        Key::ExclamationMark => Some('!'),
        Key::Quote => Some('"'),
        Key::Hash => Some('#'),
        Key::Dollar => Some('$'),
        Key::Percent => Some('%'),
        Key::Ampersand => Some('&'),
        Key::Apostrophe => Some('\''),
        Key::LeftParen => Some('('),
        Key::RightParen => Some(')'),
        Key::Star => Some('*'),
        Key::Plus => Some('+'),
        Key::Comma => Some(','),
        Key::Dash => Some('^'),
        Key::Period => Some('.'),
        Key::Slash => Some('/'),
        Key::Zero => Some('0'),
        Key::One => Some('1'),
        Key::Two => Some('2'),
        Key::Three => Some('3'),
        Key::Four => Some('4'),
        Key::Five => Some('5'),
        Key::Six => Some('6'),
        Key::Seven => Some('7'),
        Key::Eight => Some('8'),
        Key::Nine => Some('9'),
        Key::Colon => Some(':'),
        Key::Semicolon => Some(';'),
        Key::LessThan => Some('<'),
        Key::Equals => Some('='),
        Key::GreaterThan => Some('>'),
        Key::QuestionMark => Some('?'),
        Key::At => Some('@'),
        Key::ShiftA => Some('A'),
        Key::ShiftB => Some('B'),
        Key::ShiftC => Some('C'),
        Key::ShiftD => Some('D'),
        Key::ShiftE => Some('E'),
        Key::ShiftF => Some('F'),
        Key::ShiftG => Some('G'),
        Key::ShiftH => Some('H'),
        Key::ShiftI => Some('I'),
        Key::ShiftJ => Some('J'),
        Key::ShiftK => Some('K'),
        Key::ShiftL => Some('L'),
        Key::ShiftM => Some('M'),
        Key::ShiftN => Some('N'),
        Key::ShiftO => Some('O'),
        Key::ShiftP => Some('P'),
        Key::ShiftQ => Some('Q'),
        Key::ShiftR => Some('R'),
        Key::ShiftS => Some('S'),
        Key::ShiftT => Some('T'),
        Key::ShiftU => Some('U'),
        Key::ShiftV => Some('V'),
        Key::ShiftW => Some('W'),
        Key::ShiftX => Some('X'),
        Key::ShiftY => Some('Y'),
        Key::ShiftZ => Some('Z'),
        Key::LeftBracket => Some('['),
        Key::BackSlash => Some('\\'),
        Key::RightBracket => Some(']'),
        Key::Caret => Some('\r'),
        Key::Underscore => Some('_'),
        Key::BackTick => Some('`'),
        Key::A => Some('a'),
        Key::B => Some('b'),
        Key::C => Some('c'),
        Key::D => Some('d'),
        Key::E => Some('e'),
        Key::F => Some('f'),
        Key::G => Some('g'),
        Key::H => Some('h'),
        Key::I => Some('i'),
        Key::J => Some('j'),
        Key::K => Some('k'),
        Key::L => Some('l'),
        Key::M => Some('m'),
        Key::N => Some('n'),
        Key::O => Some('o'),
        Key::P => Some('p'),
        Key::Q => Some('q'),
        Key::R => Some('r'),
        Key::S => Some('s'),
        Key::T => Some('t'),
        Key::U => Some('u'),
        Key::V => Some('v'),
        Key::W => Some('w'),
        Key::X => Some('x'),
        Key::Y => Some('y'),
        Key::Z => Some('z'),
        Key::LeftBrace => Some('{'),
        Key::Pipe => Some('|'),
        Key::RightBrace => Some('}'),
        Key::Tilde => Some('~'),
        _ => None,
    }
}
/// Convert numerical value received from keyboard into it's Key representation.
pub fn map_bytes_to_key(bytes: Vec<u8>) -> Option<Key> {
    let how_many = bytes.len();
    match how_many {
        0 => None,
        1 => {
            let key = match bytes[0] {
                1 => Key::CtrlA,
                2 => Key::CtrlB,
                3 => Key::CtrlC,
                4 => Key::CtrlD,
                5 => Key::CtrlE,
                6 => Key::CtrlF,
                7 => Key::CtrlG,
                8 => Key::Backspace,
                9 => Key::Tab,
                10 => Key::Enter,
                11 => Key::CtrlK,
                12 => Key::CtrlL,
                13 => Key::CtrlM,
                14 => Key::CtrlN,
                15 => Key::CtrlO,
                16 => Key::CtrlP,
                17 => Key::CtrlQ,
                18 => Key::CtrlR,
                19 => Key::CtrlS,
                20 => Key::CtrlT,
                21 => Key::CtrlU,
                22 => Key::CtrlV,
                23 => Key::CtrlW,
                24 => Key::CtrlX,
                25 => Key::CtrlY,
                26 => Key::CtrlZ,
                27 => Key::Escape,
                28 => Key::FileSeparator,
                29 => Key::GroupSeparator,
                30 => Key::RecordSeparator,
                31 => Key::UnitSeparator,
                32 => Key::Space,
                33 => Key::ExclamationMark,
                34 => Key::Quote,
                35 => Key::Hash,
                36 => Key::Dollar,
                37 => Key::Percent,
                38 => Key::Ampersand,
                39 => Key::Apostrophe,
                40 => Key::LeftParen,
                41 => Key::RightParen,
                42 => Key::Star,
                43 => Key::Plus,
                44 => Key::Comma,
                45 => Key::Dash,
                46 => Key::Period,
                47 => Key::Slash,
                48 => Key::Zero,
                49 => Key::One,
                50 => Key::Two,
                51 => Key::Three,
                52 => Key::Four,
                53 => Key::Five,
                54 => Key::Six,
                55 => Key::Seven,
                56 => Key::Eight,
                57 => Key::Nine,
                58 => Key::Colon,
                59 => Key::Semicolon,
                60 => Key::LessThan,
                61 => Key::Equals,
                62 => Key::GreaterThan,
                63 => Key::QuestionMark,
                64 => Key::At,
                65 => Key::ShiftA,
                66 => Key::ShiftB,
                67 => Key::ShiftC,
                68 => Key::ShiftD,
                69 => Key::ShiftE,
                70 => Key::ShiftF,
                71 => Key::ShiftG,
                72 => Key::ShiftH,
                73 => Key::ShiftI,
                74 => Key::ShiftJ,
                75 => Key::ShiftK,
                76 => Key::ShiftL,
                77 => Key::ShiftM,
                78 => Key::ShiftN,
                79 => Key::ShiftO,
                80 => Key::ShiftP,
                81 => Key::ShiftQ,
                82 => Key::ShiftR,
                83 => Key::ShiftS,
                84 => Key::ShiftT,
                85 => Key::ShiftU,
                86 => Key::ShiftV,
                87 => Key::ShiftW,
                88 => Key::ShiftX,
                89 => Key::ShiftY,
                90 => Key::ShiftZ,
                91 => Key::LeftBracket,
                92 => Key::BackSlash,
                93 => Key::RightBracket,
                94 => Key::Caret,
                95 => Key::Underscore,
                96 => Key::BackTick,
                97 => Key::A,
                98 => Key::B,
                99 => Key::C,
                100 => Key::D,
                101 => Key::E,
                102 => Key::F,
                103 => Key::G,
                104 => Key::H,
                105 => Key::I,
                106 => Key::J,
                107 => Key::K,
                108 => Key::L,
                109 => Key::M,
                110 => Key::N,
                111 => Key::O,
                112 => Key::P,
                113 => Key::Q,
                114 => Key::R,
                115 => Key::S,
                116 => Key::T,
                117 => Key::U,
                118 => Key::V,
                119 => Key::W,
                120 => Key::X,
                121 => Key::Y,
                122 => Key::Z,
                123 => Key::LeftBrace,
                124 => Key::Pipe,
                125 => Key::RightBrace,
                126 => Key::Tilde,
                127 => Key::Delete,
                _ => Key::Unicode(bytes),
            };
            Some(key)
        }
        2 => {
            let key = match bytes[1] {
                1 => Key::AltCtrlA,
                2 => Key::AltCtrlB,
                3 => Key::AltCtrlC,
                4 => Key::AltCtrlD,
                5 => Key::AltCtrlE,
                6 => Key::AltCtrlF,
                7 => Key::AltCtrlG,
                8 => Key::AltCtrlH,
                9 => Key::AltTab,
                10 => Key::AltEnter,
                11 => Key::AltCtrlK,
                12 => Key::AltCtrlL,
                13 => Key::AltCtrlM,
                14 => Key::AltCtrlN,
                15 => Key::AltCtrlO,
                16 => Key::AltCtrlP,
                17 => Key::AltCtrlQ,
                18 => Key::AltCtrlR,
                19 => Key::AltCtrlS,
                20 => Key::AltCtrlT,
                21 => Key::AltCtrlU,
                22 => Key::AltCtrlV,
                23 => Key::AltCtrlW,
                24 => Key::AltCtrlX,
                25 => Key::AltCtrlY,
                26 => Key::AltCtrlZ,
                27 => Key::AltEscape,
                28 => Key::AltFileSeparator,
                29 => Key::AltGroupSeparator,
                30 => Key::AltRecordSeparator,
                31 => Key::AltUnitSeparator,
                32 => Key::AltSpace,
                33 => Key::AltExclamationMark,
                34 => Key::AltQuote,
                35 => Key::AltHash,
                36 => Key::AltDollar,
                37 => Key::AltPercent,
                38 => Key::AltAmpersand,
                39 => Key::AltApostrophe,
                40 => Key::AltLeftParen,
                41 => Key::AltRightParen,
                42 => Key::AltStar,
                43 => Key::AltPlus,
                44 => Key::AltComma,
                45 => Key::AltDash,
                46 => Key::AltPeriod,
                47 => Key::AltSlash,
                48 => Key::AltZero,
                49 => Key::AltOne,
                50 => Key::AltTwo,
                51 => Key::AltThree,
                52 => Key::AltFour,
                53 => Key::AltFive,
                54 => Key::AltSix,
                55 => Key::AltSeven,
                56 => Key::AltEight,
                57 => Key::AltNine,
                58 => Key::AltColon,
                59 => Key::AltSemicolon,
                60 => Key::AltLessThan,
                61 => Key::AltEquals,
                62 => Key::AltGreaterThan,
                63 => Key::AltQuestionMark,
                64 => Key::AltAt,
                65 => Key::AltShiftA,
                66 => Key::AltShiftB,
                67 => Key::AltShiftC,
                68 => Key::AltShiftD,
                69 => Key::AltShiftE,
                70 => Key::AltShiftF,
                71 => Key::AltShiftG,
                72 => Key::AltShiftH,
                73 => Key::AltShiftI,
                74 => Key::AltShiftJ,
                75 => Key::AltShiftK,
                76 => Key::AltShiftL,
                77 => Key::AltShiftM,
                78 => Key::AltShiftN,
                79 => Key::AltShiftO,
                80 => Key::AltShiftP,
                81 => Key::AltShiftQ,
                82 => Key::AltShiftR,
                83 => Key::AltShiftS,
                84 => Key::AltShiftT,
                85 => Key::AltShiftU,
                86 => Key::AltShiftV,
                87 => Key::AltShiftW,
                88 => Key::AltShiftX,
                89 => Key::AltShiftY,
                90 => Key::AltShiftZ,
                91 => Key::AltLeftBracket,
                92 => Key::AltBackSlash,
                93 => Key::AltRightBracket,
                94 => Key::AltCaret,
                95 => Key::AltUnderscore,
                96 => Key::AltBackTick,
                97 => Key::AltA,
                98 => Key::AltB,
                99 => Key::AltC,
                100 => Key::AltD,
                101 => Key::AltE,
                102 => Key::AltF,
                103 => Key::AltG,
                104 => Key::AltH,
                105 => Key::AltI,
                106 => Key::AltJ,
                107 => Key::AltK,
                108 => Key::AltL,
                109 => Key::AltM,
                110 => Key::AltN,
                111 => Key::AltO,
                112 => Key::AltP,
                113 => Key::AltQ,
                114 => Key::AltR,
                115 => Key::AltS,
                116 => Key::AltT,
                117 => Key::AltU,
                118 => Key::AltV,
                119 => Key::AltW,
                120 => Key::AltX,
                121 => Key::AltY,
                122 => Key::AltZ,
                123 => Key::AltLeftBrace,
                124 => Key::AltPipe,
                125 => Key::AltRightBrace,
                126 => Key::AltTilde,
                127 => Key::AltDelete,
                _ => match bytes[0] {
                    27 => Key::AltUnicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }
        3 => {
            let key = match bytes[1..3] {
                [79, 80] => Key::F1,       // 27,79,80
                [79, 81] => Key::F2,       // 27,79,81
                [79, 82] => Key::F3,       // 27,79,82
                [79, 83] => Key::F4,       // 27,79,83
                [91, 65] => Key::Up,       // 27,91,65
                [91, 66] => Key::Down,     // 27,91,66
                [91, 68] => Key::Left,     // 27,91,68
                [91, 67] => Key::Right,    // 27,91,67
                [91, 72] => Key::Home,     // 27,91,72
                [91, 70] => Key::End,      // 27,91,70
                [91, 90] => Key::ShiftTab, // 27,91,90
                _ => match bytes[0] {
                    27 => Key::AltUnicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }
        4 => {
            let key = match bytes[2] {
                49 => Key::Home,   // 27,91,49,126
                50 => Key::Insert, // 27,91,50,126
                51 => Key::Delete, // 27,91,51.126
                52 => Key::End,    // 27,91,52.126
                53 => Key::PgUp,   // 27,91,53,126
                54 => Key::PgDn,   // 27.91.54.126
                _ => match bytes[0] {
                    27 => Key::AltUnicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }

        5 => {
            let key = match bytes[2..4] {
                [49, 53] => Key::F5,   //      27,91,49,53,126
                [49, 55] => Key::F6,   //      27,91,49,55,126
                [49, 56] => Key::F7,   //      27,91,49,56,126
                [49, 57] => Key::F8,   //      27,91,49,57,126
                [50, 48] => Key::F9,   //      27,91,50,48,126
                [50, 49] => Key::F10,  //      27,91,50,49,126
                [50, 51] => Key::F11,  //      27,91,50,51,126
                [50, 52] => Key::F12,  //      27,91,50,52,126
                [50, 57] => Key::Menu, //      27,91,50,57,126
                _ => match bytes[0] {
                    27 => Key::AltUnicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }

        6 => {
            let key = match bytes[4..6] {
                [50, 80] => Key::ShiftF1,          //      27,91,49,59,50,80
                [51, 80] => Key::AltF1,            //      27,91,49,59,51,80
                [52, 80] => Key::AltShiftF1,       //      27,91,49,59,52,80
                [53, 80] => Key::CtrlF1,           //      27,91,49,59,53,80
                [54, 80] => Key::CtrlShiftF1,      //      27,91,49,59,54,80
                [55, 80] => Key::AltCtrlF1,        //      27,91,49,59,55,80
                [56, 80] => Key::AltCtrlShiftF1,   //      27,91,49,59,56,80
                [50, 81] => Key::ShiftF2,          //      27,91,49,59,50,81
                [51, 81] => Key::AltF2,            //      27,91,49,59,51,81
                [52, 81] => Key::AltShiftF2,       //      27,91,49,59,52,81
                [53, 81] => Key::CtrlF2,           //      27,91,49,59,53,81
                [54, 81] => Key::CtrlShiftF2,      //      27,91,49,59,54,81
                [55, 81] => Key::AltCtrlF2,        //      27,91,49,59,55,81
                [56, 81] => Key::AltCtrlShiftF2,   //      27,91,49,59,56,81
                [50, 82] => Key::ShiftF3,          //      27,91,49,59,50,82
                [51, 82] => Key::AltF3,            //      27,91,49,59,51,82
                [52, 82] => Key::AltShiftF3,       //      27,91,49,59,52,82
                [53, 82] => Key::CtrlF3,           //      27,91,49,59,53,82
                [54, 82] => Key::CtrlShiftF3,      //      27,91,49,59,54,82
                [55, 82] => Key::AltCtrlF3,        //      27,91,49,59,55,82
                [56, 82] => Key::AltCtrlShiftF3,   //      27,91,49,59,56,82
                [50, 83] => Key::ShiftF4,          //      27,91,49,59,50,83
                [51, 83] => Key::AltF4,            //      27,91,49,59,51,83
                [52, 83] => Key::AltShiftF4,       //      27,91,49,59,52,83
                [53, 83] => Key::CtrlF4,           //      27,91,49,59,53,83
                [54, 83] => Key::CtrlShiftF4,      //      27,91,49,59,54,83
                [55, 83] => Key::AltCtrlF4,        //      27,91,49,59,55,83
                [56, 83] => Key::AltCtrlShiftF4,   //      27,91,49,59,56,83
                [50, 68] => Key::ShiftLeft,        //      27,91,49,59,50,68
                [51, 68] => Key::AltLeft,          //      27,91,49,59,51,68
                [52, 68] => Key::AltShiftLeft,     //      27,91,49,59,52,68
                [53, 68] => Key::CtrlLeft,         //      27,91,49,59,53,68
                [54, 68] => Key::CtrlShiftLeft,    //      27,91,49,59,54,68
                [55, 68] => Key::AltCtrlLeft,      //      27,91,49,59,55,68
                [56, 68] => Key::AltCtrlShiftLeft, //      27,91,49,59,56,68

                [50, 65] => Key::ShiftUp,        //      27,91,49,59,50,65
                [51, 65] => Key::AltUp,          //      27,91,49,59,51,65
                [52, 65] => Key::AltShiftUp,     //      27,91,49,59,52,65
                [53, 65] => Key::CtrlUp,         //      27,91,49,59,53,65
                [54, 65] => Key::CtrlShiftUp,    //      27,91,49,59,54,65
                [55, 65] => Key::AltCtrlUp,      //      27,91,49,59,55,65
                [56, 65] => Key::AltCtrlShiftUp, //      27,91,49,59,56,65

                [50, 67] => Key::ShiftRight,    //      27,91,49,59,50,67
                [51, 67] => Key::AltRight,      //      27,91,49,59,51,67
                [52, 67] => Key::AltShiftRight, //      27,91,49,59,52,67
                [53, 67] => Key::CtrlRight,     //      27,91,49,59,53,67
                [54, 67] => Key::CtrlShiftRight, //      27,91,49,59,54,67
                [55, 67] => Key::AltCtrlRight,  //      27,91,49,59,55,67
                [56, 67] => Key::AltCtrlShiftRight, //      27,91,49,59,56,67

                [50, 66] => Key::ShiftDown,        // 27,91,49,59,50,66
                [51, 66] => Key::AltDown,          // 27,91,49,59,51,66
                [52, 66] => Key::AltShiftDown,     // 27,91,49,59,52,66
                [53, 66] => Key::CtrlDown,         // 27,91,49,59,53,66
                [54, 66] => Key::CtrlShiftDown,    // 27,91,49,59,54,66
                [55, 66] => Key::AltCtrlDown,      // 27,91,49,59,55,66
                [56, 66] => Key::AltCtrlShiftDown, // 27,91,49,59,56,66
                _ => match bytes[0] {
                    27 => Key::AltUnicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }

        7 => {
            let key = match bytes[3..6] {
                [49, 53] => Key::F5,                  //      27,91,49,53,126
                [53, 59, 50] => Key::ShiftF5,         //      27,91,49,53,59,50,126
                [53, 59, 51] => Key::AltF5,           //      27,91,49,53,59,51,126
                [53, 59, 52] => Key::AltShiftF5,      //      27,91,49,53,59,52,126
                [53, 59, 53] => Key::CtrlF5,          //      27,91,49,53,59,53,126
                [53, 59, 54] => Key::CtrlShiftF5,     //      27,91,49,53,59,54,126
                [53, 59, 55] => Key::AltCtrlF5,       //      27,91,49,53,59,55,126
                [53, 59, 56] => Key::AltCtrlShiftF5,  //      27,91,49,53,59,56,126
                [55, 59, 50] => Key::ShiftF6,         //      27,91,49,55,59,50,126
                [55, 59, 51] => Key::AltF6,           //      27,91,49,55,59,51,126
                [55, 59, 52] => Key::AltShiftF6,      //      27,91,49,55,59,52,126
                [55, 59, 53] => Key::CtrlF6,          //      27,91,49,55,59,53,126
                [55, 59, 54] => Key::CtrlShiftF6,     //      27,91,49,55,59,54,126
                [55, 59, 55] => Key::AltCtrlF6,       //      27,91,49,55,59,55,126
                [55, 59, 56] => Key::AltCtrlShiftF6,  //      27,91,49,55,59,56,126
                [56, 59, 50] => Key::ShiftF7,         //      27,91,49,56,59,50,126
                [56, 59, 51] => Key::AltF7,           //      27,91,49,56,59,51,126
                [56, 59, 52] => Key::AltShiftF7,      //      27,91,49,56,59,52,126
                [56, 59, 53] => Key::CtrlF7,          //      27,91,49,56,59,53,126
                [56, 59, 54] => Key::CtrlShiftF7,     //      27,91,49,56,59,54,126
                [56, 59, 55] => Key::AltCtrlF7,       //      27,91,49,56,59,55,126
                [56, 59, 56] => Key::AltCtrlShiftF7,  //      27,91,49,56,59,56,126
                [57, 59, 50] => Key::ShiftF8,         //      27,91,49,57,59,50,126
                [57, 59, 51] => Key::AltF8,           //      27,91,49,57,59,51,126
                [57, 59, 52] => Key::AltShiftF8,      //      27,91,49,57,59,52,126
                [57, 59, 53] => Key::CtrlF8,          //      27,91,49,57,59,53,126
                [57, 59, 54] => Key::CtrlShiftF8,     //      27,91,49,57,59,54,126
                [57, 59, 55] => Key::AltCtrlF8,       //      27,91,49,57,59,55,126
                [57, 59, 56] => Key::AltCtrlShiftF8,  //      27,91,49,57,59,56,126
                [48, 59, 50] => Key::ShiftF9,         //      27,91,49,48,59,50,126
                [48, 59, 51] => Key::AltF9,           //      27,91,49,48,59,51,126
                [48, 59, 52] => Key::AltShiftF9,      //      27,91,49,48,59,52,126
                [48, 59, 53] => Key::CtrlF9,          //      27,91,49,48,59,53,126
                [48, 59, 54] => Key::CtrlShiftF9,     //      27,91,49,48,59,54,126
                [48, 59, 55] => Key::AltCtrlF9,       //      27,91,49,48,59,55,126
                [48, 59, 56] => Key::AltCtrlShiftF9,  //      27,91,49,48,59,56,126
                [49, 59, 50] => Key::ShiftF10,        //      27,91,49,49,59,50,126
                [49, 59, 51] => Key::AltF10,          //      27,91,49,49,59,51,126
                [49, 59, 52] => Key::AltShiftF10,     //      27,91,49,49,59,52,126
                [49, 59, 53] => Key::CtrlF10,         //      27,91,49,49,59,53,126
                [49, 59, 54] => Key::CtrlShiftF10,    //      27,91,49,49,59,54,126
                [49, 59, 55] => Key::AltCtrlF10,      //      27,91,49,49,59,55,126
                [49, 59, 56] => Key::AltCtrlShiftF10, //      27,91,49,49,59,56,126
                [51, 59, 50] => Key::ShiftF11,        //      27,91,49,51,59,50,126
                [51, 59, 51] => Key::AltF11,          //      27,91,49,51,59,51,126
                [51, 59, 52] => Key::AltShiftF11,     //      27,91,49,51,59,52,126
                [51, 59, 53] => Key::CtrlF11,         //      27,91,49,51,59,53,126
                [51, 59, 54] => Key::CtrlShiftF11,    //      27,91,49,51,59,54,126
                [51, 59, 55] => Key::AltCtrlF11,      //      27,91,49,51,59,55,126
                [51, 59, 56] => Key::AltCtrlShiftF11, //      27,91,49,51,59,56,126
                [52, 59, 50] => Key::ShiftF12,        //      27,91,49,52,59,50,126
                [52, 59, 51] => Key::AltF12,          //      27,91,49,52,59,51,126
                [52, 59, 52] => Key::AltShiftF12,     //      27,91,49,52,59,52,126
                [52, 59, 53] => Key::CtrlF12,         //      27,91,49,52,59,53,126
                [52, 59, 54] => Key::CtrlShiftF12,    //      27,91,49,52,59,54,126
                [52, 59, 55] => Key::AltCtrlF12,      //      27,91,49,52,59,55,126
                [52, 59, 56] => Key::AltCtrlShiftF12, //      27,91,49,52,59,56,126
                _ => match bytes[0] {
                    27 => Key::AltUnicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }
        _ => match bytes[0] {
            27 => Some(Key::AltUnicode(bytes)),
            _ => Some(Key::Unicode(bytes)),
        },
    }
}

/// Map &str to Key
pub fn str_to_key(s: &str) -> Option<Key> {
    let mut s_to_key = HashMap::from([
        ("CtrlA", Key::CtrlA),
        ("CtrlB", Key::CtrlB),
        ("CtrlC", Key::CtrlC),
        ("CtrlD", Key::CtrlD),
        ("CtrlE", Key::CtrlE),
        ("CtrlF", Key::CtrlF),
        ("CtrlG", Key::CtrlG),
        ("Backspace", Key::Backspace),
        ("Tab", Key::Tab),
        ("Enter", Key::Enter),
        ("CtrlK", Key::CtrlK),
        ("CtrlL", Key::CtrlL),
        ("CtrlM", Key::CtrlM),
        ("CtrlN", Key::CtrlN),
        ("CtrlO", Key::CtrlO),
        ("CtrlP", Key::CtrlP),
        ("CtrlQ", Key::CtrlQ),
        ("CtrlR", Key::CtrlR),
        ("CtrlS", Key::CtrlS),
        ("CtrlT", Key::CtrlT),
        ("CtrlU", Key::CtrlU),
        ("CtrlV", Key::CtrlV),
        ("CtrlW", Key::CtrlW),
        ("CtrlX", Key::CtrlX),
        ("CtrlY", Key::CtrlY),
        ("CtrlZ", Key::CtrlZ),
        ("Escape", Key::Escape),
        ("FileSeparator", Key::FileSeparator),
        ("GroupSeparator", Key::GroupSeparator),
        ("RecordSeparator", Key::RecordSeparator),
        ("UnitSeparator", Key::UnitSeparator),
        ("Space", Key::Space),
        ("ExclamationMark", Key::ExclamationMark),
        ("Quote", Key::Quote),
        ("Hash", Key::Hash),
        ("Dollar", Key::Dollar),
        ("Percent", Key::Percent),
        ("Ampersand", Key::Ampersand),
        ("Apostrophe", Key::Apostrophe),
        ("LeftParen", Key::LeftParen),
        ("RightParen", Key::RightParen),
        ("Star", Key::Star),
        ("Plus", Key::Plus),
        ("Comma", Key::Comma),
        ("Dash", Key::Dash),
        ("Period", Key::Period),
        ("Slash", Key::Slash),
        ("Zero", Key::Zero),
        ("One", Key::One),
        ("Two", Key::Two),
        ("Three", Key::Three),
        ("Four", Key::Four),
        ("Five", Key::Five),
        ("Six", Key::Six),
        ("Seven", Key::Seven),
        ("Eight", Key::Eight),
        ("Nine", Key::Nine),
        ("Colon", Key::Colon),
        ("Semicolon", Key::Semicolon),
        ("LessThan", Key::LessThan),
        ("Equals", Key::Equals),
        ("GreaterThan", Key::GreaterThan),
        ("QuestionMark", Key::QuestionMark),
        ("At", Key::At),
        ("ShiftA", Key::ShiftA),
        ("ShiftB", Key::ShiftB),
        ("ShiftC", Key::ShiftC),
        ("ShiftD", Key::ShiftD),
        ("ShiftE", Key::ShiftE),
        ("ShiftF", Key::ShiftF),
        ("ShiftG", Key::ShiftG),
        ("ShiftH", Key::ShiftH),
        ("ShiftI", Key::ShiftI),
        ("ShiftJ", Key::ShiftJ),
        ("ShiftK", Key::ShiftK),
        ("ShiftL", Key::ShiftL),
        ("ShiftM", Key::ShiftM),
        ("ShiftN", Key::ShiftN),
        ("ShiftO", Key::ShiftO),
        ("ShiftP", Key::ShiftP),
        ("ShiftQ", Key::ShiftQ),
        ("ShiftR", Key::ShiftR),
        ("ShiftS", Key::ShiftS),
        ("ShiftT", Key::ShiftT),
        ("ShiftU", Key::ShiftU),
        ("ShiftV", Key::ShiftV),
        ("ShiftW", Key::ShiftW),
        ("ShiftX", Key::ShiftX),
        ("ShiftY", Key::ShiftY),
        ("ShiftZ", Key::ShiftZ),
        ("LeftBracket", Key::LeftBracket),
        ("BackSlash", Key::BackSlash),
        ("RightBracket", Key::RightBracket),
        ("Caret", Key::Caret),
        ("Underscore", Key::Underscore),
        ("BackTick", Key::BackTick),
        ("A", Key::A),
        ("B", Key::B),
        ("C", Key::C),
        ("D", Key::D),
        ("E", Key::E),
        ("F", Key::F),
        ("G", Key::G),
        ("H", Key::H),
        ("I", Key::I),
        ("J", Key::J),
        ("K", Key::K),
        ("L", Key::L),
        ("M", Key::M),
        ("N", Key::N),
        ("O", Key::O),
        ("P", Key::P),
        ("Q", Key::Q),
        ("R", Key::R),
        ("S", Key::S),
        ("T", Key::T),
        ("U", Key::U),
        ("V", Key::V),
        ("W", Key::W),
        ("X", Key::X),
        ("Y", Key::Y),
        ("Z", Key::Z),
        ("LeftBrace", Key::LeftBrace),
        ("Pipe", Key::Pipe),
        ("RightBrace", Key::RightBrace),
        ("Tilde", Key::Tilde),
        ("Delete", Key::Delete),
        ("AltCtrlA", Key::AltCtrlA),
        ("AltCtrlB", Key::AltCtrlB),
        ("AltCtrlC", Key::AltCtrlC),
        ("AltCtrlD", Key::AltCtrlD),
        ("AltCtrlE", Key::AltCtrlE),
        ("AltCtrlF", Key::AltCtrlF),
        ("AltCtrlG", Key::AltCtrlG),
        ("AltCtrlH", Key::AltCtrlH),
        ("AltTab", Key::AltTab),
        ("AltEnter", Key::AltEnter),
        ("AltCtrlK", Key::AltCtrlK),
        ("AltCtrlL", Key::AltCtrlL),
        ("AltCtrlM", Key::AltCtrlM),
        ("AltCtrlN", Key::AltCtrlN),
        ("AltCtrlO", Key::AltCtrlO),
        ("AltCtrlP", Key::AltCtrlP),
        ("AltCtrlQ", Key::AltCtrlQ),
        ("AltCtrlR", Key::AltCtrlR),
        ("AltCtrlS", Key::AltCtrlS),
        ("AltCtrlT", Key::AltCtrlT),
        ("AltCtrlU", Key::AltCtrlU),
        ("AltCtrlV", Key::AltCtrlV),
        ("AltCtrlW", Key::AltCtrlW),
        ("AltCtrlX", Key::AltCtrlX),
        ("AltCtrlY", Key::AltCtrlY),
        ("AltCtrlZ", Key::AltCtrlZ),
        ("AltEscape", Key::AltEscape),
        ("AltFileSeparator", Key::AltFileSeparator),
        ("AltGroupSeparator", Key::AltGroupSeparator),
        ("AltRecordSeparator", Key::AltRecordSeparator),
        ("AltUnitSeparator", Key::AltUnitSeparator),
        ("AltSpace", Key::AltSpace),
        ("AltExclamationMark", Key::AltExclamationMark),
        ("AltQuote", Key::AltQuote),
        ("AltHash", Key::AltHash),
        ("AltDollar", Key::AltDollar),
        ("AltPercent", Key::AltPercent),
        ("AltAmpersand", Key::AltAmpersand),
        ("AltApostrophe", Key::AltApostrophe),
        ("AltLeftParen", Key::AltLeftParen),
        ("AltRightParen", Key::AltRightParen),
        ("AltStar", Key::AltStar),
        ("AltPlus", Key::AltPlus),
        ("AltComma", Key::AltComma),
        ("AltDash", Key::AltDash),
        ("AltPeriod", Key::AltPeriod),
        ("AltSlash", Key::AltSlash),
        ("AltZero", Key::AltZero),
        ("AltOne", Key::AltOne),
        ("AltTwo", Key::AltTwo),
        ("AltThree", Key::AltThree),
        ("AltFour", Key::AltFour),
        ("AltFive", Key::AltFive),
        ("AltSix", Key::AltSix),
        ("AltSeven", Key::AltSeven),
        ("AltEight", Key::AltEight),
        ("AltNine", Key::AltNine),
        ("AltColon", Key::AltColon),
        ("AltSemicolon", Key::AltSemicolon),
        ("AltLessThan", Key::AltLessThan),
        ("AltEquals", Key::AltEquals),
        ("AltGreaterThan", Key::AltGreaterThan),
        ("AltQuestionMark", Key::AltQuestionMark),
        ("AltAt", Key::AltAt),
        ("AltShiftA", Key::AltShiftA),
        ("AltShiftB", Key::AltShiftB),
        ("AltShiftC", Key::AltShiftC),
        ("AltShiftD", Key::AltShiftD),
        ("AltShiftE", Key::AltShiftE),
        ("AltShiftF", Key::AltShiftF),
        ("AltShiftG", Key::AltShiftG),
        ("AltShiftH", Key::AltShiftH),
        ("AltShiftI", Key::AltShiftI),
        ("AltShiftJ", Key::AltShiftJ),
        ("AltShiftK", Key::AltShiftK),
        ("AltShiftL", Key::AltShiftL),
        ("AltShiftM", Key::AltShiftM),
        ("AltShiftN", Key::AltShiftN),
        ("AltShiftO", Key::AltShiftO),
        ("AltShiftP", Key::AltShiftP),
        ("AltShiftQ", Key::AltShiftQ),
        ("AltShiftR", Key::AltShiftR),
        ("AltShiftS", Key::AltShiftS),
        ("AltShiftT", Key::AltShiftT),
        ("AltShiftU", Key::AltShiftU),
        ("AltShiftV", Key::AltShiftV),
        ("AltShiftW", Key::AltShiftW),
        ("AltShiftX", Key::AltShiftX),
        ("AltShiftY", Key::AltShiftY),
        ("AltShiftZ", Key::AltShiftZ),
        ("AltLeftBracket", Key::AltLeftBracket),
        ("AltBackSlash", Key::AltBackSlash),
        ("AltRightBracket", Key::AltRightBracket),
        ("AltCaret", Key::AltCaret),
        ("AltUnderscore", Key::AltUnderscore),
        ("AltBackTick", Key::AltBackTick),
        ("AltA", Key::AltA),
        ("AltB", Key::AltB),
        ("AltC", Key::AltC),
        ("AltD", Key::AltD),
        ("AltE", Key::AltE),
        ("AltF", Key::AltF),
        ("AltG", Key::AltG),
        ("AltH", Key::AltH),
        ("AltI", Key::AltI),
        ("AltJ", Key::AltJ),
        ("AltK", Key::AltK),
        ("AltL", Key::AltL),
        ("AltM", Key::AltM),
        ("AltN", Key::AltN),
        ("AltO", Key::AltO),
        ("AltP", Key::AltP),
        ("AltQ", Key::AltQ),
        ("AltR", Key::AltR),
        ("AltS", Key::AltS),
        ("AltT", Key::AltT),
        ("AltU", Key::AltU),
        ("AltV", Key::AltV),
        ("AltW", Key::AltW),
        ("AltX", Key::AltX),
        ("AltY", Key::AltY),
        ("AltZ", Key::AltZ),
        ("AltLeftBrace", Key::AltLeftBrace),
        ("AltPipe", Key::AltPipe),
        ("AltRightBrace", Key::AltRightBrace),
        ("AltTilde", Key::AltTilde),
        ("AltDelete", Key::AltDelete),
        ("ShiftLeft", Key::ShiftLeft),
        ("AltLeft", Key::AltLeft),
        ("AltShiftLeft", Key::AltShiftLeft),
        ("CtrlLeft", Key::CtrlLeft),
        ("CtrlShiftLeft", Key::CtrlShiftLeft),
        ("AltCtrlLeft", Key::AltCtrlLeft),
        ("AltCtrlShiftLeft", Key::AltCtrlShiftLeft),
        ("ShiftUp", Key::ShiftUp),
        ("AltUp", Key::AltUp),
        ("AltShiftUp", Key::AltShiftUp),
        ("CtrlUp", Key::CtrlUp),
        ("CtrlShiftUp", Key::CtrlShiftUp),
        ("AltCtrlUp", Key::AltCtrlUp),
        ("AltCtrlShiftUp", Key::AltCtrlShiftUp),
        ("ShiftRight", Key::ShiftRight),
        ("AltRight", Key::AltRight),
        ("AltShiftRight", Key::AltShiftRight),
        ("CtrlRight", Key::CtrlRight),
        ("CtrlShiftRight", Key::CtrlShiftRight),
        ("AltCtrlRight", Key::AltCtrlRight),
        ("AltCtrlShiftRight", Key::AltCtrlShiftRight),
        ("ShiftDown", Key::ShiftDown),
        ("AltDown", Key::AltDown),
        ("AltShiftDown", Key::AltShiftDown),
        ("CtrlDown", Key::CtrlDown),
        ("CtrlShiftDown", Key::CtrlShiftDown),
        ("AltCtrlDown", Key::AltCtrlDown),
        ("AltCtrlShiftDown", Key::AltCtrlShiftDown),
        ("F1", Key::F1),
        ("ShiftF1", Key::ShiftF1),
        ("AltF1", Key::AltF1),
        ("AltShiftF1", Key::AltShiftF1),
        ("CtrlF1", Key::CtrlF1),
        ("CtrlShiftF1", Key::CtrlShiftF1),
        ("AltCtrlF1", Key::AltCtrlF1),
        ("AltCtrlShiftF1", Key::AltCtrlShiftF1),
        ("F2", Key::F2),
        ("ShiftF2", Key::ShiftF2),
        ("AltF2", Key::AltF2),
        ("AltShiftF2", Key::AltShiftF2),
        ("CtrlF2", Key::CtrlF2),
        ("CtrlShiftF2", Key::CtrlShiftF2),
        ("AltCtrlF2", Key::AltCtrlF2),
        ("AltCtrlShiftF2", Key::AltCtrlShiftF2),
        ("F3", Key::F3),
        ("ShiftF3", Key::ShiftF3),
        ("AltF3", Key::AltF3),
        ("AltShiftF3", Key::AltShiftF3),
        ("CtrlF3", Key::CtrlF3),
        ("CtrlShiftF3", Key::CtrlShiftF3),
        ("AltCtrlF3", Key::AltCtrlF3),
        ("AltCtrlShiftF3", Key::AltCtrlShiftF3),
        ("F4", Key::F4),
        ("ShiftF4", Key::ShiftF4),
        ("AltF4", Key::AltF4),
        ("AltShiftF4", Key::AltShiftF4),
        ("CtrlF4", Key::CtrlF4),
        ("CtrlShiftF4", Key::CtrlShiftF4),
        ("AltCtrlF4", Key::AltCtrlF4),
        ("AltCtrlShiftF4", Key::AltCtrlShiftF4),
        ("F5", Key::F5),
        ("ShiftF5", Key::ShiftF5),
        ("AltF5", Key::AltF5),
        ("AltShiftF5", Key::AltShiftF5),
        ("CtrlF5", Key::CtrlF5),
        ("CtrlShiftF5", Key::CtrlShiftF5),
        ("AltCtrlF5", Key::AltCtrlF5),
        ("AltCtrlShiftF5", Key::AltCtrlShiftF5),
        ("F6", Key::F6),
        ("ShiftF6", Key::ShiftF6),
        ("AltF6", Key::AltF6),
        ("AltShiftF6", Key::AltShiftF6),
        ("CtrlF6", Key::CtrlF6),
        ("CtrlShiftF6", Key::CtrlShiftF6),
        ("AltCtrlF6", Key::AltCtrlF6),
        ("AltCtrlShiftF6", Key::AltCtrlShiftF6),
        ("F7", Key::F7),
        ("ShiftF7", Key::ShiftF7),
        ("AltF7", Key::AltF7),
        ("AltShiftF7", Key::AltShiftF7),
        ("CtrlF7", Key::CtrlF7),
        ("CtrlShiftF7", Key::CtrlShiftF7),
        ("AltCtrlF7", Key::AltCtrlF7),
        ("AltCtrlShiftF7", Key::AltCtrlShiftF7),
        ("F8", Key::F8),
        ("ShiftF8", Key::ShiftF8),
        ("AltF8", Key::AltF8),
        ("AltShiftF8", Key::AltShiftF8),
        ("CtrlF8", Key::CtrlF8),
        ("CtrlShiftF8", Key::CtrlShiftF8),
        ("AltCtrlF8", Key::AltCtrlF8),
        ("AltCtrlShiftF8", Key::AltCtrlShiftF8),
        ("F9", Key::F9),
        ("ShiftF9", Key::ShiftF9),
        ("AltF9", Key::AltF9),
        ("AltShiftF9", Key::AltShiftF9),
        ("CtrlF9", Key::CtrlF9),
        ("CtrlShiftF9", Key::CtrlShiftF9),
        ("AltCtrlF9", Key::AltCtrlF9),
        ("AltCtrlShiftF9", Key::AltCtrlShiftF9),
        ("F10", Key::F10),
        ("ShiftF10", Key::ShiftF10),
        ("AltF10", Key::AltF10),
        ("AltShiftF10", Key::AltShiftF10),
        ("CtrlF10", Key::CtrlF10),
        ("CtrlShiftF10", Key::CtrlShiftF10),
        ("AltCtrlF10", Key::AltCtrlF10),
        ("AltCtrlShiftF10", Key::AltCtrlShiftF10),
        ("F11", Key::F11),
        ("ShiftF11", Key::ShiftF11),
        ("AltF11", Key::AltF11),
        ("AltShiftF11", Key::AltShiftF11),
        ("CtrlF11", Key::CtrlF11),
        ("CtrlShiftF11", Key::CtrlShiftF11),
        ("AltCtrlF11", Key::AltCtrlF11),
        ("AltCtrlShiftF11", Key::AltCtrlShiftF11),
        ("F12", Key::F12),
        ("ShiftF12", Key::ShiftF12),
        ("AltF12", Key::AltF12),
        ("AltShiftF12", Key::AltShiftF12),
        ("CtrlF12", Key::CtrlF12),
        ("CtrlShiftF12", Key::CtrlShiftF12),
        ("AltCtrlF12", Key::AltCtrlF12),
        ("AltCtrlShiftF12", Key::AltCtrlShiftF12),
        ("Up", Key::Up),
        ("Down", Key::Down),
        ("Left", Key::Left),
        ("Right", Key::Right),
        ("Insert", Key::Insert),
        ("Home", Key::Home),
        ("PgUp", Key::PgUp),
        ("PgDn", Key::PgDn),
        ("End", Key::End),
        ("Menu", Key::Menu),
        ("ShiftTab", Key::ShiftTab),
    ]);
    s_to_key.remove(s)
}
