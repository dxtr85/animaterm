//! # A TUI application for frames creation.
//!
//! This terminal application allows you to easily create
//! a basic building blocks of every graphic - a frame.
//! A frame is a collection of glyphs that are arranged into
//! a rectangular shape and presented on screen.
//!
//! A graphic consists of at least one frame.
//!
//! On exit Workspace is always saved to **output_file**, if defined.
//!
//! You can also save current state of Workspace by calling print_graphic function (default keybinding is **AltP**).
//!
//! You can also call print_screen (default **AltCtrlP**) in order to save entire screen to a timestamped file.
//!
//! ## Navigation
//! All navigation in studio is done via keyboard. You can customize your preffered key for every
//! available action in this app.
//!
//! ## TUI elements
//! studio consists of five interactive windows:
//! - Color
//! - Background
//! - Style
//! - Glyphs
//! - Workspace
//!
//! Color and  Background windows are used to select font color and background
//! of a glyph you are about to place on Workspace.
//!
//! Style window allows you to select font style such as Italic, Underline etc.
//!
//! Glyph window is used for selecting the shape of a glyph to be placed onto Workspace.
//!
//! Workspace is a canvas on which you build your creation.
//! ## Frame file format
//! A frame file is a regular text file enhanced with ANSI escape codes for colors and style modification.
//!
//! Each line in this file represents a single row of glyphs.
//!
//! You can view frame files by issuing `cat <frame_file>` or `less -R <frame_file>`.
//!
//! ## Graphic index file format
//! In order to define a graphic with some frames and animations you create a plaintext graphic index file.
//!
//! A graphic index file can contain frame definitions each in a separate line.
//!
//! Graphic index file can also contain animation definitions, also each in a separate line.
//! ### Frame definition
//!
//! Syntax for frame definition is: frame <frame_id> <frame_definition_file>
//!
//! > frame 9 9.txf
//!
//! ### Animation definition
//!
//! It's syntax is: animation \[loop\] \[run\] <frame_id>:<time_in_msec>
//!
//! loop is optional and if used it defines an animation that starts over after reaching last frame.
//!
//! run is also optional and if used will start this animation once a graphic is placed on screen.
//!
//! <frame_id>:<time_msec> defines a frame and it's display time. You can define a lot of these in every animation definition.
//!
//! > animation loop run 0:1000 1:1000 2:1000 3:1000 4:1000 5:1000 6:1000 7:1000 8:1000 9:1000

//!
//! ## Configuration file
//! Please see [default_config.txt](../../../src/bin/studio/default_config.txt)
//!
//! ## Optional command line arguments
//! Any optional arguments overwrite settings from configuration file.
//!
//! --help - print help message
//!
//! --config_file {path to config file} - load config from file
//!
//! --rows {number} - how many rows should the screen consist of (at least 29)
//!
//! --cols {number} - how many columns should be in each line (at least 84)
//!
//! --colors_offset {number}x{number} - where should Colors window be placed (i.e 0x0)
//!
//! --backgrounds_offset {number}x{number} - where should Backgrounds window be placed
//!
//! --styles_offset {number}x{number} - where should Styles window be placed
//!
//! --glyphs_offset {number}x{number} - where should Glyphs window be placed
//!
//! --workspace_offset {number}x{number} - where should Workspace window be placed
//!
//! --workspace_size {number}x{number} - Width and Height of Workspace's interior (i.e 20x10)
//!
//! --input_file {file_name} - Read a frame into workspace from file
//!
//! --output_file {file_name} - Write a workspace frame into file
//!
//! --wallpaler_file {file_name} - Load wallpaler graphic from file
//!
//! --glyphs {filename} - index file containing filenames with glyph definitions, each filename in separate line
//!
//! ## Known issues
//! - Loading a wallpaper graphic tends to slow down application startup
//! - Dimming style causes glyphs to be displayed inconsistently
//! ## Default key bingings
//! You can use number keys from 0 to 9 to perform an action selected number of times.
//!
//! If you want to place selected glyph in 64 consecutive positions on workspace you press: 6 4 Space.
//! ### General shortcuts
//!            Define a key macro: AltM
//!            Print Workspace to a file: AltP
//!            Print screen to a file: AltCtrlP
//!            Action counter reset: R
//!            Save and exit application: Escape
//! ### Workspace window
//!            Move left: Left
//!            Move right: Right
//!            Move up: Up
//!            Move down: Down
//!            Move line start: CtrlA
//!            Move line end: CtrlE
//!            Set color on selected glyph: C
//!            Set background on selected glyph: B
//!            Set glyph on selected glyph: G
//!            Set style on selected glyph: S
//!            Select color from workspace: ShiftC
//!            Select background from workspace: ShiftB
//!            Select glyph from workspace: ShiftG
//!            Select style from workspace: ShiftS
//!            Erase glyph: Delete
//! ### Colors window
//!            Move left: ShiftLeft
//!            Move right: ShiftRight
//!            Move far right: CtrlShiftRight
//!            Move far left: CtrlShiftLeft
//!            Move top: CtrlShiftUp
//!            Move up: ShiftUp
//!            Move down: ShiftDown
//!            Move bottom: CtrlShiftDown
//!            Set window invisible: I
//!            Sete window visible: ShiftI
//! ### Backgrounds window
//!            Move left: AltLeft
//!            Move right: AltRight
//!            Move far right: AltCtrlRight
//!            Move far left: AltCtrlLeft
//!            Move top: AltCtrlUp
//!            Move up: AltUp
//!            Move down: AltDown
//!            Move bottom: AltCtrlDown
//!            Set window invisible: AltI
//!            Set window visible: AltShiftI
//! ### Glyphs window
//!            Move left: CtrlLeft
//!            Move right: CtrlRight
//!            Move up: CtrlUp
//!            Move down: CtrlDown
//!            Select glyph: Space
//!            Prev set of glyphs: PgUp
//!            Next set of glyphs: PgDn
//!            First set of glyphs: Home
//!            Last set of glyphs: End
//! ### Style window
//!            Move up: AltShiftUp
//!            Move down: AltShiftDown
//!            Enable style: AltShiftRight
//!            Disable style: AltShiftLeft
//!

use animaterm::prelude::*;
use animaterm::utilities::progress_bar;
use std::cmp::max;
use std::default::Default;
use std::fs::rename;
use std::path::Path;
use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
mod helpers;
use helpers::{
    build_basic_colors_graphic, build_color_selector, build_glyph_matrix, build_glyph_selector,
    build_selector, build_style_graphics, build_workspace_matrix,
};
mod arguments;
use arguments::{parse_arguments, read_config_file, verify_cols_and_rows};
mod style_window;
use style_window::StyleWindow;
mod colors_window;
use colors_window::ColorsWindow;

fn main() {
    let mut args = parse_arguments();
    if let Some(conf_file) = args.config_file.clone() {
        let cl_args = args;
        args = read_config_file(&conf_file);
        if cl_args.rows.is_some() {
            args.rows = cl_args.rows;
        }
        if cl_args.cols.is_some() {
            args.cols = cl_args.cols;
        }
        if cl_args.colors_offset.is_some() {
            args.colors_offset = cl_args.colors_offset;
        }
        if cl_args.backgrounds_offset.is_some() {
            args.backgrounds_offset = cl_args.backgrounds_offset;
        }
        if cl_args.styles_offset.is_some() {
            args.styles_offset = cl_args.styles_offset;
        }
        if cl_args.styles_offset.is_some() {
            args.styles_offset = cl_args.styles_offset;
        }
        if cl_args.glyphs_offset.is_some() {
            args.glyphs_offset = cl_args.glyphs_offset;
        }
        if cl_args.workspace_offset.is_some() {
            args.workspace_offset = cl_args.workspace_offset;
        }
        if cl_args.workspace_size.is_some() {
            args.workspace_size = cl_args.workspace_size;
        }
        if cl_args.input_file.is_some() {
            args.input_file = cl_args.input_file;
        }
        if cl_args.output_file.is_some() {
            args.output_file = cl_args.output_file;
        }
        if cl_args.wallpaper_file.is_some() {
            args.wallpaper_file = cl_args.wallpaper_file;
        }
        if cl_args.glyphs.is_some() {
            args.glyphs = cl_args.glyphs;
        }
    }

    let cols = args.cols;
    let rows = args.rows;
    // used for providing user visual feedback on which phase of macro-recording
    // he currently is
    let mut macro_mode: u8 = 0;
    let mut macro_loop = false;
    verify_cols_and_rows(cols, rows);
    let mut glyph = Glyph::default();
    glyph.set_char(char::from_u32(9626).unwrap());
    glyph.set_background(Color::new_gray(7));
    glyph.set_bright(true);
    glyph.set_color(Color::new_gray(17));
    let macros = if !args.macros.is_empty() {
        Some(args.macros)
    } else {
        let bind_macro = args.bindings.macro_key[0].clone();
        // panic!("Macro key: {}", bind_macro);
        Some(vec![
            // Macro recording is triggered by AltM (if not overwritten by user)
            (
                bind_macro,
                // Key::AltM,
                MacroSequence::empty(),
            ),
            // One looping macro is defined, triggered by F10 function key
            (
                Key::F10,
                MacroSequence::new(
                    true,
                    vec![
                        (Key::Right, Duration::from_millis(500)),
                        (Key::Down, Duration::from_millis(500)),
                        (Key::Left, Duration::from_millis(500)),
                        (Key::Up, Duration::from_millis(500)),
                    ],
                ),
            ),
        ])
    };
    let mut mgr = Manager::new(
        true,
        cols,
        rows,
        Some(glyph),
        Some(Duration::from_millis(10)),
        macros,
    );
    let (screen_cols, screen_rows) = mgr.screen_size();
    let start_col = screen_cols.saturating_sub(84) / 2;
    let start_row = screen_rows.saturating_sub(29) / 2;
    let mut glyphs_offset = (start_col as isize, (start_row + 7) as isize);
    if let Some(user_offset) = args.glyphs_offset {
        glyphs_offset = user_offset;
    }
    let mut wallpaper_id = None;
    if let Some(wallpaper_file) = args.wallpaper_file {
        let opt_wallpaper_graphic = Graphic::from_file(wallpaper_file);
        if let Some(wallpaper_graphic) = opt_wallpaper_graphic {
            let result = mgr.add_graphic(wallpaper_graphic, 0, (0, 0));
            if result.is_none() {
                eprintln!("Unable to create wallpaper graphic.");
            } else {
                wallpaper_id = result;
            }
        } else {
            eprintln!("Unable to load wallpaper graphic file.")
        }
    }
    if let Some(w_id) = wallpaper_id {
        mgr.set_graphic(w_id, 0, true);
    } else {
        eprintln!("Wallpaper id is None");
    }
    let selector_id;
    let selector_layer = 2;
    let result = mgr.add_graphic(build_selector(), selector_layer, glyphs_offset);
    if let Some(id) = result {
        selector_id = id;
    } else {
        eprintln!("Did not receive glyph selector graphic id");
        exit(2);
    }
    let _other_selector = build_glyph_selector();
    let glyph_matrix = build_glyph_matrix(args.glyphs);
    let max_glyph_frame_id = glyph_matrix.current_frame;
    let mut glyph_frame_id = 0;
    let glyph_matrix_id;
    let result = mgr.add_graphic(glyph_matrix, 1, glyphs_offset);
    if let Some(id) = result {
        glyph_matrix_id = id;
    } else {
        eprintln!("Did not receive glyph matrix graphic id");
        exit(2);
    }

    // Color window
    let mut color_offset_cols = start_col as isize;
    let mut color_offset_rows = start_row as isize;
    if args.colors_offset.is_some() {
        color_offset_cols = args.colors_offset.unwrap().0;
        color_offset_rows = args.colors_offset.unwrap().1;
    }
    let color_selector_id;
    let result = mgr.add_graphic(
        build_color_selector(Some("Color")),
        1,
        (color_offset_cols, color_offset_rows),
    );
    if let Some(id) = result {
        color_selector_id = id;
    } else {
        eprintln!("Did not receive color selector graphic id");
        exit(2);
    }

    glyph.set_color(Color::black());
    glyph.set_background(Color::white());
    let basic_sel_id;
    let result = mgr.add_graphic(
        build_basic_colors_graphic(glyph, Glyph::default()),
        2,
        (color_offset_cols + 3, color_offset_rows + 3),
    );
    if let Some(id) = result {
        basic_sel_id = id;
    } else {
        eprintln!("Did not receive basic colors graphic id");
        exit(2);
    }

    glyph = Glyph::default();
    let vc_id;
    let result = mgr.add_graphic(
        Graphic::from_texts(
            1,
            vec![
                ("\u{25C6}    ", glyph),
                ("  \u{25C6}  ", glyph),
                ("   \u{25C6} ", glyph),
                ("    \u{25C6}", glyph),
            ],
        ),
        2,
        (color_offset_cols + 1, color_offset_rows + 1),
    );
    if let Some(id) = result {
        vc_id = id;
    } else {
        eprintln!("Did not receive vertical cursor graphic id");
        exit(2);
    }

    mgr.set_graphic(vc_id, 0, true);
    let glyph2 = glyph.clone();
    glyph.set_color(Color::red());
    let pb1t_id;
    let result = mgr.add_graphic(
        Graphic::from_texts(6, vec![("Red   ", glyph), ("Bright", glyph2)]),
        2,
        (color_offset_cols + 3, color_offset_rows + 3),
    );
    if let Some(id) = result {
        pb1t_id = id;
    } else {
        eprintln!("Did not receive first progress bar title graphic id");
        exit(2);
    }

    mgr.set_graphic(pb1t_id, 0, true);
    glyph.set_color(Color::green());
    let pb2t_id;
    let result = mgr.add_graphic(
        Graphic::from_text(6, "Green ", glyph),
        2,
        (color_offset_cols + 3, color_offset_rows + 4),
    );
    if let Some(id) = result {
        pb2t_id = id;
    } else {
        eprintln!("Did not receive second progress bar title graphic id");
        exit(2);
    }

    mgr.set_graphic(pb2t_id, 0, true);
    glyph.set_color(Color::blue());
    let pb3t_id;
    let result = mgr.add_graphic(
        Graphic::from_text(6, "Blue  ", glyph),
        2,
        (color_offset_cols + 3, color_offset_rows + 5),
    );
    if let Some(id) = result {
        pb3t_id = id;
    } else {
        eprintln!("Did not receive third progress bar title graphic id");
        exit(2);
    }
    mgr.set_graphic(pb3t_id, 0, false);
    glyph.set_color(Color::white());
    let pb1_id;
    let result = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            Some(vec![
                Glyph::default_with_char('\u{258F}'),
                Glyph::default_with_char('\u{258E}'),
                Glyph::default_with_char('\u{258D}'),
                Glyph::default_with_char('\u{258C}'),
                Glyph::default_with_char('\u{258B}'),
                Glyph::default_with_char('\u{258A}'),
                Glyph::default_with_char('\u{2589}'),
            ]),
        ),
        2,
        (color_offset_cols + 9, color_offset_rows + 3),
    );
    if let Some(id) = result {
        pb1_id = id;
    } else {
        eprintln!("Did not receive first progress bar graphic id");
        exit(2);
    }

    let pb2_id;
    let result = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            Some(vec![
                Glyph::default_with_char('\u{258F}'),
                Glyph::default_with_char('\u{258E}'),
                Glyph::default_with_char('\u{258D}'),
                Glyph::default_with_char('\u{258C}'),
                Glyph::default_with_char('\u{258B}'),
                Glyph::default_with_char('\u{258A}'),
                Glyph::default_with_char('\u{2589}'),
            ]),
        ),
        2,
        (color_offset_cols + 9, color_offset_rows + 4),
    );
    if let Some(id) = result {
        pb2_id = id;
    } else {
        eprintln!("Did not receive second progress bar graphic id");
        exit(2);
    }

    let pb3_id;
    let result = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            Some(vec![
                Glyph::default_with_char('\u{258F}'),
                Glyph::default_with_char('\u{258E}'),
                Glyph::default_with_char('\u{258D}'),
                Glyph::default_with_char('\u{258C}'),
                Glyph::default_with_char('\u{258B}'),
                Glyph::default_with_char('\u{258A}'),
                Glyph::default_with_char('\u{2589}'),
            ]),
        ),
        2,
        (color_offset_cols + 9, color_offset_rows + 5),
    );
    if let Some(id) = result {
        pb3_id = id;
    } else {
        eprintln!("Did not receive third progress bar graphic id");
        exit(2);
    }

    mgr.set_invisible(pb1t_id, true);
    mgr.set_invisible(pb1_id, true);
    mgr.set_invisible(pb2t_id, true);
    mgr.set_invisible(pb2_id, true);
    mgr.set_invisible(pb3t_id, true);
    mgr.set_invisible(pb3_id, true);

    let mut colors_window = ColorsWindow::new(
        mgr.get_message_sender(),
        0,
        0,
        color_selector_id,
        vc_id,
        basic_sel_id,
        pb1_id,
        pb1t_id,
        pb2_id,
        pb2t_id,
        pb3_id,
        pb3t_id,
        glyph_matrix_id,
    );
    mgr.set_graphic(pb1_id, 0, false);
    mgr.set_graphic(pb2_id, 0, false);
    mgr.set_graphic(pb3_id, 0, false);
    //mgr.set_graphic(selector_id, 0, true);
    mgr.start_animation(selector_id, 0);
    mgr.set_graphic(basic_sel_id, 0, true);
    mgr.set_graphic(glyph_matrix_id, 0, true);
    mgr.set_graphic(color_selector_id, 0, true);

    // Background window
    let mut bg_offset_cols = (start_col + 42) as isize;
    let mut bg_offset_rows = (start_row) as isize;
    if args.backgrounds_offset.is_some() {
        bg_offset_cols = args.backgrounds_offset.unwrap().0;
        bg_offset_rows = args.backgrounds_offset.unwrap().1;
    }
    let bg_sel_id;
    let result = mgr.add_graphic(
        build_color_selector(Some("Background")),
        1,
        (bg_offset_cols, bg_offset_rows),
    );
    if let Some(id) = result {
        bg_sel_id = id;
    } else {
        eprintln!("Did not receive background selector graphic id");
        exit(2);
    }

    mgr.set_graphic(bg_sel_id, 0, true);

    glyph = Glyph::default();
    let bg_vc_id;
    let result = mgr.add_graphic(
        Graphic::from_texts(
            1,
            vec![
                ("\u{25C6}    ", glyph),
                ("  \u{25C6}  ", glyph),
                ("   \u{25C6} ", glyph),
                ("    \u{25C6}", glyph),
            ],
        ),
        2,
        (bg_offset_cols + 1, bg_offset_rows + 1),
    );
    if let Some(id) = result {
        bg_vc_id = id;
    } else {
        eprintln!("Did not receive background's vertical cursor graphic id");
        exit(2);
    }

    mgr.set_graphic(bg_vc_id, 0, true);
    let glyph2 = glyph.clone();
    glyph.set_color(Color::red());
    let bg_pb1t_id;
    let result = mgr.add_graphic(
        Graphic::from_texts(6, vec![("Red   ", glyph), ("Bright", glyph2)]),
        2,
        (bg_offset_cols + 3, bg_offset_rows + 3),
    );
    if let Some(id) = result {
        bg_pb1t_id = id;
    } else {
        eprintln!("Did not receive first background progress bar title graphic id");
        exit(2);
    }

    mgr.set_graphic(bg_pb1t_id, 0, false);
    glyph.set_color(Color::green());
    let bg_pb2t_id;
    let result = mgr.add_graphic(
        Graphic::from_text(6, "Green ", glyph),
        2,
        (bg_offset_cols + 3, bg_offset_rows + 4),
    );
    if let Some(id) = result {
        bg_pb2t_id = id;
    } else {
        eprintln!("Did not receive second background's progress bar title graphic id");
        exit(2);
    }
    mgr.set_graphic(bg_pb2t_id, 0, false);
    glyph.set_color(Color::blue());

    let bg_pb3t_id;
    let result = mgr.add_graphic(
        Graphic::from_text(6, "Blue  ", glyph),
        2,
        (bg_offset_cols + 3, bg_offset_rows + 5),
    );
    if let Some(id) = result {
        bg_pb3t_id = id;
    } else {
        eprintln!("Did not receive third background's progress bar title graphic id");
        exit(2);
    }
    mgr.set_graphic(bg_pb3t_id, 0, false);

    glyph = Glyph::default();
    glyph.set_color(Color::white());
    let bg_pb1_id;
    let result = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            Some(vec![
                Glyph::default_with_char('\u{258F}'),
                Glyph::default_with_char('\u{258E}'),
                Glyph::default_with_char('\u{258D}'),
                Glyph::default_with_char('\u{258C}'),
                Glyph::default_with_char('\u{258B}'),
                Glyph::default_with_char('\u{258A}'),
                Glyph::default_with_char('\u{2589}'),
            ]),
        ),
        2,
        (bg_offset_cols + 9, bg_offset_rows + 3),
    );
    if let Some(id) = result {
        bg_pb1_id = id;
    } else {
        eprintln!("Did not receive first background's progress bar graphic id");
        exit(2);
    }
    let bg_pb2_id;
    let result = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            Some(vec![
                Glyph::default_with_char('\u{258F}'),
                Glyph::default_with_char('\u{258E}'),
                Glyph::default_with_char('\u{258D}'),
                Glyph::default_with_char('\u{258C}'),
                Glyph::default_with_char('\u{258B}'),
                Glyph::default_with_char('\u{258A}'),
                Glyph::default_with_char('\u{2589}'),
            ]),
        ),
        2,
        (bg_offset_cols + 9, bg_offset_rows + 4),
    );
    if let Some(id) = result {
        bg_pb2_id = id;
    } else {
        eprintln!("Did not receive second background's progress bar graphic id");
        exit(2);
    }
    let bg_pb3_id;
    let result = mgr.add_graphic(
        progress_bar(
            32,
            Glyph::default(),
            Glyph::default_with_char('\u{2588}'),
            Some(vec![
                Glyph::default_with_char('\u{258F}'),
                Glyph::default_with_char('\u{258E}'),
                Glyph::default_with_char('\u{258D}'),
                Glyph::default_with_char('\u{258C}'),
                Glyph::default_with_char('\u{258B}'),
                Glyph::default_with_char('\u{258A}'),
                Glyph::default_with_char('\u{2589}'),
            ]),
        ),
        2,
        (bg_offset_cols + 9, bg_offset_rows + 5),
    );
    if let Some(id) = result {
        bg_pb3_id = id;
    } else {
        eprintln!("Did not receive third background's progress bar graphic id");
        exit(2);
    }
    mgr.set_invisible(bg_pb1t_id, true);
    mgr.set_invisible(bg_pb1_id, true);
    mgr.set_invisible(bg_pb2t_id, true);
    mgr.set_invisible(bg_pb2_id, true);
    mgr.set_invisible(bg_pb3t_id, true);
    mgr.set_invisible(bg_pb3_id, true);
    glyph.set_color(Color::black());
    glyph.set_background(Color::white());
    let bg_basic_sel_id;
    let result = mgr.add_graphic(
        build_basic_colors_graphic(glyph, Glyph::default()),
        2,
        (bg_offset_cols + 3, bg_offset_rows + 3),
    );
    if let Some(id) = result {
        bg_basic_sel_id = id;
    } else {
        eprintln!("Did not receive background's selector graphic id");
        exit(2);
    }
    mgr.set_graphic(bg_basic_sel_id, 0, true);

    let mut backgrounds_window = ColorsWindow::new(
        mgr.get_message_sender(),
        0,
        0,
        bg_sel_id,
        bg_vc_id,
        bg_basic_sel_id,
        bg_pb1_id,
        bg_pb1t_id,
        bg_pb2_id,
        bg_pb2t_id,
        bg_pb3_id,
        bg_pb3t_id,
        glyph_matrix_id,
    );

    // Workspace window
    let mut workspace_offset = ((start_col + 18) as isize, (start_row + 7) as isize);
    if let Some(w_off) = args.workspace_offset {
        workspace_offset = w_off;
    }
    let mut matrix_cols = 64;
    let mut matrix_rows = 22;
    if let Some((user_workspace_cols, user_workspace_rows)) = args.workspace_size {
        matrix_cols = user_workspace_cols;
        matrix_rows = user_workspace_rows;
    }
    let mut initial_workspace_graphic = None;
    if let Some(i_file) = args.input_file {
        if let Some((cs, frame)) = frame_from_file(&i_file) {
            initial_workspace_graphic = Some(Graphic::from_frame(cs, frame));
            if let Some(ref loaded) = initial_workspace_graphic {
                matrix_cols = loaded.cols;
                matrix_rows = loaded.rows;
            }
        }
    }
    let workspace_id;
    let result = mgr.add_graphic(
        build_workspace_matrix(matrix_cols, matrix_rows, initial_workspace_graphic),
        1,
        workspace_offset,
    );
    if let Some(id) = result {
        workspace_id = id;
    } else {
        eprintln!("Did not receive workspace bar graphic id");
        exit(2);
    }

    mgr.set_graphic(workspace_id, 0, true);

    let mut reversed = Glyph::default();
    reversed.set_reverse(true);
    let mut styles_offset_cols = start_col as isize;
    let mut styles_offset_rows = (start_row + 19) as isize;
    if let Some((c_off, r_off)) = args.styles_offset {
        styles_offset_cols = c_off;
        styles_offset_rows = r_off;
    }
    let mut style_graphics = build_style_graphics(reversed, Glyph::default());
    let style_window_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols, styles_offset_rows),
    );
    if let Some(id) = result {
        style_window_id = id;
    } else {
        eprintln!("Did not receive style window graphic id");
        exit(2);
    }
    let style_selector_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 2, styles_offset_rows + 1),
    );
    if let Some(id) = result {
        style_selector_id = id;
    } else {
        eprintln!("Did not receive style selector graphic id");
        exit(2);
    }
    let style_plain_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 1),
    );
    if let Some(id) = result {
        style_plain_id = id;
    } else {
        eprintln!("Did not receive style plain graphic id");
        exit(2);
    }
    let style_bright_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 2),
    );
    if let Some(id) = result {
        style_bright_id = id;
    } else {
        eprintln!("Did not receive style bright graphic id");
        exit(2);
    }
    let style_dim_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 3),
    );
    if let Some(id) = result {
        style_dim_id = id;
    } else {
        eprintln!("Did not receive style dim graphic id");
        exit(2);
    }
    let style_italic_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 4),
    );
    if let Some(id) = result {
        style_italic_id = id;
    } else {
        eprintln!("Did not receive style italic graphic id");
        exit(2);
    }

    let style_underline_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 5),
    );
    if let Some(id) = result {
        style_underline_id = id;
    } else {
        eprintln!("Did not receive style underline graphic id");
        exit(2);
    }
    let style_blink_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 6),
    );
    if let Some(id) = result {
        style_blink_id = id;
    } else {
        eprintln!("Did not receive style blink graphic id");
        exit(2);
    }
    let style_blinkfast_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 7),
    );
    if let Some(id) = result {
        style_blinkfast_id = id;
    } else {
        eprintln!("Did not receive style blink fast graphic id");
        exit(2);
    }
    let style_reverse_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 8),
    );
    if let Some(id) = result {
        style_reverse_id = id;
    } else {
        eprintln!("Did not receive style reverse graphic id");
        exit(2);
    }
    let style_transparent_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 9),
    );
    if let Some(id) = result {
        style_transparent_id = id;
    } else {
        eprintln!("Did not receive style transparent graphic id");
        exit(2);
    }

    let style_strike_id;
    let result = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 10),
    );
    if let Some(id) = result {
        style_strike_id = id;
    } else {
        eprintln!("Did not receive style strike graphic id");
        exit(2);
    }

    mgr.set_graphic(style_window_id, 0, true);
    mgr.set_graphic(style_selector_id, 0, true);
    mgr.set_graphic(style_plain_id, 0, true);
    mgr.set_graphic(style_bright_id, 0, true);
    mgr.set_graphic(style_dim_id, 0, true);
    mgr.set_graphic(style_italic_id, 0, true);
    mgr.set_graphic(style_underline_id, 0, true);
    mgr.set_graphic(style_blink_id, 0, true);
    mgr.set_graphic(style_blinkfast_id, 0, true);
    mgr.set_graphic(style_reverse_id, 0, true);
    mgr.set_graphic(style_transparent_id, 0, true);
    mgr.set_graphic(style_strike_id, 0, true);

    let mut style_window = StyleWindow::new(
        mgr.get_message_sender(),
        glyph_matrix_id,
        style_window_id,
        style_selector_id,
        style_plain_id,
        style_bright_id,
        style_dim_id,
        style_italic_id,
        style_underline_id,
        style_blink_id,
        style_blinkfast_id,
        style_reverse_id,
        style_transparent_id,
        style_strike_id,
    );

    let mut c = 1; // worskpace column where cursor is placed
    let mut r = 1; // worskpace row where cursor is placed
    let mut mc: usize = 0; // glyph matrix column where selector is placed
    let mut mr: usize = 0; // glyph matrix row where selector is placed

    let mut glyph_under_cursor = Glyph::default();
    mgr.get_glyph(workspace_id, c, r);
    let result = mgr.read_result();
    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
        glyph_under_cursor = glyph;
    }

    let g = Glyph::new(
        '\u{2588}',
        Color::blue(),
        Color::white(),
        false,
        false,
        false,
        false,
        false,
        true,
        false,
        false,
        false,
        false,
    );
    mgr.set_glyph(workspace_id, g, c, r);
    let mut action_counter = 1;
    let mut counter_initialized = false;
    loop {
        let key_pressed = mgr.read_key();
        if macro_mode == 2 && !args.bindings.macro_key.contains(&key_pressed) {
            if macro_loop {
                mgr.set_graphic(selector_id, 28, true);
            } else {
                mgr.set_graphic(selector_id, 26, true);
            }
            macro_mode = 3;
        }
        match key_pressed {
            // Colors window
            k if args.bindings.colors_left.contains(&k) => {
                colors_window.move_left(false);
            }
            k if args.bindings.colors_right.contains(&k) => {
                colors_window.move_right(false);
            }
            k if args.bindings.colors_far_right.contains(&k) => {
                colors_window.move_far_right(false);
            }
            k if args.bindings.colors_far_left.contains(&k) => {
                colors_window.move_far_left(false);
            }
            k if args.bindings.colors_up.contains(&k) => {
                colors_window.move_up();
            }
            k if args.bindings.colors_top.contains(&k) => {
                colors_window.move_top();
            }
            k if args.bindings.colors_down.contains(&k) => {
                colors_window.move_down();
            }
            k if args.bindings.colors_bottom.contains(&k) => {
                colors_window.move_bottom();
            }
            k if args.bindings.colors_invisible.contains(&k) => {
                colors_window.set_invisible(true);
            }
            k if args.bindings.colors_visible.contains(&k) => {
                colors_window.set_invisible(false);
            }

            // Background window
            k if args.bindings.backgrounds_left.contains(&k) => {
                backgrounds_window.move_left(true);
            }
            k if args.bindings.backgrounds_right.contains(&k) => {
                backgrounds_window.move_right(true);
            }
            k if args.bindings.backgrounds_far_left.contains(&k) => {
                backgrounds_window.move_far_left(true);
            }
            k if args.bindings.backgrounds_far_right.contains(&k) => {
                backgrounds_window.move_far_right(true);
            }
            k if args.bindings.backgrounds_up.contains(&k) => {
                backgrounds_window.move_up();
            }
            k if args.bindings.backgrounds_top.contains(&k) => {
                backgrounds_window.move_top();
            }
            k if args.bindings.backgrounds_down.contains(&k) => {
                backgrounds_window.move_down();
            }
            k if args.bindings.backgrounds_bottom.contains(&k) => {
                backgrounds_window.move_bottom();
            }
            k if args.bindings.backgrounds_invisible.contains(&k) => {
                backgrounds_window.set_invisible(true);
            }
            k if args.bindings.backgrounds_visible.contains(&k) => {
                backgrounds_window.set_invisible(false);
            }

            //Glyphs window
            k if args.bindings.glyphs_left.contains(&k) => {
                if mc > 0 {
                    mc -= 1;
                    mgr.move_graphic(selector_id, 2, (-1, 0))
                } else {
                    mgr.move_graphic(selector_id, 2, (15, 0));
                    mc = 15;
                }
            }

            // glyphs
            k if args.bindings.glyphs_right.contains(&k) => {
                if mc < 15 {
                    mc += 1;
                    mgr.move_graphic(selector_id, 2, (1, 0));
                } else {
                    mgr.move_graphic(selector_id, 2, (-15, 0));
                    let start_x = max(0, glyphs_offset.0) as usize + mc;
                    let start_y = max(0, glyphs_offset.1) as usize + mr;
                    mgr.clear_area(selector_layer, (start_x, start_y), (2, 3));
                    mc = 0;
                }
            }

            // glyphs
            k if args.bindings.glyphs_up.contains(&k) => {
                if mr > 0 {
                    mr -= 1;
                    mgr.move_graphic(selector_id, 2, (0, -1))
                } else {
                    mgr.move_graphic(selector_id, 2, (0, 9));
                    mr = 9;
                }
            }

            //glyphs
            k if args.bindings.glyphs_down.contains(&k) => {
                if mr < 9 {
                    mr += 1;
                    mgr.move_graphic(selector_id, 2, (0, 1))
                } else {
                    mgr.move_graphic(selector_id, 2, (0, mr as isize * (-1)));
                    let start_x = max(0, glyphs_offset.0) as usize + mc;
                    let start_y = max(0, glyphs_offset.1) as usize + mr;
                    mgr.clear_area(selector_layer, (start_x as usize, start_y as usize), (2, 3));
                    mr = 0;
                }
            }

            //glyphs
            k if args.bindings.glyphs_select.contains(&k) => {
                mgr.start_animation(selector_id, 1);
                mgr.enqueue_animation(selector_id, 0, Timestamp::now());

                mgr.get_glyph(glyph_matrix_id, mc + 1, mr + 1);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    while action_counter > 0 {
                        mgr.set_glyph(workspace_id, glyph, c, r);
                        if c < matrix_cols {
                            c += 1;
                        } else if r < matrix_rows {
                            c = 1;
                            r += 1;
                        } else {
                            c = 1;
                            r = 1;
                        }
                        action_counter -= 1;
                    }
                    action_counter = 1;
                    counter_initialized = false;
                    mgr.get_glyph(workspace_id, c, r);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                        glyph_under_cursor = glyph;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                }
            }

            //glyphs
            k if args.bindings.glyphs_prev.contains(&k) => {
                if glyph_frame_id == 0 {
                    glyph_frame_id = max_glyph_frame_id;
                } else {
                    glyph_frame_id -= 1;
                }
                mgr.set_graphic(glyph_matrix_id, glyph_frame_id, true)
            }
            //glyphs
            k if args.bindings.glyphs_home.contains(&k) => {
                glyph_frame_id = 0;
                mgr.set_graphic(glyph_matrix_id, glyph_frame_id, true);
            }
            //glyphs
            k if args.bindings.glyphs_next.contains(&k) => {
                if glyph_frame_id == max_glyph_frame_id {
                    glyph_frame_id = 0;
                } else {
                    glyph_frame_id += 1;
                }
                mgr.set_graphic(glyph_matrix_id, glyph_frame_id, true)
            }
            //glyphs
            k if args.bindings.glyphs_end.contains(&k) => {
                glyph_frame_id = max_glyph_frame_id;
                mgr.set_graphic(glyph_matrix_id, glyph_frame_id, true);
            }

            // workspace window
            k if args.bindings.workspace_left.contains(&k) => {
                mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                if c > 1 {
                    c -= 1;
                } else {
                    c = matrix_cols;
                    if r > 1 {
                        r -= 1;
                    } else {
                        r = matrix_rows;
                    }
                }
                mgr.get_glyph(workspace_id, c, r);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    glyph_under_cursor = glyph;
                }
                mgr.set_glyph(workspace_id, g, c, r);
            }
            k if args.bindings.workspace_line_start.contains(&k) => {
                mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                c = 1;
                mgr.get_glyph(workspace_id, c, r);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    glyph_under_cursor = glyph;
                }
                mgr.set_glyph(workspace_id, g, c, r);
            }

            k if args.bindings.workspace_line_end.contains(&k) => {
                mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                c = matrix_cols;
                mgr.get_glyph(workspace_id, c, r);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    glyph_under_cursor = glyph;
                }
                mgr.set_glyph(workspace_id, g, c, r);
            }

            // workspace window
            k if args.bindings.workspace_right.contains(&k) => {
                mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                if c < matrix_cols {
                    c += 1;
                } else {
                    c = 1;
                    if r < matrix_rows {
                        r += 1;
                    } else {
                        r = 1;
                    }
                }
                mgr.get_glyph(workspace_id, c, r);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    glyph_under_cursor = glyph;
                }
                mgr.set_glyph(workspace_id, g, c, r);
            }

            //workspace window
            k if args.bindings.workspace_set_color.contains(&k) => {
                // println!("set color!");
                mgr.get_glyph(glyph_matrix_id, mc + 1, mr + 1);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    while action_counter > 0 {
                        glyph_under_cursor.set_color(glyph.color);
                        mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                        if c < matrix_cols {
                            c += 1;
                        } else if r < matrix_rows {
                            c = 1;
                            r += 1;
                        } else {
                            c = 1;
                            r = 1;
                        }
                        mgr.get_glyph(workspace_id, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        action_counter -= 1;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                    action_counter = 1;
                    counter_initialized = false;
                }
            }
            k if args.bindings.workspace_set_background.contains(&k) => {
                mgr.get_glyph(glyph_matrix_id, mc + 1, mr + 1);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    while action_counter > 0 {
                        glyph_under_cursor.set_background(glyph.background);
                        mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                        if c < matrix_cols {
                            c += 1;
                        } else if r < matrix_rows {
                            c = 1;
                            r += 1;
                        } else {
                            c = 1;
                            r = 1;
                        }
                        mgr.get_glyph(workspace_id, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        action_counter -= 1;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                    action_counter = 1;
                    counter_initialized = false;
                }
            }
            k if args.bindings.workspace_set_glyph.contains(&k) => {
                mgr.get_glyph(glyph_matrix_id, mc + 1, mr + 1);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    while action_counter > 0 {
                        glyph_under_cursor.set_char(glyph.character);
                        mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                        if c < matrix_cols {
                            c += 1;
                        } else if r < matrix_rows {
                            c = 1;
                            r += 1;
                        } else {
                            c = 1;
                            r = 1;
                        }
                        mgr.get_glyph(workspace_id, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        action_counter -= 1;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                    action_counter = 1;
                    counter_initialized = false;
                }
            }
            k if args.bindings.workspace_set_style.contains(&k) => {
                let mut new_glyph = style_window.style_glyph.clone();
                new_glyph.set_char(glyph_under_cursor.character);
                new_glyph.set_color(glyph_under_cursor.color);
                new_glyph.set_background(glyph_under_cursor.background);
                while action_counter > 0 {
                    mgr.set_glyph(workspace_id, new_glyph, c, r);
                    if c < matrix_cols {
                        c += 1;
                    } else if r < matrix_rows {
                        c = 1;
                        r += 1;
                    } else {
                        c = 1;
                        r = 1;
                    }
                    mgr.get_glyph(workspace_id, c, r);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                        glyph_under_cursor = glyph;
                    }
                    action_counter -= 1;
                }
                mgr.set_glyph(workspace_id, g, c, r);
                action_counter = 1;
                counter_initialized = false;
            }
            k if args.bindings.workspace_select_color.contains(&k) => {
                colors_window.select_color(glyph_under_cursor.color, false);
            }
            k if args.bindings.workspace_select_color.contains(&k) => {
                colors_window.select_color(glyph_under_cursor.color, false);
            }
            k if args.bindings.workspace_select_background.contains(&k) => {
                backgrounds_window.select_color(glyph_under_cursor.background, true);
            }
            k if args.bindings.workspace_select_glyph.contains(&k) => {
                'break_point: for c in 1..17 {
                    for r in 0..10 {
                        mgr.get_glyph(glyph_matrix_id, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            if glyph.character == glyph_under_cursor.character {
                                let dc: isize = c as isize - mc as isize - 1;
                                let dr: isize = r as isize - mr as isize - 1;
                                mc = c - 1;
                                mr = r - 1;
                                mgr.move_graphic(selector_id, 2, (dc, dr));
                                break 'break_point;
                            };
                        }
                    }
                }
            }
            k if args.bindings.workspace_select_style.contains(&k) => {
                if glyph_under_cursor.bright {
                    mgr.set_graphic(style_bright_id, 1, false);
                    style_window.style_glyph.set_bright(true);
                } else {
                    mgr.set_graphic(style_bright_id, 0, false);
                    style_window.style_glyph.set_bright(false);
                }
                if glyph_under_cursor.dim {
                    mgr.set_graphic(style_dim_id, 1, false);
                    style_window.style_glyph.set_dim(true);
                } else {
                    mgr.set_graphic(style_dim_id, 0, false);
                    style_window.style_glyph.set_dim(false);
                }
                if glyph_under_cursor.italic {
                    mgr.set_graphic(style_italic_id, 1, false);
                    style_window.style_glyph.set_italic(true);
                } else {
                    mgr.set_graphic(style_italic_id, 0, false);
                    style_window.style_glyph.set_italic(false);
                }
                if glyph_under_cursor.underline {
                    mgr.set_graphic(style_underline_id, 1, false);
                    style_window.style_glyph.set_underline(true);
                } else {
                    mgr.set_graphic(style_underline_id, 0, false);
                    style_window.style_glyph.set_underline(false);
                }
                if glyph_under_cursor.blink {
                    mgr.set_graphic(style_blink_id, 1, false);
                    style_window.style_glyph.set_blink(true);
                } else {
                    mgr.set_graphic(style_blink_id, 0, false);
                    style_window.style_glyph.set_blink(false);
                }
                if glyph_under_cursor.blink_fast {
                    mgr.set_graphic(style_blinkfast_id, 1, false);
                    style_window.style_glyph.set_blinkfast(true);
                } else {
                    mgr.set_graphic(style_blinkfast_id, 0, false);
                    style_window.style_glyph.set_blinkfast(false);
                }
                if glyph_under_cursor.reverse {
                    mgr.set_graphic(style_reverse_id, 1, false);
                    style_window.style_glyph.set_reverse(true);
                } else {
                    mgr.set_graphic(style_reverse_id, 0, false);
                    style_window.style_glyph.set_reverse(false);
                }
                if glyph_under_cursor.transparent {
                    mgr.set_graphic(style_transparent_id, 1, false);
                    style_window.style_glyph.set_transparent(true);
                } else {
                    mgr.set_graphic(style_transparent_id, 0, false);
                    style_window.style_glyph.set_transparent(false);
                }
                if glyph_under_cursor.strike {
                    mgr.set_graphic(style_strike_id, 1, false);
                    style_window.style_glyph.set_strike(true);
                } else {
                    mgr.set_graphic(style_strike_id, 0, false);
                    style_window.style_glyph.set_strike(false);
                }
                style_window.activate_style_on_glyph_matrix();
            }

            //workspace window
            k if args.bindings.workspace_up.contains(&k) => {
                // workspace_window.move_cursor_up();
                mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                if r > 1 {
                    r -= 1;
                } else {
                    r = matrix_rows;
                }
                mgr.get_glyph(workspace_id, c, r);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    glyph_under_cursor = glyph;
                }
                mgr.set_glyph(workspace_id, g, c, r);
            }

            // workspace window
            k if args.bindings.workspace_down.contains(&k) => {
                // workspace_window.move_cursor_down(glyph_under_cursor);
                mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                if r < matrix_rows {
                    r += 1;
                } else {
                    r = 1;
                }
                mgr.get_glyph(workspace_id, c, r);
                let result = mgr.read_result();
                if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                    glyph_under_cursor = glyph;
                }
                mgr.set_glyph(workspace_id, g, c, r);
            }

            // workspace window
            k if args.bindings.workspace_erase.contains(&k) => {
                while action_counter > 0 {
                    mgr.set_glyph(workspace_id, Glyph::default(), c, r);
                    if c > 1 {
                        c -= 1
                    } else {
                        if r > 1 {
                            r -= 1;
                        } else {
                            r = matrix_rows;
                        }
                        c = matrix_cols;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                    action_counter -= 1;
                }
                action_counter = 1;
                counter_initialized = false;
            }
            // Key::AltCtrlShiftUp => {
            //     // println!("move up!");
            // }
            // Key::AltCtrlShiftDown => {}
            // Key::AltCtrlShiftLeft => {}
            // Key::AltCtrlShiftRight => {}

            // style window
            k if args.bindings.style_up.contains(&k) => {
                style_window.move_selector_up();
            }
            k if args.bindings.style_down.contains(&k) => {
                style_window.move_selector_down();
            }
            k if args.bindings.style_enable.contains(&k) => {
                style_window.enable_selected_style();
            }
            k if args.bindings.style_disable.contains(&k) => {
                style_window.disable_selected_style();
            }
            // Key::AltCtrlShiftDown => {
            //    style_window.move_selector_bottom();
            // }
            // Key::AltCtrlShiftUp => {
            //     style_window.move_selector_top();
            // }
            k if args.bindings.print_graphic.contains(&k) => {
                mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                // mgr.print_screen_section(
                //     (workspace_offset.0 + 1, workspace_offset.1 + 1),
                //     matrix_cols,
                //     matrix_rows,
                // );
                mgr.print_graphic(workspace_id, true);
                mgr.set_glyph(workspace_id, g, c, r);
                let result = mgr.read_result();
                if let Ok(AnimOk::PrintScreen(print_screen_text)) = result {
                    use std::fs::OpenOptions;
                    use std::io::Write;
                    let secs = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let filename = format!("print_graphic_{}.txf", secs);

                    let mut f = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .append(false)
                        .open(filename)
                        .expect("Unable to create file");

                    for line in print_screen_text.iter() {
                        f.write_all(line.as_bytes()).expect("Unable to write data");
                        // f.write_all("\x1b[K".as_bytes())
                        //     .expect("Unable to write data");
                        // f.write_all("\x1b[1B".as_bytes())
                        //     .expect("Unable to write data");
                        let fmted = format!("\n");
                        f.write_all(fmted.as_bytes()).expect("Unable to write data");
                    }
                    //     let fmted = format!("\n\x1b[{}D", self.wiersze[0].len());
                    //     f.write_all(fmted.as_bytes()).expect("Unable to write data");
                }
            }
            k if args.bindings.print_screen.contains(&k) => {
                mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                mgr.print_screen();
                mgr.set_glyph(workspace_id, g, c, r);
                let result = mgr.read_result();
                if let Ok(AnimOk::PrintScreen(print_screen_text)) = result {
                    use std::fs::OpenOptions;
                    use std::io::Write;
                    let secs = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let filename = format!("print_screen_{}.txf", secs);

                    let mut f = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .append(false)
                        .open(filename)
                        .expect("Unable to create file");

                    for line in print_screen_text.iter() {
                        f.write_all(line.as_bytes()).expect("Unable to write data");
                        // f.write_all("\x1b[K".as_bytes())
                        //     .expect("Unable to write data");
                        // f.write_all("\x1b[1B".as_bytes())
                        //     .expect("Unable to write data");
                        let fmted = format!("\n");
                        f.write_all(fmted.as_bytes()).expect("Unable to write data");
                    }
                    //     let fmted = format!("\n\x1b[{}D", self.wiersze[0].len());
                    //     f.write_all(fmted.as_bytes()).expect("Unable to write data");
                }
            }
            // reset action counter
            k if args.bindings.action_counter_reset.contains(&k) => {
                action_counter = 1;
                counter_initialized = false;
            }
            // macro recording presentation logic
            k if args.bindings.macro_key.contains(&k) => {
                match macro_mode {
                    0 => {
                        macro_mode = 1;
                        mgr.stop_animation(selector_id);
                        mgr.set_graphic(selector_id, 25, true);
                    }
                    1 => {}
                    2 => {
                        macro_loop = !macro_loop;
                        if macro_loop {
                            mgr.set_graphic(selector_id, 27, true);
                        } else {
                            mgr.set_graphic(selector_id, 25, true);
                        }
                    }
                    3 => {
                        macro_mode = 0;
                        macro_loop = false;
                        mgr.start_animation(selector_id, 0);
                        // looped = false;
                    }
                    _ => {
                        // println!("This should not happen");
                    }
                };
            }
            // exit program
            k if args.bindings.exit.contains(&k) => {
                if let Some(output_file) = args.output_file {
                    mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                    mgr.print_graphic(workspace_id, true);
                    mgr.set_glyph(workspace_id, g, c, r);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::PrintScreen(print_screen_text)) = result {
                        use std::fs::OpenOptions;
                        use std::io::Write;
                        let old_path = Path::new(&output_file);

                        if Path::exists(old_path) {
                            let mut old_file;
                            let mut base = output_file.clone();
                            base.push('_');
                            for i in 0..usize::MAX {
                                old_file = base.clone();
                                old_file.push_str(&format!("{}", i));
                                let new_path = Path::new(&old_file);
                                if !Path::exists(new_path) {
                                    if rename(old_path, new_path).is_err() {
                                        eprintln!(
                                            "Unable to rename existing file {}, removing it.",
                                            output_file
                                        );
                                    }
                                    break;
                                }
                            }
                        }

                        let mut f = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(output_file)
                            .expect("Unable to create file");

                        for line in print_screen_text.iter() {
                            f.write_all(line.as_bytes()).expect("Unable to write data");
                            // f.write_all("\x1b[K".as_bytes())
                            //     .expect("Unable to write data");
                            // f.write_all("\x1b[1B".as_bytes())
                            //     .expect("Unable to write data");
                            let fmted = format!("\n");
                            f.write_all(fmted.as_bytes()).expect("Unable to write data");
                        }
                        //     let fmted = format!("\n\x1b[{}D", self.wiersze[0].len());
                        //     f.write_all(fmted.as_bytes()).expect("Unable to write data");
                    }
                }
                break;
            }
            Key::Zero => {
                action_counter *= 10;
            }
            Key::One => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 1;
                }
                counter_initialized = true;
            }
            Key::Two => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 2;
                } else {
                    counter_initialized = true;
                    action_counter = 2;
                }
            }
            Key::Three => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 3;
                } else {
                    counter_initialized = true;
                    action_counter = 3;
                }
            }
            Key::Four => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 4;
                } else {
                    counter_initialized = true;
                    action_counter = 4;
                }
            }
            Key::Five => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 5;
                } else {
                    counter_initialized = true;
                    action_counter = 5;
                }
            }
            Key::Six => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 6;
                } else {
                    counter_initialized = true;
                    action_counter = 6;
                }
            }
            Key::Seven => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 7;
                } else {
                    counter_initialized = true;
                    action_counter = 7;
                }
            }
            Key::Eight => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 8;
                } else {
                    counter_initialized = true;
                    action_counter = 8;
                }
            }
            Key::Nine => {
                if counter_initialized {
                    action_counter *= 10;
                    action_counter += 9;
                } else {
                    counter_initialized = true;
                    action_counter = 9;
                }
            }

            _ => {
                // println!("You pressed {}", key);
                continue;
            }
        }
        if macro_mode == 1 {
            macro_mode = 2;
        }
        // }
    }
    mgr.terminate();
}
