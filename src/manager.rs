use super::animation::Animation;
use super::color::Color;
use super::error::AnimError;
use super::glyph::Glyph;
use super::graphic::Graphic;
use super::helpers::map_bytes_to_key;
use super::key::Key;
use super::response::AnimOk::{self, *};
use super::screen::Screen;
use super::Timestamp;
use std::io;
use std::io::Read;
use std::mem::replace;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum Message {
    Finish,
    EmptyFrame(usize),
    CloneFrame(usize, Option<usize>),
    AddAnimation(usize, Animation),
    StartAnimation(usize, usize),
    EnqueueAnimation(usize, usize, Timestamp),
    PauseAnimation(usize),
    PauseAnimationOnFrame(usize, usize),
    StopAnimation(usize),
    RestartAnimation(usize, usize, Timestamp),
    AddGraphic(Graphic, usize, (usize, usize)),
    SetGlyph(usize, Glyph, usize, usize),
    GetGlyph(usize, usize, usize),
    SetGraphic(usize, usize, bool),
    SetGraphicColor(usize, Color),
    SetGraphicBackground(usize, Color),
    SetGraphicStyle(usize, Glyph),
    SetInvisible(usize, bool),
    MoveGraphic(usize, usize, (isize, isize)),
    DeleteGraphic(usize),
    NewDisplay(usize, bool),
    RestoreDisplay(usize, bool),
    PrintGraphic(usize, bool),
    PrintScreen,
    PrintScreenSection((usize, usize), usize, usize),
}

pub struct Manager {
    scrn_size: (usize, usize),
    join_handle: thread::JoinHandle<()>,
    //    next_id: usize,
    next_screen_id: usize,
    sender: mpsc::Sender<Message>,
    key_receiver: Option<mpsc::Receiver<u8>>,
    key_recv_timeout: Duration,
    result_receiver: Option<mpsc::IntoIter<Result<AnimOk, AnimError>>>,
}

impl Manager {
    pub fn new(
        capture_keyboard: bool,
        cols: Option<usize>,
        rows: Option<usize>,
        glyph: Option<Glyph>,
        screen_refresh_timeout: Option<Duration>,
    ) -> Self {
        let mut screen = Screen::new(cols, rows, glyph);
        let cols = screen.cols;
        let rows = screen.rows;
        screen.initialize();
        screen.cls();
        let (sender, receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();

        // current granularity of Timestamp structure is 1ms
        let mut refresh_timeout = Duration::from_millis(30);
        if let Some(dur) = screen_refresh_timeout {
            refresh_timeout = dur;
        }
        let join_handle = thread::spawn(move || {
            let mut finish = false;
            while !finish {
                if let Ok(value) = receiver.recv_timeout(refresh_timeout) {
                    match value {
                        Message::Finish => {
                            finish = true;
                        }
                        Message::AddAnimation(gid, anim) => {
                            let add_result = screen.add_animation(gid, anim);
                            if let Some(id) = add_result {
                                if result_sender.send(Result::Ok(AnimationAdded(id))).is_err() {
                                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send AnimationAdded message.")
                                }
                            } else {
                                if result_sender
                                    .send(Result::Err(AnimError::FailAddingAnimation(gid)))
                                    .is_err()
                                {
                                    eprintln!(
                        "\x1b[97;41;5mERR\x1b[m Unable to send FailAddingAnimation message.")
                                }
                            };
                        }
                        Message::StartAnimation(gid, aid) => {
                            screen.start_animation(&gid, aid);
                        }
                        Message::PauseAnimation(gid) => {
                            screen.pause_animation(gid);
                        }
                        Message::PauseAnimationOnFrame(gid, fid) => {
                            screen.pause_animation_on_frame(&gid, fid);
                        }
                        Message::StopAnimation(gid) => {
                            screen.stop_animation(&gid);
                        }
                        Message::RestartAnimation(gid, aid, when) => {
                            screen.restart_animation(gid, aid, when);
                        }
                        Message::EnqueueAnimation(gid, aid, when) => {
                            screen.enqueue_animation(&gid, aid, when);
                        }
                        Message::AddGraphic(gr, layer, offset) => {
                            let graphic_id = screen.add_graphic(gr, layer, offset);
                            if result_sender
                                .send(Result::Ok(GraphicAdded(graphic_id)))
                                .is_err()
                            {
                                eprintln!(
                                    "\x1b[97;41;5mERR\x1b[m Failed to send GraphicAdded message"
                                );
                            }
                        }
                        Message::MoveGraphic(gid, layer, offset) => {
                            screen.move_graphic(gid, layer, offset);
                        }
                        Message::SetGlyph(gid, glyph, col, row) => {
                            screen.set_glyph(gid, glyph, col, row);
                        }
                        Message::GetGlyph(gid, col, row) => {
                            let result = screen.get_glyph(gid, col, row);
                            if let Some(glyph) = result {
                                if result_sender
                                    .send(Result::Ok(GlyphRetrieved(gid, glyph)))
                                    .is_err()
                                {
                                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send GlyphRetrieved message")
                                }
                            } else {
                                if result_sender
                                    .send(Result::Err(AnimError::FailGettingGlyph(gid)))
                                    .is_err()
                                {
                                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send FailGettingGlyph message")
                                }
                            };
                        }
                        Message::SetGraphic(gid, fid, force) => {
                            screen.set_graphic((&gid, &fid), force);
                        }
                        Message::SetGraphicColor(gid, color) => {
                            screen.set_graphic_color(gid, color);
                        }
                        Message::SetGraphicBackground(gid, color) => {
                            screen.set_graphic_background(gid, color);
                        }
                        Message::SetGraphicStyle(gid, style) => {
                            screen.set_graphic_style(gid, style);
                        }
                        Message::SetInvisible(gid, invisible) => {
                            screen.set_invisible(gid, invisible);
                        }
                        Message::DeleteGraphic(gid) => {
                            screen.delete_graphic(&gid);
                        }
                        Message::PrintScreen => {
                            let text = screen.print_screen();
                            if result_sender.send(Result::Ok(PrintScreen(text))).is_err() {
                                eprintln!(
                                    "\x1b[97;41;5mERR\x1b[m Unable to send PrintScreen message"
                                )
                            };
                        }
                        Message::PrintScreenSection(offset, cols, rows) => {
                            let text = screen.print_screen_section(offset, cols, rows);
                            if result_sender.send(Result::Ok(PrintScreen(text))).is_err() {
                                eprintln!(
                                    "\x1b[97;41;5mERR\x1b[m Unable to send PrintScreen message for section"
                                )
                            };
                        }
                        Message::PrintGraphic(gid, skip_border) => {
                            let text = screen.print_graphic(gid, skip_border);
                            if result_sender.send(Result::Ok(PrintScreen(text))).is_err() {
                                eprintln!(
                                    "\x1b[97;41;5mERR\x1b[m Unable to send PrintScreen message for Graphic"
                                )
                            };
                        }
                        Message::EmptyFrame(gid) => {
                            let result = screen.empty_frame(gid);
                            if let Some(id) = result {
                                if result_sender.send(Result::Ok(FrameAdded(gid, id))).is_err() {
                                    eprintln!(
                                        "\x1b[97;41;5mERR\x1b[m Unable to send FrameAdded message"
                                    )
                                }
                            } else {
                                if result_sender
                                    .send(Result::Err(AnimError::FailAddingFrame(gid)))
                                    .is_err()
                                {
                                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send FailAddingFrame message")
                                }
                            };
                        }
                        Message::CloneFrame(gid, fid) => {
                            let result = screen.clone_frame(gid, fid);
                            if let Some(id) = result {
                                if result_sender.send(Result::Ok(FrameAdded(gid, id))).is_err() {
                                    eprintln!(
                                        "\x1b[97;41;5mERR\x1b[m Unable to send FrameAdded message"
                                    )
                                }
                            } else {
                                if result_sender
                                    .send(Result::Err(AnimError::FailAddingFrame(gid)))
                                    .is_err()
                                {
                                    eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send FailAddingFrame message")
                                }
                            };
                        }
                        Message::NewDisplay(display_id, keep_existing) => {
                            // TODO deal with display_id - should it be provided by the user?
                            let _display_id = screen.new_display(display_id, keep_existing);
                            //result_sender.send(Result::Ok(DisplayCreated(display_id)));
                        }
                        Message::RestoreDisplay(display_id, keep_existing) => {
                            // if let Some(stored_display_id) =
                            screen.restore_display(display_id, keep_existing);
                            // {
                            //     result_sender.send(Result::Ok(DisplayRestored(stored_display_id)));
                            // }
                        }
                    }
                }
                screen.update_graphics();
            }
            screen.cleanup();
        });
        let mut key_receiver = None;
        if capture_keyboard {
            let (key_sender, key_rcver) = mpsc::sync_channel(1024);
            let mut reader = io::stdin();

            let mut buffer = [0; 1]; // read exactly one byte
                                     // print!("Hit a key! ");
                                     //let mut keys_read = HashSet::with_capacity(32);
                                     //let mut i = 0;
            let _kb_join_handle = thread::spawn(move || {
                let mut finish = false;
                while !finish {
                    if reader.read_exact(&mut buffer).is_err() {
                        eprintln!("\x1b[97;41;5mERR\x1b[m Unable to read to buffer")
                    }
                    if buffer[0] > 0 {
                        if key_sender.send(buffer[0]).is_err() {
                            finish = true;
                        }
                    }
                }
            });
            key_receiver = Some(key_rcver);
        }
        Manager {
            scrn_size: (cols, rows),
            join_handle,
            //next_id: 0,
            next_screen_id: 1,
            sender,
            key_receiver,
            key_recv_timeout: Duration::from_millis(1),
            result_receiver: Some(result_receiver.into_iter()),
        }
    }

    pub fn get_key_receiver(&mut self) -> Option<mpsc::Receiver<u8>> {
        replace(&mut self.key_receiver, None)
    }

    pub fn set_key_receive_timeout(&mut self, t: Duration) {
        self.key_recv_timeout = t;
    }

    pub fn get_message_sender(&mut self) -> mpsc::Sender<Message> {
        self.sender.clone()
    }

    fn read_bytes(&self, receiver: &Option<mpsc::Receiver<u8>>) -> Vec<u8> {
        let mut keys_read: Vec<u8> = Vec::with_capacity(10);
        if let Some(key_rcvr) = receiver {
            let mut all_bytes_read = false;
            if let Ok(first_byte) = key_rcvr.recv() {
                keys_read.push(first_byte);
                if first_byte != 27 && first_byte < 128 {
                    all_bytes_read = true
                }
                while !all_bytes_read {
                    match key_rcvr.recv_timeout(self.key_recv_timeout) {
                        Ok(byte) => keys_read.push(byte),
                        Err(_error) => {
                            all_bytes_read = true;
                        }
                    }
                }
            }
        } else {
            eprintln!("mgr has no key receiver!")
        }
        keys_read
    }
    pub fn read_key(&mut self) -> Option<Key> {
        let k_rcvr = replace(&mut self.key_receiver, None);
        //if let Some(key_rcvr) = k_rcvr {
        let keys_read = self.read_bytes(&k_rcvr);
        let _replaced = replace(&mut self.key_receiver, k_rcvr);
        return map_bytes_to_key(keys_read);
        // } else {
        //     eprintln!("mgr has no key receiver!");
        // }
    }

    pub fn read_line(&mut self) -> String {
        let k_rcvr = replace(&mut self.key_receiver, None);
        let mut all_bytes: Vec<u8> = Vec::with_capacity(128);
        let mut keys_read;
        let mut enter_pressed = false;
        while !enter_pressed {
            keys_read = self.read_bytes(&k_rcvr);
            if keys_read.len() == 1 && keys_read[0] == 10 {
                enter_pressed = true;
            } else if keys_read.len() > 0 {
                all_bytes.append(&mut keys_read);
            }
        }
        let _replaced = replace(&mut self.key_receiver, k_rcvr);
        String::from_utf8_lossy(&all_bytes).into_owned()
    }

    pub fn read_char(&mut self) -> Option<char> {
        let k_rcvr = replace(&mut self.key_receiver, None);
        let mut keys_read = Vec::new();
        let mut something_read = false;
        while !something_read {
            keys_read = self.read_bytes(&k_rcvr);
            if keys_read.len() > 0 {
                something_read = true;
            }
        }
        let _replaced = replace(&mut self.key_receiver, k_rcvr);
        String::from_utf8_lossy(&keys_read).chars().next()
    }

    pub fn set_key_iter(&mut self, receiver: mpsc::Receiver<u8>) -> Option<mpsc::Receiver<u8>> {
        replace(&mut self.key_receiver, Some(receiver))
    }

    pub fn get_result_iter(&mut self) -> Option<mpsc::IntoIter<Result<AnimOk, AnimError>>> {
        replace(&mut self.result_receiver, None)
    }

    pub fn set_result_iter(
        &mut self,
        receiver: mpsc::IntoIter<Result<AnimOk, AnimError>>,
    ) -> Option<mpsc::IntoIter<Result<AnimOk, AnimError>>> {
        replace(&mut self.result_receiver, Some(receiver))
    }

    pub fn read_result(&mut self) -> Result<AnimOk, AnimError> {
        if let Some(receiver) = &mut self.result_receiver {
            if let Some(result) = receiver.next() {
                return result;
            } else {
                return Ok(AnimOk::AllResultsRead);
            }
        }
        Err(AnimError::ResultReceiverNotSet)
    }

    pub fn screen_size(&self) -> (usize, usize) {
        self.scrn_size
    }

    pub fn add_animation(&mut self, graphic_id: usize, anim: Animation) {
        if self
            .sender
            .send(Message::AddAnimation(graphic_id, anim))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send AddAnimation message")
        };
    }
    pub fn start_animation(&self, graph_id: usize, anim_id: usize) {
        if self
            .sender
            .send(Message::StartAnimation(graph_id, anim_id))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send StartAnimation message")
        };
    }
    pub fn enqueue_animation(&self, graph_id: usize, anim_id: usize, when: Timestamp) {
        if self
            .sender
            .send(Message::EnqueueAnimation(graph_id, anim_id, when))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send EnqueueAnimation message")
        };
    }
    pub fn pause_animation(&self, graphic_id: usize) {
        if self
            .sender
            .send(Message::PauseAnimation(graphic_id))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send Pauseanimation message")
        };
    }
    pub fn pause_animation_on_frame(&self, graphic_id: usize, frame_id: usize) {
        if self
            .sender
            .send(Message::PauseAnimationOnFrame(graphic_id, frame_id))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send PauseAnimationOnFrame message")
        };
    }
    pub fn stop_animation(&self, graph_id: usize) {
        if self.sender.send(Message::StopAnimation(graph_id)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send StopAnimation message")
        };
    }
    pub fn restart_animation(&self, graphic_id: usize, anim_id: usize, when: Timestamp) {
        if self
            .sender
            .send(Message::RestartAnimation(graphic_id, anim_id, when))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send RestartAnimation message")
        };
    }

    pub fn move_graphic(&self, gid: usize, layer: usize, offset: (isize, isize)) {
        if self
            .sender
            .send(Message::MoveGraphic(gid, layer, offset))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send MoveGraphic message")
        };
    }
    pub fn set_invisible(&self, gid: usize, invisible: bool) {
        if self
            .sender
            .send(Message::SetInvisible(gid, invisible))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetInvisible message")
        };
    }
    pub fn set_glyph(&self, gid: usize, glyph: Glyph, col: usize, row: usize) {
        if self
            .sender
            .send(Message::SetGlyph(gid, glyph, col, row))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGlyph message")
        };
    }
    pub fn get_glyph(&self, gid: usize, col: usize, row: usize) {
        if self.sender.send(Message::GetGlyph(gid, col, row)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send GetGlyph message")
        };
    }

    pub fn load_graphic_from_file<P>(&self, filename: P) -> Result<AnimOk, AnimError>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        if let Some(graphic) = Graphic::from_file(filename) {
            return Ok(AnimOk::GraphicCreated(graphic));
        } else {
            return Err(AnimError::UnableToBuildGraphicFromFile);
        }
    }

    pub fn empty_frame(&self, gid: usize) {
        if self.sender.send(Message::EmptyFrame(gid)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send EmptyFrame message")
        };
    }
    //fn update_animations() {}

    pub fn cls() {}
    pub fn cla() {}
    pub fn new_display(&mut self, keep_existing: bool) -> usize {
        let new_id = self.next_screen_id;
        self.next_screen_id += 1;
        if self
            .sender
            .send(Message::NewDisplay(new_id, keep_existing))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send NewDisplay message")
        };
        new_id
    }
    pub fn restore_display(&mut self, display_id: usize, keep_existing: bool) {
        if self
            .sender
            .send(Message::RestoreDisplay(display_id, keep_existing))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send RestoreDisplay message")
        };
    }

    pub fn add_graphic(
        &mut self,
        gr: Graphic,
        layer: usize,
        offset: (usize, usize),
    ) -> Option<usize> {
        // let gid = self.next_id;
        // self.next_id += 1;
        if self
            .sender
            .send(Message::AddGraphic(gr, layer, offset))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send AddGraphic message")
        };
        //TODO gid should be returned by Screen
        let result = self.read_result();
        if let Ok(AnimOk::GraphicAdded(gid)) = result {
            return Some(gid);
        } else {
            eprintln!("Unable to read GraphicAdded message");
        }
        None
    }

    pub fn set_graphic(&self, gid: usize, fid: usize, force: bool) {
        if self
            .sender
            .send(Message::SetGraphic(gid, fid, force))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGraphic message")
        };
    }
    pub fn set_graphic_color(&self, gid: usize, color: Color) {
        if self
            .sender
            .send(Message::SetGraphicColor(gid, color))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGraphicColor message")
        };
    }
    pub fn set_graphic_background(&self, gid: usize, color: Color) {
        if self
            .sender
            .send(Message::SetGraphicBackground(gid, color))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGraphicBackground message")
        };
    }
    pub fn set_graphic_style(&self, gid: usize, glyph: Glyph) {
        if self
            .sender
            .send(Message::SetGraphicStyle(gid, glyph))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGraphicStyle message")
        };
    }
    pub fn delete_graphic(&self, gid: usize) {
        if self.sender.send(Message::DeleteGraphic(gid)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send DeleteGraphic message")
        };
    }

    pub fn print_graphic(&self, gid: usize, skip_border: bool) {
        if self
            .sender
            .send(Message::PrintGraphic(gid, skip_border))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send PrintGraphic message")
        };
    }

    pub fn print_screen(&self) {
        if self.sender.send(Message::PrintScreen).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send PrintScreen message")
        };
    }

    pub fn print_screen_section(&self, offset: (usize, usize), cols: usize, rows: usize) {
        if self
            .sender
            .send(Message::PrintScreenSection(offset, cols, rows))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send PrintScreenSection message")
        };
    }

    pub fn terminate(self) {
        if self.sender.send(Message::Finish).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send Finish message")
        };
        if self.join_handle.join().is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Failed to join thread")
        };
    }
}
