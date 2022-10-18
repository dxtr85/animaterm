use super::key::Key;
use std::collections::HashMap;
use std::env;
use std::process::Command;

pub fn ask_os_for_rows_and_cols() -> (usize, usize) {
    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
        .collect();
    let rows = match Command::new("tput")
        .arg("lines")
        .env_clear()
        .envs(&filtered_env)
        .output()
    {
        Ok(data) => {
            let output = String::from_utf8(data.stdout);
            if output.is_ok() {
                let output = output.unwrap();
                let number = usize::from_str_radix(output.trim(), 10);
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
            if output.is_ok() {
                let output = output.unwrap();
                let number = usize::from_str_radix(output.trim(), 10);
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
                51 => Key::Delete, // 27,91,51.126
                50 => Key::Insert, // 27,91,50,126
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
