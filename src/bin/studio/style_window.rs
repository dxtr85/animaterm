use animaterm::glyph::Glyph;
use animaterm::Message;
use std::sync::mpsc;

pub struct StyleWindow {
    pub style_glyph: Glyph,
    sender: mpsc::Sender<Message>,
    glyph_matrix_id: usize,
    style_window_id: usize,
    style_selector_id: usize,
    style_transparent_id: usize,
    style_bright_id: usize,
    style_italic_id: usize,
    style_underline_id: usize,
    style_blink_id: usize,
    style_blinkfast_id: usize,
    style_reverse_id: usize,
    style_strike_id: usize,
    style_selector_value: usize,
}

impl StyleWindow {
    pub fn new(
        sender: mpsc::Sender<Message>,
        glyph_matrix_id: usize,
        style_window_id: usize,
        style_selector_id: usize,
        style_transparent_id: usize,
        style_bright_id: usize,
        style_italic_id: usize,
        style_underline_id: usize,
        style_blink_id: usize,
        style_blinkfast_id: usize,
        style_reverse_id: usize,
        style_strike_id: usize,
    ) -> Self {
        StyleWindow {
            style_glyph: Glyph::default(),
            sender,
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
            style_selector_value: 0,
        }
    }

    pub fn move_selector_up(&mut self) {
        if self.style_selector_value == 0 {
            self.style_selector_value = 7;
        } else {
            self.style_selector_value -= 1;
        }
        self.sender.send(Message::SetGraphic(
            self.style_selector_id,
            self.style_selector_value,
            true,
        ));
    }

    pub fn move_selector_down(&mut self) {
        if self.style_selector_value == 7 {
            self.style_selector_value = 0;
        } else {
            self.style_selector_value += 1;
        }
        self.sender.send(Message::SetGraphic(
            self.style_selector_id,
            self.style_selector_value,
            true,
        ));
    }

    pub fn move_selector_top(&mut self) {
        self.style_selector_value = 0;
        self.sender.send(Message::SetGraphic(
            self.style_selector_id,
            self.style_selector_value,
            true,
        ));
    }

    pub fn move_selector_bottom(&mut self) {
        self.style_selector_value = 7;
        self.sender.send(Message::SetGraphic(
            self.style_selector_id,
            self.style_selector_value,
            true,
        ));
    }

    pub fn disable_selected_style(&mut self) {
        match self.style_selector_value {
            0 => {
                self.sender
                    .send(Message::SetGraphic(self.style_transparent_id, 0, true));
                self.style_glyph.set_transparent(false);
            }
            1 => {
                self.sender
                    .send(Message::SetGraphic(self.style_bright_id, 0, true));
                self.style_glyph.set_bright(false);
            }
            2 => {
                self.sender
                    .send(Message::SetGraphic(self.style_italic_id, 0, true));
                self.style_glyph.set_italic(false);
            }
            3 => {
                self.sender
                    .send(Message::SetGraphic(self.style_underline_id, 0, true));
                self.style_glyph.set_underline(false);
            }
            4 => {
                self.sender
                    .send(Message::SetGraphic(self.style_blink_id, 0, true));
                self.style_glyph.set_blink(false);
            }
            5 => {
                self.sender
                    .send(Message::SetGraphic(self.style_blinkfast_id, 0, true));
                self.style_glyph.set_blinkfast(false);
            }
            6 => {
                self.sender
                    .send(Message::SetGraphic(self.style_reverse_id, 0, true));
                self.style_glyph.set_reverse(false);
            }
            7 => {
                self.sender
                    .send(Message::SetGraphic(self.style_strike_id, 0, true));
                self.style_glyph.set_strike(false);
            }
            _ => {}
        };
        self.sender.send(Message::SetGraphicStyle(
            self.glyph_matrix_id,
            self.style_glyph,
        ));
    }

    pub fn enable_selected_style(&mut self) {
        match self.style_selector_value {
            0 => {
                self.sender
                    .send(Message::SetGraphic(self.style_transparent_id, 1, true));
                self.style_glyph.set_transparent(true);
            }
            1 => {
                self.sender
                    .send(Message::SetGraphic(self.style_bright_id, 1, true));
                self.style_glyph.set_bright(true);
            }
            2 => {
                self.sender
                    .send(Message::SetGraphic(self.style_italic_id, 1, true));
                self.style_glyph.set_italic(true);
            }
            3 => {
                self.sender
                    .send(Message::SetGraphic(self.style_underline_id, 1, true));
                self.style_glyph.set_underline(true);
            }
            4 => {
                self.sender
                    .send(Message::SetGraphic(self.style_blink_id, 1, true));
                self.style_glyph.set_blink(true);
            }
            5 => {
                self.sender
                    .send(Message::SetGraphic(self.style_blinkfast_id, 1, true));
                self.style_glyph.set_blinkfast(true);
            }
            6 => {
                self.sender
                    .send(Message::SetGraphic(self.style_reverse_id, 1, true));
                self.style_glyph.set_reverse(true);
            }
            7 => {
                self.sender
                    .send(Message::SetGraphic(self.style_strike_id, 1, true));
                self.style_glyph.set_strike(true);
            }
            _ => {}
        };
        self.sender.send(Message::SetGraphicStyle(
            self.glyph_matrix_id,
            self.style_glyph,
        ));
    }
}
