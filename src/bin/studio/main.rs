use animaterm::prelude::*;
use animaterm::utilities::progress_bar;
use std::default::Default;
mod helpers;
use helpers::{
    build_basic_colors_graphic, build_color_selector, build_empty_matrix, build_glyph_matrix,
    build_selector, build_style_graphics,
};
mod arguments;
use arguments::{parse_arguments, verify_cols_and_rows};
mod style_window;
use style_window::StyleWindow;
mod colors_window;
use colors_window::ColorsWindow;

fn main() {
    let args = parse_arguments();
    let cols = args.cols;
    let rows = args.rows;
    verify_cols_and_rows(cols, rows);
    let mut mgr = Manager::new(true, cols, rows, None, None);
    // let (cols, rows) = mgr.screen_size();
    let mut keep_running = true;
    let mut glyphs_offset = (0, 7);
    if let Some(user_offset) = args.glyphs_offset {
        glyphs_offset = user_offset;
    }
    let selector_id = mgr.add_graphic(build_selector(), 1, glyphs_offset);
    let glyph_matrix = build_glyph_matrix(args.glyphs);
    let max_glyph_frame_id = glyph_matrix.current_frame;
    let mut glyph_frame_id = 0;
    let glyph_matrix_id = mgr.add_graphic(glyph_matrix, glyph_frame_id, glyphs_offset);

    // Color window
    let mut color_offset_cols = 0;
    let mut color_offset_rows = 0;
    if args.colors_offset.is_some() {
        color_offset_cols = args.colors_offset.unwrap().0;
        color_offset_rows = args.colors_offset.unwrap().1;
    }
    let color_selector_id = mgr.add_graphic(
        build_color_selector(Some("Color")),
        0,
        (color_offset_cols, color_offset_rows),
    );
    let mut glyph = Glyph::default();
    glyph.set_color(Color::black());
    glyph.set_background(Color::white());
    let basic_sel_id = mgr.add_graphic(
        build_basic_colors_graphic(glyph, Glyph::default()),
        2,
        (color_offset_cols + 3, color_offset_rows + 3),
    );
    glyph = Glyph::default();
    let vc_id = mgr.add_graphic(
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
    mgr.set_graphic(vc_id, 0, true);
    let glyph2 = glyph.clone();
    glyph.set_color(Color::red());
    let pb1t_id = mgr.add_graphic(
        Graphic::from_texts(6, vec![("Red   ", glyph), ("Bright", glyph2)]),
        1,
        (color_offset_cols + 3, color_offset_rows + 3),
    );
    mgr.set_graphic(pb1t_id, 0, true);
    glyph.set_color(Color::green());
    let pb2t_id = mgr.add_graphic(
        Graphic::from_text(6, "Green", glyph),
        1,
        (color_offset_cols + 3, color_offset_rows + 4),
    );
    mgr.set_graphic(pb2t_id, 0, true);
    glyph.set_color(Color::blue());
    let pb3t_id = mgr.add_graphic(
        Graphic::from_text(6, "Blue", glyph),
        1,
        (color_offset_cols + 3, color_offset_rows + 5),
    );
    mgr.set_graphic(pb3t_id, 0, true);
    glyph.set_color(Color::white());
    let pb1_id = mgr.add_graphic(
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
    let pb2_id = mgr.add_graphic(
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
    let pb3_id = mgr.add_graphic(
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
    mgr.set_graphic(pb1_id, 0, true);
    mgr.set_graphic(pb2_id, 0, true);
    mgr.set_graphic(pb3_id, 0, true);
    mgr.set_graphic(selector_id, 0, true);
    mgr.set_graphic(glyph_matrix_id, 0, true);
    mgr.set_graphic(color_selector_id, 0, true);

    // Background window
    let mut bg_offset_cols = 42;
    let mut bg_offset_rows = 0;
    if args.backgrounds_offset.is_some() {
        bg_offset_cols = args.backgrounds_offset.unwrap().0;
        bg_offset_rows = args.backgrounds_offset.unwrap().1;
    }
    let bg_sel_id = mgr.add_graphic(
        build_color_selector(Some("Background")),
        0,
        (bg_offset_cols, bg_offset_rows),
    );
    mgr.set_graphic(bg_sel_id, 0, true);

    glyph.set_color(Color::black());
    glyph.set_background(Color::white());
    let bg_basic_sel_id = mgr.add_graphic(
        build_basic_colors_graphic(glyph, Glyph::default()),
        2,
        (bg_offset_cols + 3, bg_offset_rows + 3),
    );
    mgr.set_graphic(bg_basic_sel_id, 0, true);
    glyph = Glyph::default();
    let bg_vc_id = mgr.add_graphic(
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
    mgr.set_graphic(bg_vc_id, 0, true);
    let glyph2 = glyph.clone();
    glyph.set_color(Color::red());
    let bg_pb1t_id = mgr.add_graphic(
        Graphic::from_texts(6, vec![("Red   ", glyph), ("Bright", glyph2)]),
        1,
        (bg_offset_cols + 3, bg_offset_rows + 3),
    );
    mgr.set_graphic(bg_pb1t_id, 0, true);
    glyph.set_color(Color::green());
    let bg_pb2t_id = mgr.add_graphic(
        Graphic::from_text(6, "Green", glyph),
        1,
        (bg_offset_cols + 3, bg_offset_rows + 4),
    );
    mgr.set_graphic(bg_pb2t_id, 0, true);
    glyph.set_color(Color::blue());
    let bg_pb3t_id = mgr.add_graphic(
        Graphic::from_text(6, "Blue", glyph),
        1,
        (bg_offset_cols + 3, bg_offset_rows + 5),
    );
    mgr.set_graphic(bg_pb3t_id, 0, true);

    glyph = Glyph::default();
    mgr.set_graphic(basic_sel_id, 0, false);
    mgr.set_invisible(basic_sel_id, false);

    glyph.set_color(Color::white());
    let bg_pb1_id = mgr.add_graphic(
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
    let bg_pb2_id = mgr.add_graphic(
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
    let bg_pb3_id = mgr.add_graphic(
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
    mgr.set_invisible(bg_pb1t_id, true);
    mgr.set_invisible(bg_pb1_id, true);
    mgr.set_invisible(bg_pb2t_id, true);
    mgr.set_invisible(bg_pb2_id, true);
    mgr.set_invisible(bg_pb3t_id, true);
    mgr.set_invisible(bg_pb3_id, true);

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
    let mut workspace_offset = (18, 7);
    if let Some(w_off) = args.workspace_offset {
        workspace_offset = w_off;
    }
    let mut matrix_cols = 64;
    let mut matrix_rows = 20;
    if let Some((user_workspace_cols, user_workspace_rows)) = args.workspace_size {
        matrix_cols = user_workspace_cols;
        matrix_rows = user_workspace_rows;
    }
    let workspace_id = mgr.add_graphic(
        build_empty_matrix(matrix_cols, matrix_rows),
        0,
        workspace_offset,
    );
    mgr.set_graphic(workspace_id, 0, true);

    let mut reversed = Glyph::default();
    reversed.set_reverse(true);
    let mut styles_offset_cols = 0;
    let mut styles_offset_rows = 19;
    if let Some((c_off, r_off)) = args.styles_offset {
        styles_offset_cols = c_off;
        styles_offset_rows = r_off;
    }
    let mut style_graphics = build_style_graphics(reversed, Glyph::default());
    let style_window_id = mgr.add_graphic(
        style_graphics.remove(0),
        0,
        (styles_offset_cols, styles_offset_rows),
    );
    let style_selector_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 2, styles_offset_rows + 1),
    );
    let style_transparent_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 1),
    );
    let style_bright_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 2),
    );
    let style_italic_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 3),
    );
    let style_underline_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 4),
    );
    let style_blink_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 5),
    );
    let style_blinkfast_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 6),
    );
    let style_reverse_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 7),
    );
    let style_strike_id = mgr.add_graphic(
        style_graphics.remove(0),
        1,
        (styles_offset_cols + 4, styles_offset_rows + 8),
    );

    mgr.set_graphic(style_window_id, 0, true);
    mgr.set_graphic(style_selector_id, 0, true);
    mgr.set_graphic(style_transparent_id, 0, true);
    mgr.set_graphic(style_bright_id, 0, true);
    mgr.set_graphic(style_italic_id, 0, true);
    mgr.set_graphic(style_underline_id, 0, true);
    mgr.set_graphic(style_blink_id, 0, true);
    mgr.set_graphic(style_blinkfast_id, 0, true);
    mgr.set_graphic(style_reverse_id, 0, true);
    mgr.set_graphic(style_strike_id, 0, true);

    let mut style_window = StyleWindow::new(
        mgr.get_message_sender(),
        glyph_matrix_id,
        style_window_id,
        style_selector_id,
        style_transparent_id,
        style_bright_id,
        style_italic_id,
        style_underline_id,
        style_blink_id,
        style_blinkfast_id,
        style_reverse_id,
        style_strike_id,
    );

    let mut c = 2; // worskpace column where cursor is placed
    let mut r = 2; // worskpace row where cursor is placed
    let mut mc = 2; // glyph matrix column where selector is placed
    let mut mr = 2; // glyph matrix row where selector is placed

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
        true,
        false,
        false,
        false,
    );
    mgr.set_glyph(workspace_id, g, c, r);
    while keep_running {
        if let Some(key) = mgr.read_key() {
            match key {
                // Colors window
                Key::Shift_Left => {
                    colors_window.move_left(false);
                }
                Key::Shift_Right => {
                    colors_window.move_right(false);
                }
                Key::Ctrl_Shift_Right => {
                    colors_window.move_far_right(false);
                }
                Key::Ctrl_Shift_Left => {
                    colors_window.move_far_left(false);
                }
                Key::Shift_Up => {
                    colors_window.move_up();
                }
                Key::Ctrl_Shift_Up => {
                    colors_window.move_top();
                }
                Key::Shift_Down => {
                    colors_window.move_down();
                }
                Key::Ctrl_Shift_Down => {
                    colors_window.move_bottom();
                }
                Key::i => {
                    colors_window.set_invisible(true);
                }
                Key::I => {
                    colors_window.set_invisible(false);
                }

                // Background window
                Key::Alt_Left => {
                    backgrounds_window.move_left(true);
                }
                Key::Alt_Right => {
                    backgrounds_window.move_right(true);
                }
                Key::Alt_Ctrl_Left => {
                    backgrounds_window.move_far_left(true);
                }
                Key::Alt_Ctrl_Right => {
                    backgrounds_window.move_far_right(true);
                }
                Key::Alt_Up => {
                    backgrounds_window.move_up();
                }
                Key::Alt_Ctrl_Up => {
                    backgrounds_window.move_top();
                }
                Key::Alt_Down => {
                    backgrounds_window.move_down();
                }
                Key::Alt_Ctrl_Down => {
                    backgrounds_window.move_bottom();
                }
                Key::Alt_i => {
                    backgrounds_window.set_invisible(true);
                }
                Key::Alt_I => {
                    backgrounds_window.set_invisible(false);
                }

                //Glyphs window
                Key::Left => {
                    if mc > 2 {
                        mc -= 1;
                        mgr.move_graphic(selector_id, 2, (-1, 0))
                    } else {
                        mgr.move_graphic(selector_id, 2, (15, 0));
                        mc = 17;
                    }
                }

                // glyphs
                Key::Right => {
                    if mc < 17 {
                        mc += 1;
                        mgr.move_graphic(selector_id, 2, (1, 0));
                    } else {
                        mgr.move_graphic(selector_id, 2, (-15, 0));
                        mc = 2;
                    }
                }

                // glyphs
                Key::Up => {
                    if mr > 2 {
                        mr -= 1;
                        mgr.move_graphic(selector_id, 2, (0, -1))
                    } else {
                        mgr.move_graphic(selector_id, 2, (0, 9));
                        mr = 11;
                    }
                }

                //glyphs
                Key::Down => {
                    if mr < 11 {
                        mr += 1;
                        mgr.move_graphic(selector_id, 2, (0, 1))
                    } else {
                        mgr.move_graphic(selector_id, 2, (0, mr as isize * (-1) + 2));
                        mr = 2;
                    }
                }

                //glyphs
                Key::Space => {
                    mgr.start_animation(selector_id, 0);
                    mgr.get_glyph(glyph_matrix_id, mc, mr);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                        mgr.set_glyph(workspace_id, glyph, c, r);
                        if c < matrix_cols + 1 {
                            c += 1;
                        } else if r < matrix_rows + 1 {
                            c = 2;
                            r += 1;
                        } else {
                            c = 2;
                            r = 2;
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
                Key::PgUp => {
                    if glyph_frame_id == 0 {
                        glyph_frame_id = max_glyph_frame_id;
                    } else {
                        glyph_frame_id -= 1;
                    }
                    mgr.set_graphic(glyph_matrix_id, glyph_frame_id, true)
                }
                //glyphs
                Key::Home => {
                    glyph_frame_id = 0;
                    mgr.set_graphic(glyph_matrix_id, glyph_frame_id, true);
                }
                //glyphs
                Key::PgDn => {
                    if glyph_frame_id == max_glyph_frame_id {
                        glyph_frame_id = 0;
                    } else {
                        glyph_frame_id += 1;
                    }
                    mgr.set_graphic(glyph_matrix_id, glyph_frame_id, true)
                }
                //glyphs
                Key::End => {
                    glyph_frame_id = max_glyph_frame_id;
                    mgr.set_graphic(glyph_matrix_id, glyph_frame_id, true);
                }

                // workspace window
                Key::Ctrl_Left => {
                    mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                    if c > 2 {
                        c -= 1;
                    } else {
                        c = matrix_cols + 1;
                        if r > 2 {
                            r -= 1;
                        } else {
                            r = matrix_rows + 1;
                        }
                    }
                    mgr.get_glyph(workspace_id, c, r);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                        glyph_under_cursor = glyph;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                }

                // workspace window
                Key::Ctrl_Right => {
                    mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                    if c < matrix_cols + 1 {
                        c += 1;
                    } else {
                        c = 2;
                        if r < matrix_rows + 1 {
                            r += 1;
                        } else {
                            r = 2;
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
                Key::Ctrl_Up => {
                    // workspace_window.move_cursor_up();
                    mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                    if r > 2 {
                        r -= 1;
                    } else {
                        r = matrix_rows + 1;
                    }
                    mgr.get_glyph(workspace_id, c, r);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                        glyph_under_cursor = glyph;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                }

                // workspace window
                Key::Ctrl_Down => {
                    // workspace_window.move_cursor_down(glyph_under_cursor);
                    mgr.set_glyph(workspace_id, glyph_under_cursor, c, r);
                    if r < matrix_rows + 1 {
                        r += 1;
                    } else {
                        r = 2;
                    }
                    mgr.get_glyph(workspace_id, c, r);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                        glyph_under_cursor = glyph;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                }

                // workspace window
                Key::Backspace => {
                    // workspace_window.erase_glyph();
                    mgr.set_glyph(workspace_id, Glyph::default(), c, r);
                    if c > 2 {
                        c -= 1
                    } else {
                        r -= 1;
                        c = 5;
                    }
                    mgr.set_glyph(workspace_id, g, c, r);
                }

                // style window
                Key::Alt_Shift_Down => {
                    style_window.move_selector_down();
                }
                Key::Alt_Ctrl_Shift_Down => {
                    style_window.move_selector_bottom();
                }
                Key::Alt_Shift_Up => {
                    style_window.move_selector_up();
                }
                Key::Alt_Ctrl_Shift_Up => {
                    style_window.move_selector_top();
                }
                Key::Alt_Shift_Left => {
                    style_window.disable_selected_style();
                }
                Key::Alt_Shift_Right => {
                    style_window.enable_selected_style();
                }

                // exit program
                Key::Escape | Key::Q | Key::Ctrl_q => {
                    keep_running = false;
                    break;
                }

                _ => {
                    continue;
                }
            }
        }
    }
    mgr.terminate();
}
