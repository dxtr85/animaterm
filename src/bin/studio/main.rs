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
        if cl_args.glyphs.is_some() {
            args.glyphs = cl_args.glyphs;
        }
    }

    let cols = args.cols;
    let rows = args.rows;
    verify_cols_and_rows(cols, rows);
    let mut glyph = Glyph::default();
    glyph.set_char(char::from_u32(9626).unwrap());
    glyph.set_background(Color::new_gray(7));
    glyph.set_bright(true);
    glyph.set_color(Color::new_gray(17));
    let mut mgr = Manager::new(
        true,
        cols,
        rows,
        Some(glyph),
        Some(Duration::from_millis(10)),
    );
    let (screen_cols, screen_rows) = mgr.screen_size();
    let start_col = screen_cols.saturating_sub(84) / 2;
    let start_row = screen_rows.saturating_sub(29) / 2;
    let mut glyphs_offset = (start_col as isize, (start_row + 7) as isize);
    if let Some(user_offset) = args.glyphs_offset {
        glyphs_offset = user_offset;
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
        0,
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
        1,
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
        1,
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
        1,
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
        1,
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
        1,
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
        1,
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
        1,
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
        1,
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
        0,
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
        1,
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
        1,
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
        1,
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
        1,
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
        1,
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
        1,
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
        1,
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
        1,
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
        0,
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
    loop {
        if let Some(key) = mgr.read_key() {
            match key {
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
                        mgr.clear_area(
                            selector_layer,
                            (start_x as usize, start_y as usize),
                            (2, 3),
                        );
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
                    // workspace_window.erase_glyph();
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

                _ => {
                    println!("You pressed {}", key);
                    continue;
                }
            }
        }
    }
    mgr.terminate();
}
