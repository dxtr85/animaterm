use animaterm::prelude::*;
use animaterm::utilities::{progress_bar, text_to_frame, wrap_border_around};
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::mem::replace;
use std::path::Path;
use std::process::exit;

static ROWS_MIN: usize = 4;
static COLS_MIN: usize = 5;

fn main() {
    let basic_colors = [
        Color::black(),
        Color::red(),
        Color::green(),
        Color::yellow(),
        Color::blue(),
        Color::magenta(),
        Color::cyan(),
        Color::white(),
    ];
    let args = parse_arguments();
    let cols = args.cols;
    let rows = args.rows;
    verify_cols_and_rows(cols, rows);
    let mut mgr = Manager::new(true, cols, rows, None);
    let (cols, rows) = mgr.screen_size();
    let mut keep_running = true;
    let mut selector = build_selector();
    let sid = mgr.add_graphic(selector, 1, (0, 7));
    let gm = build_glyph_matrix(args.glyphs);
    let max_glyph_frame_id = gm.current_frame;
    let mut glyph_frame_id = 0;
    let gmid = mgr.add_graphic(gm, glyph_frame_id, (0, 7));
    let mut curr_col_tab = 3;
    let col_sel_id = mgr.add_graphic(build_color_selector(Some("Color")), 0, (0, 0));
    let mut curr_bg_tab = 1;
    let bg_sel_id = mgr.add_graphic(build_color_selector(Some("Background")), 0, (cols - 62, 0));
    let matrix_cols = 60;
    let matrix_rows = 10;
    let mut glyph = Glyph::default();
    glyph.set_color(Color::black());
    glyph.set_background(Color::white());
    let basic_sel_id = mgr.add_graphic(
        build_basic_colors_graphic(glyph, Glyph::default()),
        2,
        (4, 3),
    );
    let mut basic_selected_color = 0;
    let mut grayscale_selected_brightness: u8 = 24;
    let mut eight_bit_selected_red: u8 = 5;
    let mut eight_bit_selected_green = 4;
    let mut eight_bit_selected_blue = 3;
    let mut truecolor_bit_selected_red: u8 = 255;
    let mut truecolor_bit_selected_green = 255;
    let mut truecolor_bit_selected_blue = 255;
    glyph = Glyph::default();
    mgr.set_graphic(basic_sel_id, basic_selected_color, false);
    mgr.set_invisible(basic_sel_id, true);
    let did = mgr.add_graphic(build_empty_matrix(matrix_cols, matrix_rows), 0, (18, 7));
    let mut vc_cursor = 0;
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
        (2, 1),
    );
    mgr.set_graphic(vc_id, vc_cursor, true);
    let glyph2 = glyph.clone();
    glyph.set_color(Color::red());
    let pb1t_id = mgr.add_graphic(
        Graphic::from_texts(6, vec![("Red   ", glyph), ("Bright", glyph2)]),
        1,
        (4, 3),
    );
    mgr.set_graphic(pb1t_id, 0, true);
    glyph.set_color(Color::green());
    let pb2t_id = mgr.add_graphic(Graphic::from_text(6, "Green", glyph), 1, (4, 4));
    mgr.set_graphic(pb2t_id, 0, true);
    glyph.set_color(Color::blue());
    let pb3t_id = mgr.add_graphic(Graphic::from_text(6, "Blue", glyph), 1, (4, 5));
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
                Glyph::default_with_char('\u{2588}'),
            ]),
        ),
        1,
        (10, 3),
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
                Glyph::default_with_char('\u{2588}'),
            ]),
        ),
        1,
        (10, 4),
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
                Glyph::default_with_char('\u{2588}'),
            ]),
        ),
        1,
        (10, 5),
    );
    let mut reversed = Glyph::default();
    reversed.set_reverse(true);
    let mut style_graphics = build_style_graphics(reversed, Glyph::default());
    let mut style_glyph = Glyph::default();
    let style_window_id = mgr.add_graphic(style_graphics.remove(0), 0, (cols - 16, 7));
    let style_selector_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 14, 8));
    let style_transparent_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 12, 8));
    let style_bright_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 12, 9));
    let style_italic_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 12, 10));
    let style_underline_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 12, 11));
    let style_blink_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 12, 12));
    let style_blinkfast_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 12, 13));
    let style_reverse_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 12, 14));
    let style_strike_id = mgr.add_graphic(style_graphics.remove(0), 1, (cols - 12, 15));

    let mut style_selector_value = 0;
    let mut transparent_value = 0;
    let mut bright_value = 0;
    let mut italic_value = 0;
    let mut underline_value = 0;
    let mut blink_value = 0;
    let mut blinkfast_value = 0;
    let mut reverse_value = 0;
    let mut strike_value = 0;

    mgr.set_graphic(style_window_id, 0, true);
    mgr.set_graphic(style_selector_id, style_selector_value, true);
    mgr.set_graphic(style_transparent_id, transparent_value, true);
    mgr.set_graphic(style_bright_id, bright_value, true);
    mgr.set_graphic(style_italic_id, italic_value, true);
    mgr.set_graphic(style_underline_id, underline_value, true);
    mgr.set_graphic(style_blink_id, blink_value, true);
    mgr.set_graphic(style_blinkfast_id, blinkfast_value, true);
    mgr.set_graphic(style_reverse_id, reverse_value, true);
    mgr.set_graphic(style_strike_id, strike_value, true);

    mgr.set_graphic(pb1_id, truecolor_bit_selected_red as usize, true);
    mgr.set_graphic(pb2_id, truecolor_bit_selected_green as usize, true);
    mgr.set_graphic(pb3_id, truecolor_bit_selected_blue as usize, true);
    mgr.set_graphic(sid, 0, true);
    mgr.set_graphic(gmid, 0, true);
    mgr.set_graphic(did, 0, true);
    mgr.set_graphic(col_sel_id, curr_col_tab, true);
    mgr.set_graphic(bg_sel_id, curr_bg_tab, true);

    let mut c = 2;
    let mut r = 2;
    let mut glyph_under_cursor = Glyph::default();
    mgr.get_glyph(did, c, r);
    let result = mgr.read_result();
    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
        glyph_under_cursor = glyph;
    }

    let mut mc = 2;
    let mut mr = 2;
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
    mgr.set_glyph(did, g, c, r);
    while keep_running {
        if let Some(key) = mgr.read_key() {
            match key {
                //color window
                Key::Shift_Right => {
                    if vc_cursor == 0 {
                        curr_col_tab += 1;
                        if curr_col_tab > 3 {
                            curr_col_tab = 0;
                        }
                        mgr.set_graphic(col_sel_id, curr_col_tab, true);
                        match curr_col_tab {
                            0 => {
                                mgr.set_invisible(pb1t_id, true);
                                mgr.set_invisible(pb2t_id, true);
                                mgr.set_invisible(pb3t_id, true);
                                mgr.set_invisible(pb1_id, true);
                                mgr.set_invisible(pb2_id, true);
                                mgr.set_invisible(pb3_id, true);
                                mgr.set_invisible(basic_sel_id, false);
                            }
                            1 => {
                                //Grayscale
                                mgr.set_graphic(pb1t_id, 1, true);
                                mgr.set_graphic(
                                    pb1_id,
                                    grayscale_selected_brightness as usize * 10,
                                    true,
                                );
                                mgr.set_invisible(pb1t_id, false);
                                mgr.set_invisible(pb1_id, false);
                                mgr.set_invisible(basic_sel_id, true);
                            }
                            2 => {
                                //8-bit
                                mgr.set_graphic(pb1t_id, 0, true);
                                mgr.set_graphic(pb1_id, eight_bit_selected_red as usize * 51, true);
                                mgr.set_graphic(pb2_id, 0, true);
                                mgr.set_graphic(
                                    pb2_id,
                                    eight_bit_selected_green as usize * 51,
                                    true,
                                );
                                mgr.set_graphic(pb3t_id, 0, true);
                                mgr.set_graphic(
                                    pb3_id,
                                    eight_bit_selected_blue as usize * 51,
                                    true,
                                );
                                mgr.set_invisible(pb1t_id, false);
                                mgr.set_invisible(pb2t_id, false);
                                mgr.set_invisible(pb3t_id, false);
                                mgr.set_invisible(pb1_id, false);
                                mgr.set_invisible(pb2_id, false);
                                mgr.set_invisible(pb3_id, false);
                            }
                            3 => {
                                //Truecolor
                                mgr.set_graphic(pb1_id, truecolor_bit_selected_red as usize, true);
                                mgr.set_graphic(
                                    pb2_id,
                                    truecolor_bit_selected_green as usize,
                                    true,
                                );
                                mgr.set_graphic(pb3_id, truecolor_bit_selected_blue as usize, true);
                            }
                            _ => {
                                continue;
                            }
                        }
                    } else {
                        match (vc_cursor, curr_col_tab) {
                            (1, 0) => {
                                //Basic
                                basic_selected_color += 1;
                                if basic_selected_color > 7 {
                                    basic_selected_color = 0;
                                }
                                mgr.set_graphic(basic_sel_id, basic_selected_color, true);
                                mgr.set_graphic_color(gmid, basic_colors[basic_selected_color]);
                            }
                            (1, 1) => {
                                //Grayscale brightness
                                grayscale_selected_brightness += 1;
                                if grayscale_selected_brightness > 23 {
                                    grayscale_selected_brightness = 0;
                                }
                                mgr.set_graphic(
                                    pb1_id,
                                    grayscale_selected_brightness as usize * 10,
                                    true,
                                );
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_gray(grayscale_selected_brightness),
                                );
                            }
                            (1, 2) => {
                                //8-bit red
                                eight_bit_selected_red += 1;
                                if eight_bit_selected_red > 5 {
                                    eight_bit_selected_red = 0;
                                }
                                mgr.set_graphic(pb1_id, eight_bit_selected_red as usize * 51, true);
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_8bit(
                                        eight_bit_selected_red,
                                        eight_bit_selected_green,
                                        eight_bit_selected_blue,
                                    ),
                                );
                            }
                            (2, 2) => {
                                //8-bit green
                                eight_bit_selected_green += 1;
                                if eight_bit_selected_green > 5 {
                                    eight_bit_selected_green = 0;
                                }
                                mgr.set_graphic(
                                    pb2_id,
                                    eight_bit_selected_green as usize * 51,
                                    true,
                                );
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_8bit(
                                        eight_bit_selected_red,
                                        eight_bit_selected_green,
                                        eight_bit_selected_blue,
                                    ),
                                );
                            }
                            (3, 2) => {
                                //8-bit blue
                                eight_bit_selected_blue += 1;
                                if eight_bit_selected_blue > 5 {
                                    eight_bit_selected_blue = 0;
                                }
                                mgr.set_graphic(
                                    pb3_id,
                                    eight_bit_selected_blue as usize * 51,
                                    true,
                                );
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_8bit(
                                        eight_bit_selected_red,
                                        eight_bit_selected_green,
                                        eight_bit_selected_blue,
                                    ),
                                );
                            }
                            (1, 3) => {
                                //Truecolor red
                                if truecolor_bit_selected_red > 254 {
                                    truecolor_bit_selected_red = 0;
                                } else {
                                    truecolor_bit_selected_red += 1;
                                }
                                mgr.set_graphic(pb1_id, truecolor_bit_selected_red as usize, true);
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_truecolor(
                                        truecolor_bit_selected_red,
                                        truecolor_bit_selected_green,
                                        truecolor_bit_selected_blue,
                                    ),
                                );
                            }
                            (2, 3) => {
                                //Truecolor green

                                if truecolor_bit_selected_green > 254 {
                                    truecolor_bit_selected_green = 0;
                                } else {
                                    truecolor_bit_selected_green += 1;
                                }
                                mgr.set_graphic(
                                    pb2_id,
                                    truecolor_bit_selected_green as usize,
                                    true,
                                );
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_truecolor(
                                        truecolor_bit_selected_red,
                                        truecolor_bit_selected_green,
                                        truecolor_bit_selected_blue,
                                    ),
                                );
                            }
                            (3, 3) => {
                                //Truecolor blue
                                if truecolor_bit_selected_blue > 254 {
                                    truecolor_bit_selected_blue = 0;
                                } else {
                                    truecolor_bit_selected_blue += 1;
                                }
                                mgr.set_graphic(pb3_id, truecolor_bit_selected_blue as usize, true);
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_truecolor(
                                        truecolor_bit_selected_red,
                                        truecolor_bit_selected_green,
                                        truecolor_bit_selected_blue,
                                    ),
                                );
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }
                // color window
                Key::Ctrl_Shift_Right => {
                    match (vc_cursor, curr_col_tab) {
                        (1, 1) => {
                            //Grayscale brightness
                            grayscale_selected_brightness += 5;
                            if grayscale_selected_brightness > 23 {
                                grayscale_selected_brightness = 23;
                            }
                            mgr.set_graphic(
                                pb1_id,
                                grayscale_selected_brightness as usize * 10,
                                true,
                            );
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_gray(grayscale_selected_brightness),
                            );
                        }
                        (1, 2) => {
                            eight_bit_selected_red = 5;
                            mgr.set_graphic(pb1_id, eight_bit_selected_red as usize * 51, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_8bit(
                                    eight_bit_selected_red,
                                    eight_bit_selected_green,
                                    eight_bit_selected_blue,
                                ),
                            );
                        }
                        (2, 2) => {
                            //8-bit green
                            eight_bit_selected_green = 5;
                            mgr.set_graphic(pb2_id, eight_bit_selected_green as usize * 51, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_8bit(
                                    eight_bit_selected_red,
                                    eight_bit_selected_green,
                                    eight_bit_selected_blue,
                                ),
                            );
                        }
                        (3, 2) => {
                            //8-bit blue
                            eight_bit_selected_blue = 5;
                            mgr.set_graphic(pb3_id, eight_bit_selected_blue as usize * 51, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_8bit(
                                    eight_bit_selected_red,
                                    eight_bit_selected_green,
                                    eight_bit_selected_blue,
                                ),
                            );
                        }
                        (1, 3) => {
                            //Truecolor red
                            if truecolor_bit_selected_red > 230 {
                                truecolor_bit_selected_red = 255;
                            } else {
                                truecolor_bit_selected_red += 25;
                            }
                            mgr.set_graphic(pb1_id, truecolor_bit_selected_red as usize, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_truecolor(
                                    truecolor_bit_selected_red,
                                    truecolor_bit_selected_green,
                                    truecolor_bit_selected_blue,
                                ),
                            );
                        }
                        (2, 3) => {
                            //Truecolor green
                            if truecolor_bit_selected_green > 230 {
                                truecolor_bit_selected_green = 255;
                            } else {
                                truecolor_bit_selected_green += 25;
                            }
                            mgr.set_graphic(pb2_id, truecolor_bit_selected_green as usize, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_truecolor(
                                    truecolor_bit_selected_red,
                                    truecolor_bit_selected_green,
                                    truecolor_bit_selected_blue,
                                ),
                            );
                        }
                        (3, 3) => {
                            //Truecolor blue

                            if truecolor_bit_selected_blue > 230 {
                                truecolor_bit_selected_blue = 255;
                            } else {
                                truecolor_bit_selected_blue += 25;
                            }
                            mgr.set_graphic(pb3_id, truecolor_bit_selected_blue as usize, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_truecolor(
                                    truecolor_bit_selected_red,
                                    truecolor_bit_selected_green,
                                    truecolor_bit_selected_blue,
                                ),
                            );
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                //color window
                Key::Ctrl_Shift_Left => {
                    match (vc_cursor, curr_col_tab) {
                        (1, 1) => {
                            //Grayscale brightness
                            if grayscale_selected_brightness < 6 {
                                grayscale_selected_brightness = 0
                            } else {
                                grayscale_selected_brightness -= 5;
                            }
                            mgr.set_graphic(
                                pb1_id,
                                grayscale_selected_brightness as usize * 10,
                                true,
                            );
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_gray(grayscale_selected_brightness),
                            );
                        }
                        (1, 2) => {
                            eight_bit_selected_red = 0;
                            mgr.set_graphic(pb1_id, eight_bit_selected_red as usize * 51, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_8bit(
                                    eight_bit_selected_red,
                                    eight_bit_selected_green,
                                    eight_bit_selected_blue,
                                ),
                            );
                        }
                        (2, 2) => {
                            eight_bit_selected_green = 0;
                            mgr.set_graphic(pb2_id, eight_bit_selected_green as usize * 51, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_8bit(
                                    eight_bit_selected_red,
                                    eight_bit_selected_green,
                                    eight_bit_selected_blue,
                                ),
                            );
                        }
                        (3, 2) => {
                            //8-bit blue
                            eight_bit_selected_blue = 0;
                            mgr.set_graphic(pb3_id, eight_bit_selected_blue as usize * 51, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_8bit(
                                    eight_bit_selected_red,
                                    eight_bit_selected_green,
                                    eight_bit_selected_blue,
                                ),
                            );
                        }
                        (1, 3) => {
                            //Truecolor red
                            if truecolor_bit_selected_red < 25 {
                                truecolor_bit_selected_red = 0;
                            } else {
                                truecolor_bit_selected_red -= 25;
                            }
                            mgr.set_graphic(pb1_id, truecolor_bit_selected_red as usize, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_truecolor(
                                    truecolor_bit_selected_red,
                                    truecolor_bit_selected_green,
                                    truecolor_bit_selected_blue,
                                ),
                            );
                        }
                        (2, 3) => {
                            //Truecolor green
                            if truecolor_bit_selected_green < 25 {
                                truecolor_bit_selected_green = 0;
                            } else {
                                truecolor_bit_selected_green -= 25;
                            }
                            mgr.set_graphic(pb2_id, truecolor_bit_selected_green as usize, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_truecolor(
                                    truecolor_bit_selected_red,
                                    truecolor_bit_selected_green,
                                    truecolor_bit_selected_blue,
                                ),
                            );
                        }
                        (3, 3) => {
                            //Truecolor blue
                            if truecolor_bit_selected_blue < 25 {
                                truecolor_bit_selected_blue = 0;
                            } else {
                                truecolor_bit_selected_blue -= 25;
                            }
                            mgr.set_graphic(pb3_id, truecolor_bit_selected_blue as usize, true);
                            mgr.set_graphic_color(
                                gmid,
                                Color::new_truecolor(
                                    truecolor_bit_selected_red,
                                    truecolor_bit_selected_green,
                                    truecolor_bit_selected_blue,
                                ),
                            );
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                //color window
                Key::Shift_Up => {
                    if vc_cursor == 0 {
                        if curr_col_tab == 0 || curr_col_tab == 1 {
                            vc_cursor = 1;
                        } else {
                            vc_cursor = 3;
                        }
                    } else {
                        vc_cursor -= 1;
                    }
                    mgr.set_graphic(vc_id, vc_cursor, true);
                }
                //color window
                Key::Ctrl_Shift_Up => {
                    vc_cursor = 0;
                    mgr.set_graphic(vc_id, vc_cursor, true);
                }
                //color window
                Key::Shift_Down => {
                    if vc_cursor == 3 {
                        vc_cursor = 0;
                    } else {
                        vc_cursor += 1;
                        if vc_cursor > 1 && (curr_col_tab == 0 || curr_col_tab == 1) {
                            vc_cursor = 0;
                        }
                    }
                    mgr.set_graphic(vc_id, vc_cursor, true);
                }
                // Color window
                Key::Ctrl_Shift_Down => {
                    match curr_col_tab {
                        0 | 1 => {
                            vc_cursor = 1;
                        }
                        _ => {
                            vc_cursor = 3;
                        }
                    }
                    mgr.set_graphic(vc_id, vc_cursor, true);
                }

                // Color window
                Key::Shift_Left => {
                    if vc_cursor == 0 {
                        if curr_col_tab == 0 {
                            curr_col_tab = 3;
                        } else {
                            curr_col_tab -= 1;
                        }
                        mgr.set_graphic(col_sel_id, curr_col_tab, true);
                        match curr_col_tab {
                            0 => {
                                mgr.set_invisible(basic_sel_id, false);
                                mgr.set_invisible(pb1t_id, true);
                                mgr.set_invisible(pb2t_id, true);
                                mgr.set_invisible(pb3t_id, true);
                                mgr.set_invisible(pb1_id, true);
                                mgr.set_invisible(pb2_id, true);
                                mgr.set_invisible(pb3_id, true);
                            } //TODO basic color select visible
                            1 => {
                                //Grayscale
                                mgr.set_graphic(pb1t_id, 1, true);
                                mgr.set_graphic(
                                    pb1_id,
                                    grayscale_selected_brightness as usize * 10,
                                    true,
                                );
                                mgr.set_invisible(pb1t_id, false);
                                mgr.set_invisible(pb1_id, false);
                                mgr.set_invisible(pb2t_id, true);
                                mgr.set_invisible(pb3t_id, true);
                                mgr.set_invisible(pb2_id, true);
                                mgr.set_invisible(pb3_id, true);
                            } //TODO basic color invisible
                            2 => {
                                //8-bit
                                mgr.set_graphic(pb1_id, eight_bit_selected_red as usize * 51, true);
                                mgr.set_graphic(
                                    pb2_id,
                                    eight_bit_selected_green as usize * 51,
                                    true,
                                );
                                mgr.set_graphic(
                                    pb3_id,
                                    eight_bit_selected_blue as usize * 51,
                                    true,
                                );
                                //mgr.set_graphic(pb1t_id, 0, true);
                            }
                            3 => {
                                //Truecolor
                                mgr.set_invisible(basic_sel_id, true);
                                mgr.set_graphic(pb1t_id, 0, true);
                                mgr.set_graphic(pb1_id, truecolor_bit_selected_red as usize, true);
                                mgr.set_graphic(
                                    pb2_id,
                                    truecolor_bit_selected_green as usize,
                                    true,
                                );
                                mgr.set_graphic(pb3_id, truecolor_bit_selected_blue as usize, true);
                                mgr.set_invisible(pb1t_id, false);
                                mgr.set_invisible(pb2t_id, false);
                                mgr.set_invisible(pb3t_id, false);
                                mgr.set_invisible(pb1_id, false);
                                mgr.set_invisible(pb2_id, false);
                                mgr.set_invisible(pb3_id, false);
                            }
                            _ => {
                                continue;
                            }
                        }
                    } else {
                        match (vc_cursor, curr_col_tab) {
                            (1, 0) => {
                                //Basic
                                if basic_selected_color == 0 {
                                    basic_selected_color = 7;
                                } else {
                                    basic_selected_color -= 1;
                                }
                                mgr.set_graphic(basic_sel_id, basic_selected_color, true);
                                mgr.set_graphic_color(gmid, basic_colors[basic_selected_color]);
                            }
                            (1, 1) => {
                                //Grayscale brightness
                                if grayscale_selected_brightness == 0 {
                                    grayscale_selected_brightness = 23;
                                } else {
                                    grayscale_selected_brightness -= 1;
                                }
                                mgr.set_graphic(
                                    pb1_id,
                                    grayscale_selected_brightness as usize * 10,
                                    true,
                                );
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_gray(grayscale_selected_brightness),
                                );
                            }
                            (1, 2) => {
                                //8-bit red
                                if eight_bit_selected_red == 0 {
                                    eight_bit_selected_red = 5;
                                } else {
                                    eight_bit_selected_red -= 1;
                                }
                                mgr.set_graphic(pb1_id, eight_bit_selected_red as usize * 51, true);
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_8bit(
                                        eight_bit_selected_red,
                                        eight_bit_selected_green,
                                        eight_bit_selected_blue,
                                    ),
                                );
                            }
                            (2, 2) => {
                                //8-bit green
                                if eight_bit_selected_green == 0 {
                                    eight_bit_selected_green = 5;
                                } else {
                                    eight_bit_selected_green -= 1;
                                }
                                mgr.set_graphic(
                                    pb2_id,
                                    eight_bit_selected_green as usize * 51,
                                    true,
                                );
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_8bit(
                                        eight_bit_selected_red,
                                        eight_bit_selected_green,
                                        eight_bit_selected_blue,
                                    ),
                                );
                            }
                            (3, 2) => {
                                //8-bit blue
                                if eight_bit_selected_blue == 0 {
                                    eight_bit_selected_blue = 5;
                                } else {
                                    eight_bit_selected_blue -= 1;
                                }
                                mgr.set_graphic(
                                    pb3_id,
                                    eight_bit_selected_blue as usize * 51,
                                    true,
                                );
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_8bit(
                                        eight_bit_selected_red,
                                        eight_bit_selected_green,
                                        eight_bit_selected_blue,
                                    ),
                                );
                            }
                            (1, 3) => {
                                //Truecolor red
                                if truecolor_bit_selected_red == 0 {
                                    truecolor_bit_selected_red = 255;
                                } else {
                                    truecolor_bit_selected_red -= 1;
                                }
                                mgr.set_graphic(pb1_id, truecolor_bit_selected_red as usize, true);
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_truecolor(
                                        truecolor_bit_selected_red,
                                        truecolor_bit_selected_green,
                                        truecolor_bit_selected_blue,
                                    ),
                                );
                            }
                            (2, 3) => {
                                //Truecolor green
                                if truecolor_bit_selected_green == 0 {
                                    truecolor_bit_selected_green = 255;
                                } else {
                                    truecolor_bit_selected_green -= 1;
                                }
                                mgr.set_graphic(
                                    pb2_id,
                                    truecolor_bit_selected_green as usize,
                                    true,
                                );
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_truecolor(
                                        truecolor_bit_selected_red,
                                        truecolor_bit_selected_green,
                                        truecolor_bit_selected_blue,
                                    ),
                                );
                            }
                            (3, 3) => {
                                //Truecolor blue
                                if truecolor_bit_selected_blue == 0 {
                                    truecolor_bit_selected_blue = 255;
                                } else {
                                    truecolor_bit_selected_blue -= 1;
                                }
                                mgr.set_graphic(pb3_id, truecolor_bit_selected_blue as usize, true);
                                mgr.set_graphic_color(
                                    gmid,
                                    Color::new_truecolor(
                                        truecolor_bit_selected_red,
                                        truecolor_bit_selected_green,
                                        truecolor_bit_selected_blue,
                                    ),
                                );
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                }

                //color window
                Key::i => mgr.set_invisible(col_sel_id, true),
                Key::I => mgr.set_invisible(col_sel_id, false),

                //background window
                Key::Alt_i => mgr.set_invisible(bg_sel_id, true),
                Key::Alt_I => mgr.set_invisible(bg_sel_id, false),

                // Background window
                Key::Alt_Right => {
                    curr_bg_tab += 1;
                    if curr_bg_tab > 3 {
                        curr_bg_tab = 0;
                    }
                    mgr.set_graphic(bg_sel_id, curr_bg_tab, true);
                }

                // Background window
                Key::Alt_Left => {
                    if curr_bg_tab == 0 {
                        curr_bg_tab = 3;
                    } else {
                        curr_bg_tab -= 1;
                    }
                    mgr.set_graphic(bg_sel_id, curr_bg_tab, true);
                }

                //Glyphs window
                Key::Left => {
                    if mc > 2 {
                        mc -= 1;
                        mgr.move_graphic(sid, 2, (-1, 0))
                    } else {
                        mgr.move_graphic(sid, 2, (15, 0));
                        mc = 17;
                    }
                }

                // glyphs
                Key::Right => {
                    if mc < 17 {
                        mc += 1;
                        mgr.move_graphic(sid, 2, (1, 0));
                    } else {
                        mgr.move_graphic(sid, 2, (-15, 0));
                        mc = 2;
                    }
                }

                // glyphs
                Key::Up => {
                    if mr > 2 {
                        mr -= 1;
                        mgr.move_graphic(sid, 2, (0, -1))
                    } else {
                        mgr.move_graphic(sid, 2, (0, 9));
                        mr = 11;
                    }
                }

                //glyphs
                Key::Down => {
                    if mr < 11 {
                        mr += 1;
                        mgr.move_graphic(sid, 2, (0, 1))
                    } else {
                        mgr.move_graphic(sid, 2, (0, mr as isize * (-1) + 2));
                        mr = 2;
                    }
                }

                //glyphs
                Key::Space => {
                    mgr.start_animation(sid, 0);
                    mgr.get_glyph(gmid, mc, mr);
                    let result = mgr.read_result();
                    if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                        mgr.set_glyph(did, glyph, c, r);
                        if c < matrix_cols + 1 {
                            c += 1;
                        } else if r < matrix_rows + 1 {
                            c = 2;
                            r += 1;
                        } else {
                            c = 2;
                            r = 2;
                        }
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }

                //glyphs
                Key::PgUp => {
                    if glyph_frame_id == 0 {
                        glyph_frame_id = max_glyph_frame_id;
                    } else {
                        glyph_frame_id -= 1;
                    }
                    mgr.set_graphic(gmid, glyph_frame_id, true)
                }
                //glyphs
                Key::Home => {
                    glyph_frame_id = 0;
                    mgr.set_graphic(gmid, glyph_frame_id, true);
                }
                //glyphs
                Key::PgDn => {
                    if glyph_frame_id == max_glyph_frame_id {
                        glyph_frame_id = 0;
                    } else {
                        glyph_frame_id += 1;
                    }
                    mgr.set_graphic(gmid, glyph_frame_id, true)
                }
                //glyphs
                Key::End => {
                    glyph_frame_id = max_glyph_frame_id;
                    mgr.set_graphic(gmid, glyph_frame_id, true);
                }

                // workspace window
                Key::Ctrl_Left => {
                    let mut pos_changed = false;
                    if c > 2 {
                        mgr.set_glyph(did, glyph_under_cursor, c, r);
                        pos_changed = true;

                        c -= 1;
                    } else {
                        if r > 2 {
                            pos_changed = true;
                            mgr.set_glyph(did, glyph_under_cursor, c, r);
                            c = matrix_cols + 1;
                            r -= 1;
                        }
                    }
                    if pos_changed {
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }

                // Workspace window
                Key::Ctrl_Right => {
                    let mut pos_changed = false;
                    if c < matrix_cols + 1 {
                        mgr.set_glyph(did, glyph_under_cursor, c, r);
                        pos_changed = true;
                        c += 1;
                    } else {
                        if r < matrix_rows + 1 {
                            mgr.set_glyph(did, glyph_under_cursor, c, r);
                            pos_changed = true;
                            c = 2;
                            r += 1;
                        }
                    }
                    if pos_changed {
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }

                //workspace
                Key::Ctrl_Up => {
                    if r > 2 {
                        mgr.set_glyph(did, glyph_under_cursor, c, r);
                        r -= 1;
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }

                // workspace
                Key::Ctrl_Down => {
                    if r < matrix_rows + 1 {
                        mgr.set_glyph(did, glyph_under_cursor, c, r);
                        r += 1;
                        mgr.get_glyph(did, c, r);
                        let result = mgr.read_result();
                        if let Ok(AnimOk::GlyphRetrieved(_gid, glyph)) = result {
                            glyph_under_cursor = glyph;
                        }
                        mgr.set_glyph(did, g, c, r);
                    }
                }

                //workspace
                Key::Backspace => {
                    mgr.set_glyph(did, Glyph::default(), c, r);
                    if c > 2 {
                        c -= 1
                    } else {
                        r -= 1;
                        c = 5;
                    }
                    mgr.set_glyph(did, g, c, r);
                }

                //style window
                Key::Alt_Shift_Down => {
                    if style_selector_value == 7 {
                        style_selector_value = 0;
                    } else {
                        style_selector_value += 1;
                    }
                    mgr.set_graphic(style_selector_id, style_selector_value, true);
                }
                //style window
                Key::Alt_Ctrl_Shift_Down => {
                    style_selector_value = 7;
                    mgr.set_graphic(style_selector_id, style_selector_value, true);
                }
                //style window
                Key::Alt_Shift_Up => {
                    if style_selector_value == 0 {
                        style_selector_value = 7;
                    } else {
                        style_selector_value -= 1;
                    }
                    mgr.set_graphic(style_selector_id, style_selector_value, true);
                }
                //style window
                Key::Alt_Ctrl_Shift_Up => {
                    style_selector_value = 0;
                    mgr.set_graphic(style_selector_id, style_selector_value, true);
                }
                //style window
                Key::Alt_Shift_Left => match style_selector_value {
                    0 => {
                        transparent_value = 0;
                        mgr.set_graphic(style_transparent_id, transparent_value, true);
                        style_glyph.set_transparent(false);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    1 => {
                        bright_value = 0;
                        mgr.set_graphic(style_bright_id, bright_value, true);
                        style_glyph.set_bright(false);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    2 => {
                        italic_value = 0;
                        mgr.set_graphic(style_italic_id, italic_value, true);
                        style_glyph.set_italic(false);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    3 => {
                        underline_value = 0;
                        mgr.set_graphic(style_underline_id, underline_value, true);
                        style_glyph.set_underline(false);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    4 => {
                        blink_value = 0;
                        mgr.set_graphic(style_blink_id, blink_value, true);
                        style_glyph.set_blink(false);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    5 => {
                        blinkfast_value = 0;
                        mgr.set_graphic(style_blinkfast_id, blinkfast_value, true);
                        style_glyph.set_blinkfast(false);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    6 => {
                        reverse_value = 0;
                        mgr.set_graphic(style_reverse_id, reverse_value, true);
                        style_glyph.set_reverse(false);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    7 => {
                        strike_value = 0;
                        mgr.set_graphic(style_strike_id, strike_value, true);
                        style_glyph.set_strike(false);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    _ => continue,
                },
                //style window
                Key::Alt_Shift_Right => match style_selector_value {
                    0 => {
                        transparent_value = 1;
                        mgr.set_graphic(style_transparent_id, transparent_value, true);
                        style_glyph.set_transparent(true);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    1 => {
                        bright_value = 1;
                        mgr.set_graphic(style_bright_id, bright_value, true);
                        style_glyph.set_bright(true);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    2 => {
                        italic_value = 1;
                        mgr.set_graphic(style_italic_id, italic_value, true);
                        style_glyph.set_italic(true);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    3 => {
                        underline_value = 1;
                        mgr.set_graphic(style_underline_id, underline_value, true);
                        style_glyph.set_underline(true);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    4 => {
                        blink_value = 1;
                        mgr.set_graphic(style_blink_id, blink_value, true);
                        style_glyph.set_blink(true);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    5 => {
                        blinkfast_value = 1;
                        mgr.set_graphic(style_blinkfast_id, blinkfast_value, true);
                        style_glyph.set_blinkfast(true);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    6 => {
                        reverse_value = 1;
                        mgr.set_graphic(style_reverse_id, reverse_value, true);
                        style_glyph.set_reverse(true);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    7 => {
                        strike_value = 1;
                        mgr.set_graphic(style_strike_id, strike_value, true);
                        style_glyph.set_strike(true);
                        mgr.set_graphic_style(gmid, style_glyph);
                    }
                    _ => continue,
                },

                //exit program
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

struct Arguments {
    rows: Option<usize>,
    cols: Option<usize>,
    glyphs: Option<String>,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            rows: None,
            cols: None,
            glyphs: None,
        }
    }
}

enum ArgType {
    Rows,
    Cols,
    Glyphs,
}

enum WhatToParse {
    Name,
    Number,
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
                        "glyphs" => {
                            what_to_parse = WhatToParse::Name;
                            Some(ArgType::Glyphs)
                        }
                        &_ => None,
                    };
                } else {
                    match &name {
                        &Some(ArgType::Glyphs) => {
                            arguments.glyphs = Some(arg.trim().to_owned());
                        }
                        _ => {
                            eprintln!(
                        "\x1b[97;41;5mERR\x1b[m Expected argument name (e.g. --argument), got: {}",
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
            }
        }
    }
    arguments
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

fn build_selector() -> Graphic {
    let mut library = HashMap::with_capacity(2);
    let color = Color::white();
    let mut background = Color::black();
    let gt = Glyph::transparent();
    let mut h = Glyph::new(
        '\u{2500}', color, background, false, true, false, false, false, false, false, false,
    );
    let mut v = Glyph::new(
        '\u{2502}', color, background, false, true, false, false, false, false, false, false,
    );
    let cr = Glyph::new(
        '\u{253C}', color, background, false, true, false, false, false, false, false, false,
    );
    library.insert(0, vec![cr, h, cr, v, gt, v, cr, h, cr]);
    background = Color::yellow();
    h.set_background(background);
    v.set_background(background);
    let lu = Glyph::new(
        '\u{2518}', color, background, false, true, false, false, false, false, false, false,
    );
    let ru = Glyph::new(
        '\u{2514}', color, background, false, true, false, false, false, false, false, false,
    );
    let ld = Glyph::new(
        '\u{2510}', color, background, false, true, false, false, false, false, false, false,
    );
    let rd = Glyph::new(
        '\u{250C}', color, background, false, true, false, false, false, false, false, false,
    );
    library.insert(1, vec![rd, h, ld, v, gt, v, ru, h, lu]);
    let anim_select = Animation::new(
        false,
        false,
        vec![(1, Timestamp::new(0, 100)), (0, Timestamp::new(0, 1))],
        Timestamp::now(),
    );
    let mut anims = HashMap::new();
    anims.insert(0, anim_select);
    let mut gr = Graphic::new(3, 3, 0, library, Some(anims));
    gr
}

fn build_glyph_matrix(index_file: Option<String>) -> Graphic {
    let mut glyph_files = vec![];
    if let Some(index_file) = index_file {
        if let Ok(file) = File::open(index_file) {
            for line in io::BufReader::new(file).lines() {
                glyph_files.push(line);
            }
        }
    }
    //    let start_points = vec![9472, 9488, 9504, 9520, 9536, 9552, 9568, 9584, 9600, 9616];
    let border = [
        Glyph::default_with_char('\u{256D}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256E}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2570}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256F}'),
    ];
    let mut library = HashMap::new();
    let cols = 16;
    let rows = 10; //start_points.len();
    let mut g = Glyph::default();
    g.set_color(Color::new_truecolor(48, 250, 13));
    g.set_background(Color::new_8bit(0, 5, 5));
    let mut avail_index = 0;
    for file in glyph_files {
        if let Ok(file_name) = file {
            if file_name.trim().starts_with('#') {
                continue;
            }
            if let Ok(file) = File::open(&file_name) {
                g.set_char(' ');
                let mut frame = vec![g; cols * rows];
                let mut start_points = vec![];
                for line in io::BufReader::new(file).lines() {
                    if let Ok(line) = line {
                        if line.trim().starts_with('#') {
                            continue;
                        }
                        if let Ok(number) = line.parse::<u32>() {
                            start_points.push(number);
                        }
                    }
                }
                let mut next_to_replace = 0;
                for sp in start_points {
                    for i in 0..cols {
                        g.set_char(char::from_u32(sp + i as u32).unwrap());
                        //(sp + i as u32) as char
                        replace(&mut frame[next_to_replace], g.clone());
                        next_to_replace += 1;
                    }
                }
                let mut name = file_name;
                if name.contains('/') {
                    name = name.split('/').last().unwrap().to_string();
                }

                library.insert(
                    avail_index,
                    wrap_border_around(frame, cols, border, Some(&name)),
                );
                avail_index += 1;
            }
        }
    }
    let mut frame = Vec::with_capacity(cols * rows);
    let mut start_points = vec![9472, 9488, 9504, 9520, 9536, 9552, 9568, 9584, 9600, 9616];
    for sp in start_points {
        for i in 0..cols {
            g.set_char(char::from_u32(sp + i as u32).unwrap());
            //(sp + i as u32) as char
            //replace(&mut frame[next_to_replace], g.clone());
            frame.push(g);
        }
    }
    library.insert(
        avail_index,
        wrap_border_around(frame, cols, border, Some("default".into())),
    );

    Graphic::new(cols + 2, rows + 2, avail_index, library, None)
}

fn build_empty_matrix(cols: usize, rows: usize) -> Graphic {
    let border = [
        Glyph::default_with_char('\u{256D}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256E}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2570}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256F}'),
    ];
    let mut library = HashMap::new();
    let mut g = Glyph::default();
    let mut frame = vec![g; cols * rows];
    library.insert(
        0,
        wrap_border_around(frame, cols, border, Some("Workspace")),
    );
    let mut gr = Graphic::new(cols + 2, rows + 2, 0, library, None);
    gr
}

fn build_color_selector(title: Option<&str>) -> Graphic {
    let border = [
        Glyph::default_with_char('\u{256D}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256E}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2570}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256F}'),
    ];
    let cols = 60;
    let rows = 5;
    let mut glyph = Glyph::default();
    glyph.set_reverse(true);
    let mut text = text_to_frame("Basic ", glyph);
    glyph.set_reverse(false);
    text.append(&mut text_to_frame("Grayscale 8-bit TrueColor", glyph));
    let mut frame = vec![Glyph::default(); cols * rows];
    for (i, t) in text.iter().enumerate() {
        replace(&mut frame[i + 2], *t);
    }
    let mut library = HashMap::new();
    library.insert(0, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("Grayscale ", glyph));
    glyph.set_reverse(false);
    text.append(&mut text_to_frame("8-bit TrueColor", glyph));
    for (i, t) in text.iter().enumerate() {
        replace(&mut frame[i + 2], *t);
    }
    library.insert(1, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic Grayscale ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("8-bit ", glyph));
    glyph.set_reverse(false);
    text.append(&mut text_to_frame("TrueColor ", glyph));
    for (i, t) in text.iter().enumerate() {
        replace(&mut frame[i + 2], *t);
    }
    library.insert(2, wrap_border_around(frame.clone(), cols, border, title));
    text = text_to_frame("Basic Grayscale 8-bit ", glyph);
    glyph.set_reverse(true);
    text.append(&mut text_to_frame("Truecolor ", glyph));
    for (i, t) in text.iter().enumerate() {
        replace(&mut frame[i + 2], *t);
    }
    library.insert(3, wrap_border_around(frame, cols, border, title));
    let mut gr = Graphic::new(cols + 2, rows + 2, 0, library, None);
    gr
}

fn build_basic_colors_graphic(mut selected: Glyph, mut deselected: Glyph) -> Graphic {
    let mut library = HashMap::new();

    let mut text = text_to_frame("Black   ", selected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(0, text);

    text = text_to_frame("Black   ", deselected);
    selected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", selected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(1, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    selected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", selected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(2, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    selected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", selected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(3, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    selected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", selected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(4, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    selected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", selected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(5, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    selected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", selected));
    deselected.set_color(Color::white());
    text.append(&mut text_to_frame("White   ", deselected));
    library.insert(6, text);

    text = text_to_frame("Black   ", deselected);
    deselected.set_color(Color::red());
    text.append(&mut text_to_frame("Red     ", deselected));
    deselected.set_color(Color::green());
    text.append(&mut text_to_frame("Green   ", deselected));
    deselected.set_color(Color::yellow());
    text.append(&mut text_to_frame("Yellow  ", deselected));
    deselected.set_color(Color::blue());
    text.append(&mut text_to_frame("Blue    ", deselected));
    deselected.set_color(Color::magenta());
    text.append(&mut text_to_frame("Magenta ", deselected));
    deselected.set_color(Color::cyan());
    text.append(&mut text_to_frame("Cyan    ", deselected));
    selected.set_color(Color::black());
    text.append(&mut text_to_frame("White   ", selected));
    library.insert(7, text);

    Graphic::new(32, 2, 0, library, None)
}

fn build_style_graphics(mut selected: Glyph, mut deselected: Glyph) -> Vec<Graphic> {
    let border = [
        Glyph::default_with_char('\u{256D}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256E}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2502}'),
        Glyph::default_with_char('\u{2570}'),
        Glyph::default_with_char('\u{2500}'),
        Glyph::default_with_char('\u{256F}'),
    ];
    let mut library = HashMap::new();
    let glyph = Glyph::default();
    let selection = Graphic::from_texts(
        1,
        vec![
            ("\u{25C6}       ", glyph),
            (" \u{25C6}      ", glyph),
            ("  \u{25C6}     ", glyph),
            ("   \u{25C6}    ", glyph),
            ("    \u{25C6}   ", glyph),
            ("     \u{25C6}  ", glyph),
            ("      \u{25C6} ", glyph),
            ("       \u{25C6}", glyph),
        ],
    );
    library.insert(
        0,
        wrap_border_around(vec![Glyph::default(); 14 * 8], 14, border, Some("Style")),
    );
    let style_window = Graphic::new(16, 10, 0, library, None);

    let transparent = Graphic::from_texts(
        11,
        vec![("Transparent", deselected), ("Transparent", selected)],
    );
    let bright = Graphic::from_texts(6, vec![("Bright", deselected), ("Bright", selected)]);
    let italic = Graphic::from_texts(6, vec![("Italic", deselected), ("Italic", selected)]);
    let underline =
        Graphic::from_texts(9, vec![("Underline", deselected), ("Underline", selected)]);
    let blink = Graphic::from_texts(5, vec![("Blink", deselected), ("Blink", selected)]);
    let blinkfast = Graphic::from_texts(
        10,
        vec![("Blink fast", deselected), ("Blink fast", selected)],
    );
    let reverse = Graphic::from_texts(7, vec![("Reverse", deselected), ("Reverse", selected)]);
    let strike = Graphic::from_texts(6, vec![("Strike", deselected), ("Strike", selected)]);
    let mut result = Vec::with_capacity(10);
    result.push(style_window);
    result.push(selection);
    result.push(transparent);
    result.push(bright);
    result.push(italic);
    result.push(underline);
    result.push(blink);
    result.push(blinkfast);
    result.push(reverse);
    result.push(strike);
    result
}
