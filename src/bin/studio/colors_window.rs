use animaterm::Color;
use animaterm::Glyph;
use animaterm::Message;
use std::sync::mpsc;

pub struct ColorsWindow {
    sender: mpsc::Sender<Message>,
    basic_colors: [Color; 8],
    selected_tab: usize,
    selected_vertical_cursor: usize,
    basic_selected_color: usize,
    grayscale_selected_brightness: u8,
    eight_bit_selected_red: u8,
    eight_bit_selected_green: u8,
    eight_bit_selected_blue: u8,
    truecolor_bit_selected_red: u8,
    truecolor_bit_selected_green: u8,
    truecolor_bit_selected_blue: u8,
    color_window_id: usize,
    vertical_cursor_id: usize,
    basic_colors_id: usize,
    progress_bar_1_id: usize,
    progress_bar_1_title_id: usize,
    progress_bar_2_id: usize,
    progress_bar_2_title_id: usize,
    progress_bar_3_id: usize,
    progress_bar_3_title_id: usize,
    glyph_matrix_id: usize,
}

impl ColorsWindow {
    pub fn new(
        sender: mpsc::Sender<Message>,
        selected_tab: usize,
        selected_vertical_cursor: usize,
        color_window_id: usize,
        vertical_cursor_id: usize,
        basic_colors_id: usize,
        progress_bar_1_id: usize,
        progress_bar_1_title_id: usize,
        progress_bar_2_id: usize,
        progress_bar_2_title_id: usize,
        progress_bar_3_id: usize,
        progress_bar_3_title_id: usize,
        glyph_matrix_id: usize,
    ) -> Self {
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
        ColorsWindow {
            sender,
            basic_colors,
            selected_tab: 0,
            selected_vertical_cursor: 0,
            basic_selected_color: 0,
            grayscale_selected_brightness: 0,
            eight_bit_selected_red: 0,
            eight_bit_selected_green: 0,
            eight_bit_selected_blue: 0,
            truecolor_bit_selected_red: 0,
            truecolor_bit_selected_green: 0,
            truecolor_bit_selected_blue: 0,
            color_window_id,
            vertical_cursor_id,
            basic_colors_id,
            progress_bar_1_id,
            progress_bar_1_title_id,
            progress_bar_2_id,
            progress_bar_2_title_id,
            progress_bar_3_id,
            progress_bar_3_title_id,
            glyph_matrix_id,
        }
    }

    pub fn move_left(&mut self, background: bool) {
        if self.selected_vertical_cursor == 0 {
            if self.selected_tab == 0 {
                self.selected_tab = 3;
            } else {
                self.selected_tab -= 1;
            }
            self.sender.send(Message::SetGraphic(
                self.color_window_id,
                self.selected_tab,
                true,
            ));
            match self.selected_tab {
                0 => {
                    self.sender
                        .send(Message::SetInvisible(self.basic_colors_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_title_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_title_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_title_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_id, true));
                } //TODO basic color select visible
                1 => {
                    //Grayscale
                    self.sender
                        .send(Message::SetGraphic(self.progress_bar_1_title_id, 1, true));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.grayscale_selected_brightness as usize * 10,
                        true,
                    ));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_title_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_title_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_id, true));
                } //TODO basic color invisible
                2 => {
                    //8-bit
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.eight_bit_selected_red as usize * 51,
                        true,
                    ));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_2_id,
                        self.eight_bit_selected_green as usize * 51,
                        true,
                    ));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_3_id,
                        self.eight_bit_selected_blue as usize * 51,
                        true,
                    ));
                    //mgr.set_graphic(pb1t_id, 0, true);
                }
                3 => {
                    //Truecolor
                    self.sender
                        .send(Message::SetInvisible(self.basic_colors_id, true));
                    self.sender
                        .send(Message::SetGraphic(self.progress_bar_1_title_id, 0, true));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.truecolor_bit_selected_red as usize,
                        true,
                    ));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_2_id,
                        self.truecolor_bit_selected_green as usize,
                        true,
                    ));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_3_id,
                        self.truecolor_bit_selected_blue as usize,
                        true,
                    ));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_id, false));
                }
                _ => {}
            }
        } else {
            match (self.selected_vertical_cursor, self.selected_tab) {
                (1, 0) => {
                    //Basic
                    if self.basic_selected_color == 0 {
                        self.basic_selected_color = 7;
                    } else {
                        self.basic_selected_color -= 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.basic_colors_id,
                        self.basic_selected_color,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            self.basic_colors[self.basic_selected_color],
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            self.basic_colors[self.basic_selected_color],
                        ));
                    }
                }
                (1, 1) => {
                    //Grayscale brightness
                    if self.grayscale_selected_brightness == 0 {
                        self.grayscale_selected_brightness = 23;
                    } else {
                        self.grayscale_selected_brightness -= 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.grayscale_selected_brightness as usize * 10,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_gray(self.grayscale_selected_brightness),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_gray(self.grayscale_selected_brightness),
                        ));
                    }
                }
                (1, 2) => {
                    //8-bit red
                    if self.eight_bit_selected_red == 0 {
                        self.eight_bit_selected_red = 5;
                    } else {
                        self.eight_bit_selected_red -= 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.eight_bit_selected_red as usize * 51,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (2, 2) => {
                    //8-bit green
                    if self.eight_bit_selected_green == 0 {
                        self.eight_bit_selected_green = 5;
                    } else {
                        self.eight_bit_selected_green -= 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_2_id,
                        self.eight_bit_selected_green as usize * 51,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (3, 2) => {
                    //8-bit blue
                    if self.eight_bit_selected_blue == 0 {
                        self.eight_bit_selected_blue = 5;
                    } else {
                        self.eight_bit_selected_blue -= 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_3_id,
                        self.eight_bit_selected_blue as usize * 51,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (1, 3) => {
                    //Truecolor red
                    if self.truecolor_bit_selected_red == 0 {
                        self.truecolor_bit_selected_red = 255;
                    } else {
                        self.truecolor_bit_selected_red -= 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.truecolor_bit_selected_red as usize,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (2, 3) => {
                    //Truecolor green
                    if self.truecolor_bit_selected_green == 0 {
                        self.truecolor_bit_selected_green = 255;
                    } else {
                        self.truecolor_bit_selected_green -= 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_2_id,
                        self.truecolor_bit_selected_green as usize,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (3, 3) => {
                    //Truecolor blue
                    if self.truecolor_bit_selected_blue == 0 {
                        self.truecolor_bit_selected_blue = 255;
                    } else {
                        self.truecolor_bit_selected_blue -= 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_3_id,
                        self.truecolor_bit_selected_blue as usize,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    pub fn move_right(&mut self, background: bool) {
        if self.selected_vertical_cursor == 0 {
            self.selected_tab += 1;
            if self.selected_tab > 3 {
                self.selected_tab = 0;
            }
            self.sender.send(Message::SetGraphic(
                self.color_window_id,
                self.selected_tab,
                true,
            ));
            match self.selected_tab {
                0 => {
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_title_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_title_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_title_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_id, true));
                    self.sender
                        .send(Message::SetInvisible(self.basic_colors_id, false));
                }
                1 => {
                    //Grayscale
                    self.sender
                        .send(Message::SetGraphic(self.progress_bar_1_title_id, 1, true));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.grayscale_selected_brightness as usize * 10,
                        true,
                    ));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.basic_colors_id, true));
                }
                2 => {
                    //8-bit
                    self.sender
                        .send(Message::SetGraphic(self.progress_bar_1_title_id, 0, true));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.eight_bit_selected_red as usize * 51,
                        true,
                    ));
                    self.sender
                        .send(Message::SetGraphic(self.progress_bar_2_id, 0, true));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_2_id,
                        self.eight_bit_selected_green as usize * 51,
                        true,
                    ));
                    self.sender
                        .send(Message::SetGraphic(self.progress_bar_3_title_id, 0, true));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_3_id,
                        self.eight_bit_selected_blue as usize * 51,
                        true,
                    ));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_id, false));
                }
                3 => {
                    //Truecolor
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.truecolor_bit_selected_red.into(),
                        true,
                    ));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_2_id,
                        self.truecolor_bit_selected_green.into(),
                        true,
                    ));
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_3_id,
                        self.truecolor_bit_selected_blue.into(),
                        true,
                    ));
                }
                _ => {}
            }
        } else {
            match (self.selected_vertical_cursor, self.selected_tab) {
                (1, 0) => {
                    //Basic
                    self.basic_selected_color += 1;
                    if self.basic_selected_color > 7 {
                        self.basic_selected_color = 0;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.basic_colors_id,
                        self.basic_selected_color,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            self.basic_colors[self.basic_selected_color],
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            self.basic_colors[self.basic_selected_color],
                        ));
                    }
                }
                (1, 1) => {
                    //Grayscale brightness
                    self.grayscale_selected_brightness += 1;
                    if self.grayscale_selected_brightness > 23 {
                        self.grayscale_selected_brightness = 0;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.grayscale_selected_brightness as usize * 10,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_gray(self.grayscale_selected_brightness),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_gray(self.grayscale_selected_brightness),
                        ));
                    }
                }
                (1, 2) => {
                    //8-bit red
                    self.eight_bit_selected_red += 1;
                    if self.eight_bit_selected_red > 5 {
                        self.eight_bit_selected_red = 0;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.eight_bit_selected_red as usize * 51,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (2, 2) => {
                    //8-bit green
                    self.eight_bit_selected_green += 1;
                    if self.eight_bit_selected_green > 5 {
                        self.eight_bit_selected_green = 0;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_2_id,
                        self.eight_bit_selected_green as usize * 51,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (3, 2) => {
                    //8-bit blue
                    self.eight_bit_selected_blue += 1;
                    if self.eight_bit_selected_blue > 5 {
                        self.eight_bit_selected_blue = 0;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_3_id,
                        self.eight_bit_selected_blue as usize * 51,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_8bit(
                                self.eight_bit_selected_red,
                                self.eight_bit_selected_green,
                                self.eight_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (1, 3) => {
                    //Truecolor red
                    if self.truecolor_bit_selected_red > 254 {
                        self.truecolor_bit_selected_red = 0;
                    } else {
                        self.truecolor_bit_selected_red += 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_1_id,
                        self.truecolor_bit_selected_red as usize,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (2, 3) => {
                    //Truecolor green

                    if self.truecolor_bit_selected_green > 254 {
                        self.truecolor_bit_selected_green = 0;
                    } else {
                        self.truecolor_bit_selected_green += 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_2_id,
                        self.truecolor_bit_selected_green as usize,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    }
                }
                (3, 3) => {
                    //Truecolor blue
                    if self.truecolor_bit_selected_blue > 254 {
                        self.truecolor_bit_selected_blue = 0;
                    } else {
                        self.truecolor_bit_selected_blue += 1;
                    }
                    self.sender.send(Message::SetGraphic(
                        self.progress_bar_3_id,
                        self.truecolor_bit_selected_blue as usize,
                        true,
                    ));
                    if background {
                        self.sender.send(Message::SetGraphicBackground(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    } else {
                        self.sender.send(Message::SetGraphicColor(
                            self.glyph_matrix_id,
                            Color::new_truecolor(
                                self.truecolor_bit_selected_red,
                                self.truecolor_bit_selected_green,
                                self.truecolor_bit_selected_blue,
                            ),
                        ));
                    }
                }
                _ => {}
            }
        }
    }
    pub fn move_far_right(&mut self, background: bool) {
        match (self.selected_vertical_cursor, self.selected_tab) {
            (1, 1) => {
                //Grayscale brightness
                self.grayscale_selected_brightness += 5;
                if self.grayscale_selected_brightness > 23 {
                    self.grayscale_selected_brightness = 23;
                }
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_1_id,
                    self.grayscale_selected_brightness as usize * 10,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_gray(self.grayscale_selected_brightness),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_gray(self.grayscale_selected_brightness),
                    ));
                }
            }
            (1, 2) => {
                self.eight_bit_selected_red = 5;
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_1_id,
                    self.eight_bit_selected_red as usize * 51,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                }
            }
            (2, 2) => {
                //8-bit green
                self.eight_bit_selected_green = 5;
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_2_id,
                    self.eight_bit_selected_green as usize * 51,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                }
            }
            (3, 2) => {
                //8-bit blue
                self.eight_bit_selected_blue = 5;
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_3_id,
                    self.eight_bit_selected_blue as usize * 51,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                }
            }
            (1, 3) => {
                //Truecolor red
                if self.truecolor_bit_selected_red > 230 {
                    self.truecolor_bit_selected_red = 255;
                } else {
                    self.truecolor_bit_selected_red += 25;
                }
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_1_id,
                    self.truecolor_bit_selected_red as usize,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                }
            }
            (2, 3) => {
                //Truecolor green
                if self.truecolor_bit_selected_green > 230 {
                    self.truecolor_bit_selected_green = 255;
                } else {
                    self.truecolor_bit_selected_green += 25;
                }
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_2_id,
                    self.truecolor_bit_selected_green as usize,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                }
            }
            (3, 3) => {
                //Truecolor blue

                if self.truecolor_bit_selected_blue > 230 {
                    self.truecolor_bit_selected_blue = 255;
                } else {
                    self.truecolor_bit_selected_blue += 25;
                }
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_3_id,
                    self.truecolor_bit_selected_blue as usize,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                }
            }
            _ => {}
        }
    }

    pub fn move_far_left(&mut self, background: bool) {
        match (self.selected_vertical_cursor, self.selected_tab) {
            (1, 1) => {
                //Grayscale brightness
                if self.grayscale_selected_brightness < 6 {
                    self.grayscale_selected_brightness = 0
                } else {
                    self.grayscale_selected_brightness -= 5;
                }
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_1_id,
                    self.grayscale_selected_brightness as usize * 10,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_gray(self.grayscale_selected_brightness),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_gray(self.grayscale_selected_brightness),
                    ));
                }
            }
            (1, 2) => {
                self.eight_bit_selected_red = 0;
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_1_id,
                    self.eight_bit_selected_red as usize * 51,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                }
            }
            (2, 2) => {
                self.eight_bit_selected_green = 0;
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_2_id,
                    self.eight_bit_selected_green as usize * 51,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                }
            }
            (3, 2) => {
                //8-bit blue
                self.eight_bit_selected_blue = 0;
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_3_id,
                    self.eight_bit_selected_blue as usize * 51,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_8bit(
                            self.eight_bit_selected_red,
                            self.eight_bit_selected_green,
                            self.eight_bit_selected_blue,
                        ),
                    ));
                }
            }
            (1, 3) => {
                //Truecolor red
                if self.truecolor_bit_selected_red < 25 {
                    self.truecolor_bit_selected_red = 0;
                } else {
                    self.truecolor_bit_selected_red -= 25;
                }
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_1_id,
                    self.truecolor_bit_selected_red as usize,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                }
            }
            (2, 3) => {
                //Truecolor green
                if self.truecolor_bit_selected_green < 25 {
                    self.truecolor_bit_selected_green = 0;
                } else {
                    self.truecolor_bit_selected_green -= 25;
                }
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_2_id,
                    self.truecolor_bit_selected_green as usize,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                }
            }
            (3, 3) => {
                //Truecolor blue
                if self.truecolor_bit_selected_blue < 25 {
                    self.truecolor_bit_selected_blue = 0;
                } else {
                    self.truecolor_bit_selected_blue -= 25;
                }
                self.sender.send(Message::SetGraphic(
                    self.progress_bar_3_id,
                    self.truecolor_bit_selected_blue as usize,
                    true,
                ));
                if background {
                    self.sender.send(Message::SetGraphicBackground(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                } else {
                    self.sender.send(Message::SetGraphicColor(
                        self.glyph_matrix_id,
                        Color::new_truecolor(
                            self.truecolor_bit_selected_red,
                            self.truecolor_bit_selected_green,
                            self.truecolor_bit_selected_blue,
                        ),
                    ));
                }
            }
            _ => {}
        }
    }
    pub fn move_up(&mut self) {
        if self.selected_vertical_cursor == 0 {
            if self.selected_tab == 0 || self.selected_tab == 1 {
                self.selected_vertical_cursor = 1;
            } else {
                self.selected_vertical_cursor = 3;
            }
        } else {
            self.selected_vertical_cursor -= 1;
        }
        self.sender.send(Message::SetGraphic(
            self.vertical_cursor_id,
            self.selected_vertical_cursor,
            true,
        ));
    }

    pub fn move_down(&mut self) {
        if self.selected_vertical_cursor == 3 {
            self.selected_vertical_cursor = 0;
        } else {
            self.selected_vertical_cursor += 1;
            if self.selected_vertical_cursor > 1
                && (self.selected_tab == 0 || self.selected_tab == 1)
            {
                self.selected_vertical_cursor = 0;
            }
        }
        self.sender.send(Message::SetGraphic(
            self.vertical_cursor_id,
            self.selected_vertical_cursor,
            true,
        ));
    }

    pub fn move_top(&mut self) {
        self.selected_vertical_cursor = 0;
        self.sender.send(Message::SetGraphic(
            self.vertical_cursor_id,
            self.selected_vertical_cursor,
            true,
        ));
    }

    pub fn move_bottom(&mut self) {
        match self.selected_tab {
            0 | 1 => {
                self.selected_vertical_cursor = 1;
            }
            _ => {
                self.selected_vertical_cursor = 3;
            }
        }
        self.sender.send(Message::SetGraphic(
            self.vertical_cursor_id,
            self.selected_vertical_cursor,
            true,
        ));
    }

    pub fn set_invisible(&mut self, invisible: bool) {
        if invisible {
            self.sender
                .send(Message::SetInvisible(self.color_window_id, true));
            self.sender
                .send(Message::SetInvisible(self.vertical_cursor_id, true));
            self.sender
                .send(Message::SetInvisible(self.basic_colors_id, true));
            self.sender
                .send(Message::SetInvisible(self.progress_bar_1_id, true));
            self.sender
                .send(Message::SetInvisible(self.progress_bar_1_title_id, true));
            self.sender
                .send(Message::SetInvisible(self.progress_bar_2_id, true));
            self.sender
                .send(Message::SetInvisible(self.progress_bar_2_title_id, true));
            self.sender
                .send(Message::SetInvisible(self.progress_bar_3_id, true));
            self.sender
                .send(Message::SetInvisible(self.progress_bar_3_title_id, true));
        } else {
            self.sender
                .send(Message::SetInvisible(self.color_window_id, false));
            self.sender
                .send(Message::SetInvisible(self.vertical_cursor_id, false));
            match self.selected_tab {
                0 => {
                    self.sender
                        .send(Message::SetInvisible(self.basic_colors_id, false));
                }
                1 => {
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_title_id, false));
                }
                2 | 3 => {
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_1_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_2_title_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_id, false));
                    self.sender
                        .send(Message::SetInvisible(self.progress_bar_3_title_id, false));
                }
                _ => {}
            }
        }
    }
}
