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
            let output = String::from_utf8(data.stdout).unwrap();
            let number = usize::from_str_radix(output.trim(), 10);
            match number {
                Ok(a_number) => a_number,
                Err(e) => {
                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to determine lines count from {}, using defaults\n{}", output, e);
                    35
                }
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
            let output = String::from_utf8(data.stdout).unwrap();
            let number = usize::from_str_radix(output.trim(), 10);
            match number {
                Ok(a_number) => a_number,
                Err(e) => {
                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to determine cols count from {}, using defaults\n{}", output, e);
                    80
                }
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
                1 => Key::Ctrl_a,
                2 => Key::Ctrl_b,
                3 => Key::Ctrl_c,
                4 => Key::Ctrl_d,
                5 => Key::Ctrl_e,
                6 => Key::Ctrl_f,
                7 => Key::Ctrl_g,
                8 => Key::Backspace,
                9 => Key::Tab,
                10 => Key::Enter,
                11 => Key::Ctrl_k,
                12 => Key::Ctrl_l,
                13 => Key::Ctrl_m,
                14 => Key::Ctrl_n,
                15 => Key::Ctrl_o,
                16 => Key::Ctrl_p,
                17 => Key::Ctrl_q,
                18 => Key::Ctrl_r,
                19 => Key::Ctrl_s,
                20 => Key::Ctrl_t,
                21 => Key::Ctrl_u,
                22 => Key::Ctrl_v,
                23 => Key::Ctrl_w,
                24 => Key::Ctrl_x,
                25 => Key::Ctrl_y,
                26 => Key::Ctrl_z,
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
                65 => Key::A,
                66 => Key::B,
                67 => Key::C,
                68 => Key::D,
                69 => Key::E,
                70 => Key::F,
                71 => Key::G,
                72 => Key::H,
                73 => Key::I,
                74 => Key::J,
                75 => Key::K,
                76 => Key::L,
                77 => Key::M,
                78 => Key::N,
                79 => Key::O,
                80 => Key::P,
                81 => Key::Q,
                82 => Key::R,
                83 => Key::S,
                84 => Key::T,
                85 => Key::U,
                86 => Key::V,
                87 => Key::W,
                88 => Key::X,
                89 => Key::Y,
                90 => Key::Z,
                91 => Key::LeftBracket,
                92 => Key::BackSlash,
                93 => Key::RightBracket,
                94 => Key::Caret,
                95 => Key::Underscore,
                96 => Key::BackTick,
                97 => Key::a,
                98 => Key::b,
                99 => Key::c,
                100 => Key::d,
                101 => Key::e,
                102 => Key::f,
                103 => Key::g,
                104 => Key::h,
                105 => Key::i,
                106 => Key::j,
                107 => Key::k,
                108 => Key::l,
                109 => Key::m,
                110 => Key::n,
                111 => Key::o,
                112 => Key::p,
                113 => Key::q,
                114 => Key::r,
                115 => Key::s,
                116 => Key::t,
                117 => Key::u,
                118 => Key::v,
                119 => Key::w,
                120 => Key::x,
                121 => Key::y,
                122 => Key::z,
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
                1 => Key::Alt_Ctrl_a,
                2 => Key::Alt_Ctrl_b,
                3 => Key::Alt_Ctrl_c,
                4 => Key::Alt_Ctrl_d,
                5 => Key::Alt_Ctrl_e,
                6 => Key::Alt_Ctrl_f,
                7 => Key::Alt_Ctrl_g,
                8 => Key::Alt_Ctrl_h,
                9 => Key::Alt_Tab,
                10 => Key::Alt_Enter,
                11 => Key::Alt_Ctrl_k,
                12 => Key::Alt_Ctrl_l,
                13 => Key::Alt_Ctrl_m,
                14 => Key::Alt_Ctrl_n,
                15 => Key::Alt_Ctrl_o,
                16 => Key::Alt_Ctrl_p,
                17 => Key::Alt_Ctrl_q,
                18 => Key::Alt_Ctrl_r,
                19 => Key::Alt_Ctrl_s,
                20 => Key::Alt_Ctrl_t,
                21 => Key::Alt_Ctrl_u,
                22 => Key::Alt_Ctrl_v,
                23 => Key::Alt_Ctrl_w,
                24 => Key::Alt_Ctrl_x,
                25 => Key::Alt_Ctrl_y,
                26 => Key::Alt_Ctrl_z,
                27 => Key::Alt_Escape,
                28 => Key::Alt_FileSeparator,
                29 => Key::Alt_GroupSeparator,
                30 => Key::Alt_RecordSeparator,
                31 => Key::Alt_UnitSeparator,
                32 => Key::Alt_Space,
                33 => Key::Alt_ExclamationMark,
                34 => Key::Alt_Quote,
                35 => Key::Alt_Hash,
                36 => Key::Alt_Dollar,
                37 => Key::Alt_Percent,
                38 => Key::Alt_Ampersand,
                39 => Key::Alt_Apostrophe,
                40 => Key::Alt_LeftParen,
                41 => Key::Alt_RightParen,
                42 => Key::Alt_Star,
                43 => Key::Alt_Plus,
                44 => Key::Alt_Comma,
                45 => Key::Alt_Dash,
                46 => Key::Alt_Period,
                47 => Key::Alt_Slash,
                48 => Key::Alt_Zero,
                49 => Key::Alt_One,
                50 => Key::Alt_Two,
                51 => Key::Alt_Three,
                52 => Key::Alt_Four,
                53 => Key::Alt_Five,
                54 => Key::Alt_Six,
                55 => Key::Alt_Seven,
                56 => Key::Alt_Eight,
                57 => Key::Alt_Nine,
                58 => Key::Alt_Colon,
                59 => Key::Alt_Semicolon,
                60 => Key::Alt_LessThan,
                61 => Key::Alt_Equals,
                62 => Key::Alt_GreaterThan,
                63 => Key::Alt_QuestionMark,
                64 => Key::Alt_At,
                65 => Key::Alt_A,
                66 => Key::Alt_B,
                67 => Key::Alt_C,
                68 => Key::Alt_D,
                69 => Key::Alt_E,
                70 => Key::Alt_F,
                71 => Key::Alt_G,
                72 => Key::Alt_H,
                73 => Key::Alt_I,
                74 => Key::Alt_J,
                75 => Key::Alt_K,
                76 => Key::Alt_L,
                77 => Key::Alt_M,
                78 => Key::Alt_N,
                79 => Key::Alt_O,
                80 => Key::Alt_P,
                81 => Key::Alt_Q,
                82 => Key::Alt_R,
                83 => Key::Alt_S,
                84 => Key::Alt_T,
                85 => Key::Alt_U,
                86 => Key::Alt_V,
                87 => Key::Alt_W,
                88 => Key::Alt_X,
                89 => Key::Alt_Y,
                90 => Key::Alt_Z,
                91 => Key::Alt_LeftBracket,
                92 => Key::Alt_BackSlash,
                93 => Key::Alt_RightBracket,
                94 => Key::Alt_Caret,
                95 => Key::Alt_Underscore,
                96 => Key::Alt_BackTick,
                97 => Key::Alt_a,
                98 => Key::Alt_b,
                99 => Key::Alt_c,
                100 => Key::Alt_d,
                101 => Key::Alt_e,
                102 => Key::Alt_f,
                103 => Key::Alt_g,
                104 => Key::Alt_h,
                105 => Key::Alt_i,
                106 => Key::Alt_j,
                107 => Key::Alt_k,
                108 => Key::Alt_l,
                109 => Key::Alt_m,
                110 => Key::Alt_n,
                111 => Key::Alt_o,
                112 => Key::Alt_p,
                113 => Key::Alt_q,
                114 => Key::Alt_r,
                115 => Key::Alt_s,
                116 => Key::Alt_t,
                117 => Key::Alt_u,
                118 => Key::Alt_v,
                119 => Key::Alt_w,
                120 => Key::Alt_x,
                121 => Key::Alt_y,
                122 => Key::Alt_z,
                123 => Key::Alt_LeftBrace,
                124 => Key::Alt_Pipe,
                125 => Key::Alt_RightBrace,
                126 => Key::Alt_Tilde,
                127 => Key::Alt_Delete,
                _ => match bytes[0] {
                    27 => Key::Alt_Unicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }
        3 => {
            let key = match bytes[1..3] {
                [79, 80] => Key::F1,        // 27,79,80
                [79, 81] => Key::F2,        // 27,79,81
                [79, 82] => Key::F3,        // 27,79,82
                [79, 83] => Key::F4,        // 27,79,83
                [91, 65] => Key::Up,        // 27,91,65
                [91, 66] => Key::Down,      // 27,91,66
                [91, 68] => Key::Left,      // 27,91,68
                [91, 67] => Key::Right,     // 27,91,67
                [91, 72] => Key::Home,      // 27,91,72
                [91, 70] => Key::End,       // 27,91,70
                [91, 90] => Key::Shift_Tab, // 27,91,90
                _ => match bytes[0] {
                    27 => Key::Alt_Unicode(bytes),
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
                    27 => Key::Alt_Unicode(bytes),
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
                    27 => Key::Alt_Unicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }

        6 => {
            let key = match bytes[4..6] {
                [50, 80] => Key::Shift_F1,            //      27,91,49,59,50,80
                [51, 80] => Key::Alt_F1,              //      27,91,49,59,51,80
                [52, 80] => Key::Alt_Shift_F1,        //      27,91,49,59,52,80
                [53, 80] => Key::Ctrl_F1,             //      27,91,49,59,53,80
                [54, 80] => Key::Ctrl_Shift_F1,       //      27,91,49,59,54,80
                [55, 80] => Key::Alt_Ctrl_F1,         //      27,91,49,59,55,80
                [56, 80] => Key::Alt_Ctrl_Shift_F1,   //      27,91,49,59,56,80
                [50, 81] => Key::Shift_F2,            //      27,91,49,59,50,81
                [51, 81] => Key::Alt_F2,              //      27,91,49,59,51,81
                [52, 81] => Key::Alt_Shift_F2,        //      27,91,49,59,52,81
                [53, 81] => Key::Ctrl_F2,             //      27,91,49,59,53,81
                [54, 81] => Key::Ctrl_Shift_F2,       //      27,91,49,59,54,81
                [55, 81] => Key::Alt_Ctrl_F2,         //      27,91,49,59,55,81
                [56, 81] => Key::Alt_Ctrl_Shift_F2,   //      27,91,49,59,56,81
                [50, 82] => Key::Shift_F3,            //      27,91,49,59,50,82
                [51, 82] => Key::Alt_F3,              //      27,91,49,59,51,82
                [52, 82] => Key::Alt_Shift_F3,        //      27,91,49,59,52,82
                [53, 82] => Key::Ctrl_F3,             //      27,91,49,59,53,82
                [54, 82] => Key::Ctrl_Shift_F3,       //      27,91,49,59,54,82
                [55, 82] => Key::Alt_Ctrl_F3,         //      27,91,49,59,55,82
                [56, 82] => Key::Alt_Ctrl_Shift_F3,   //      27,91,49,59,56,82
                [50, 83] => Key::Shift_F4,            //      27,91,49,59,50,83
                [51, 83] => Key::Alt_F4,              //      27,91,49,59,51,83
                [52, 83] => Key::Alt_Shift_F4,        //      27,91,49,59,52,83
                [53, 83] => Key::Ctrl_F4,             //      27,91,49,59,53,83
                [54, 83] => Key::Ctrl_Shift_F4,       //      27,91,49,59,54,83
                [55, 83] => Key::Alt_Ctrl_F4,         //      27,91,49,59,55,83
                [56, 83] => Key::Alt_Ctrl_Shift_F4,   //      27,91,49,59,56,83
                [50, 68] => Key::Shift_Left,          //      27,91,49,59,50,68
                [51, 68] => Key::Alt_Left,            //      27,91,49,59,51,68
                [52, 68] => Key::Alt_Shift_Left,      //      27,91,49,59,52,68
                [53, 68] => Key::Ctrl_Left,           //      27,91,49,59,53,68
                [54, 68] => Key::Ctrl_Shift_Left,     //      27,91,49,59,54,68
                [55, 68] => Key::Alt_Ctrl_Left,       //      27,91,49,59,55,68
                [56, 68] => Key::Alt_Ctrl_Shift_Left, //      27,91,49,59,56,68

                [50, 65] => Key::Shift_Up,      //      27,91,49,59,50,65
                [51, 65] => Key::Alt_Up,        //      27,91,49,59,51,65
                [52, 65] => Key::Alt_Shift_Up,  //      27,91,49,59,52,65
                [53, 65] => Key::Ctrl_Up,       //      27,91,49,59,53,65
                [54, 65] => Key::Ctrl_Shift_Up, //      27,91,49,59,54,65
                [55, 65] => Key::Alt_Ctrl_Up,   //      27,91,49,59,55,65
                [56, 65] => Key::Alt_Ctrl_Shift_Up, //      27,91,49,59,56,65

                [50, 67] => Key::Shift_Right, //      27,91,49,59,50,67
                [51, 67] => Key::Alt_Right,   //      27,91,49,59,51,67
                [52, 67] => Key::Alt_Shift_Right, //      27,91,49,59,52,67
                [53, 67] => Key::Ctrl_Right,  //      27,91,49,59,53,67
                [54, 67] => Key::Ctrl_Shift_Right, //      27,91,49,59,54,67
                [55, 67] => Key::Alt_Ctrl_Right, //      27,91,49,59,55,67
                [56, 67] => Key::Alt_Ctrl_Shift_Right, //      27,91,49,59,56,67

                [50, 66] => Key::Shift_Down,      // 27,91,49,59,50,66
                [51, 66] => Key::Alt_Down,        // 27,91,49,59,51,66
                [52, 66] => Key::Alt_Shift_Down,  // 27,91,49,59,52,66
                [53, 66] => Key::Ctrl_Down,       // 27,91,49,59,53,66
                [54, 66] => Key::Ctrl_Shift_Down, // 27,91,49,59,54,66
                [55, 66] => Key::Alt_Ctrl_Down,   // 27,91,49,59,55,66
                [56, 66] => Key::Alt_Ctrl_Shift_Down, // 27,91,49,59,56,66
                _ => match bytes[0] {
                    27 => Key::Alt_Unicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }

        7 => {
            let key = match bytes[3..6] {
                [49, 53] => Key::F5,                     //      27,91,49,53,126
                [53, 59, 50] => Key::Shift_F5,           //      27,91,49,53,59,50,126
                [53, 59, 51] => Key::Alt_F5,             //      27,91,49,53,59,51,126
                [53, 59, 52] => Key::Alt_Shift_F5,       //      27,91,49,53,59,52,126
                [53, 59, 53] => Key::Ctrl_F5,            //      27,91,49,53,59,53,126
                [53, 59, 54] => Key::Ctrl_Shift_F5,      //      27,91,49,53,59,54,126
                [53, 59, 55] => Key::Alt_Ctrl_F5,        //      27,91,49,53,59,55,126
                [53, 59, 56] => Key::Alt_Ctrl_Shift_F5,  //      27,91,49,53,59,56,126
                [55, 59, 50] => Key::Shift_F6,           //      27,91,49,55,59,50,126
                [55, 59, 51] => Key::Alt_F6,             //      27,91,49,55,59,51,126
                [55, 59, 52] => Key::Alt_Shift_F6,       //      27,91,49,55,59,52,126
                [55, 59, 53] => Key::Ctrl_F6,            //      27,91,49,55,59,53,126
                [55, 59, 54] => Key::Ctrl_Shift_F6,      //      27,91,49,55,59,54,126
                [55, 59, 55] => Key::Alt_Ctrl_F6,        //      27,91,49,55,59,55,126
                [55, 59, 56] => Key::Alt_Ctrl_Shift_F6,  //      27,91,49,55,59,56,126
                [56, 59, 50] => Key::Shift_F7,           //      27,91,49,56,59,50,126
                [56, 59, 51] => Key::Alt_F7,             //      27,91,49,56,59,51,126
                [56, 59, 52] => Key::Alt_Shift_F7,       //      27,91,49,56,59,52,126
                [56, 59, 53] => Key::Ctrl_F7,            //      27,91,49,56,59,53,126
                [56, 59, 54] => Key::Ctrl_Shift_F7,      //      27,91,49,56,59,54,126
                [56, 59, 55] => Key::Alt_Ctrl_F7,        //      27,91,49,56,59,55,126
                [56, 59, 56] => Key::Alt_Ctrl_Shift_F7,  //      27,91,49,56,59,56,126
                [57, 59, 50] => Key::Shift_F8,           //      27,91,49,57,59,50,126
                [57, 59, 51] => Key::Alt_F8,             //      27,91,49,57,59,51,126
                [57, 59, 52] => Key::Alt_Shift_F8,       //      27,91,49,57,59,52,126
                [57, 59, 53] => Key::Ctrl_F8,            //      27,91,49,57,59,53,126
                [57, 59, 54] => Key::Ctrl_Shift_F8,      //      27,91,49,57,59,54,126
                [57, 59, 55] => Key::Alt_Ctrl_F8,        //      27,91,49,57,59,55,126
                [57, 59, 56] => Key::Alt_Ctrl_Shift_F8,  //      27,91,49,57,59,56,126
                [48, 59, 50] => Key::Shift_F9,           //      27,91,49,48,59,50,126
                [48, 59, 51] => Key::Alt_F9,             //      27,91,49,48,59,51,126
                [48, 59, 52] => Key::Alt_Shift_F9,       //      27,91,49,48,59,52,126
                [48, 59, 53] => Key::Ctrl_F9,            //      27,91,49,48,59,53,126
                [48, 59, 54] => Key::Ctrl_Shift_F9,      //      27,91,49,48,59,54,126
                [48, 59, 55] => Key::Alt_Ctrl_F9,        //      27,91,49,48,59,55,126
                [48, 59, 56] => Key::Alt_Ctrl_Shift_F9,  //      27,91,49,48,59,56,126
                [49, 59, 50] => Key::Shift_F10,          //      27,91,49,49,59,50,126
                [49, 59, 51] => Key::Alt_F10,            //      27,91,49,49,59,51,126
                [49, 59, 52] => Key::Alt_Shift_F10,      //      27,91,49,49,59,52,126
                [49, 59, 53] => Key::Ctrl_F10,           //      27,91,49,49,59,53,126
                [49, 59, 54] => Key::Ctrl_Shift_F10,     //      27,91,49,49,59,54,126
                [49, 59, 55] => Key::Alt_Ctrl_F10,       //      27,91,49,49,59,55,126
                [49, 59, 56] => Key::Alt_Ctrl_Shift_F10, //      27,91,49,49,59,56,126
                [51, 59, 50] => Key::Shift_F11,          //      27,91,49,51,59,50,126
                [51, 59, 51] => Key::Alt_F11,            //      27,91,49,51,59,51,126
                [51, 59, 52] => Key::Alt_Shift_F11,      //      27,91,49,51,59,52,126
                [51, 59, 53] => Key::Ctrl_F11,           //      27,91,49,51,59,53,126
                [51, 59, 54] => Key::Ctrl_Shift_F11,     //      27,91,49,51,59,54,126
                [51, 59, 55] => Key::Alt_Ctrl_F11,       //      27,91,49,51,59,55,126
                [51, 59, 56] => Key::Alt_Ctrl_Shift_F11, //      27,91,49,51,59,56,126
                [52, 59, 50] => Key::Shift_F12,          //      27,91,49,52,59,50,126
                [52, 59, 51] => Key::Alt_F12,            //      27,91,49,52,59,51,126
                [52, 59, 52] => Key::Alt_Shift_F12,      //      27,91,49,52,59,52,126
                [52, 59, 53] => Key::Ctrl_F12,           //      27,91,49,52,59,53,126
                [52, 59, 54] => Key::Ctrl_Shift_F12,     //      27,91,49,52,59,54,126
                [52, 59, 55] => Key::Alt_Ctrl_F12,       //      27,91,49,52,59,55,126
                [52, 59, 56] => Key::Alt_Ctrl_Shift_F12, //      27,91,49,52,59,56,126
                _ => match bytes[0] {
                    27 => Key::Alt_Unicode(bytes),
                    _ => Key::Unicode(bytes),
                },
            };
            Some(key)
        }
        _ => match bytes[0] {
            27 => Some(Key::Alt_Unicode(bytes)),
            _ => Some(Key::Unicode(bytes)),
        },
    }
}
