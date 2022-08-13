use super::animation::Animation;
use super::color::Color;
use super::error::AnimError;
use super::glyph::Glyph;
use super::graphic::Graphic;
use super::helpers::map_bytes_to_key;
use super::key::Key;
use super::response::AnimOk::{self, *};
use super::screen::Screen;
use super::time::Timestamp;
use std::io;
use std::io::Read;
use std::mem::replace;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Message {
    Finish,
    EmptyFrame(usize),
    CloneFrame(usize, Option<usize>),
    AddAnimation(usize, Animation),
    StartAnimation(usize, usize),
    PauseAnimation(usize),
    PauseAnimationOnFrame(usize, usize),
    StopAnimation(usize),
    RestartAnimation(usize),
    AddGraphic(usize, Graphic, usize, (usize, usize)),
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
}

pub struct Manager {
    scrn_size: (usize, usize),
    join_handle: thread::JoinHandle<()>,
    next_id: usize,
    next_screen_id: usize,
    sender: mpsc::Sender<Message>,
    key_receiver: Option<mpsc::Receiver<u8>>,
    result_receiver: Option<mpsc::IntoIter<Result<AnimOk, AnimError>>>,
}

impl Manager {
    pub fn new(
        capture_keyboard: bool,
        cols: Option<usize>,
        rows: Option<usize>,
        glyph: Option<Glyph>,
    ) -> Self {
        let mut screen = Screen::new(cols, rows, glyph);
        let cols = screen.cols;
        let rows = screen.rows;
        screen.initialize();
        screen.cls();
        let (sender, receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        // current granularity of Timestamp structure is 1ms
        let timeout = Duration::from_millis(1);
        let join_handle = thread::spawn(move || {
            let mut finish = false;
            while !finish {
                if let Ok(value) = receiver.recv_timeout(timeout) {
                    match value {
                        Message::Finish => {
                            finish = true;
                        }
                        Message::AddAnimation(gid, anim) => {
                            let add_result = screen.add_animation(gid, anim);
                            if let Some(id) = add_result {
                                result_sender.send(Result::Ok(AnimationAdded(id)))
                            } else {
                                result_sender.send(Result::Err(AnimError::FailAddingAnimation(gid)))
                            };
                        }
                        Message::StartAnimation(gid, aid) => {
                            screen.start_animation(&gid, aid);
                        }
                        Message::PauseAnimation(gid) => {
                            screen.pause_animation(&gid);
                        }
                        Message::PauseAnimationOnFrame(gid, fid) => {
                            screen.pause_animation_on_frame(&gid, fid);
                        }
                        Message::StopAnimation(gid) => {
                            screen.stop_animation(&gid);
                        }
                        Message::RestartAnimation(aid) => {
                            screen.restart_animation(&aid);
                        }
                        Message::AddGraphic(gid, gr, layer, offset) => {
                            screen.add_graphic(gr, layer, offset);
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
                                result_sender.send(Result::Ok(GlyphRetrieved(gid, glyph)))
                            } else {
                                result_sender.send(Result::Err(AnimError::FailGettingGlyph(gid)))
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
                        Message::EmptyFrame(gid) => {
                            let result = screen.empty_frame(gid);
                            if let Some(id) = result {
                                result_sender.send(Result::Ok(FrameAdded(gid, id)))
                            } else {
                                result_sender.send(Result::Err(AnimError::FailAddingFrame(gid)))
                            };
                        }
                        Message::CloneFrame(gid, fid) => {
                            let result = screen.clone_frame(gid, fid);
                            if let Some(id) = result {
                                result_sender.send(Result::Ok(FrameAdded(gid, id)))
                            } else {
                                result_sender.send(Result::Err(AnimError::FailAddingFrame(gid)))
                            };
                        }
                        Message::NewDisplay(display_id, keep_existing) => {
                            let display_id = screen.new_display(display_id, keep_existing);
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
                //screen.update_animations();
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
            let kb_join_handle = thread::spawn(move || {
                let mut finish = false;
                while !finish {
                    reader.read_exact(&mut buffer);
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
            next_id: 0,
            next_screen_id: 1,
            sender,
            key_receiver,
            result_receiver: Some(result_receiver.into_iter()),
        }
    }

    pub fn get_key_receiver(&mut self) -> Option<mpsc::Receiver<u8>> {
        replace(&mut self.key_receiver, None)
    }

    pub fn read_key(&mut self) -> Option<Key> {
        let mut k_rcvr = replace(&mut self.key_receiver, None);
        if let Some(key_rcvr) = k_rcvr {
            let mut keys_read: Vec<u8> = Vec::with_capacity(10);
            let mut multibyte_sequence = false;
            let mut all_bytes_read = false;
            if let Ok(first_byte) = key_rcvr.recv() {
                keys_read.push(first_byte);
                if first_byte != 27 && first_byte < 128 {
                    all_bytes_read = true
                }
                while !all_bytes_read {
                    match key_rcvr.recv_timeout(Duration::from_nanos(1)) {
                        Ok(byte) => keys_read.push(byte),
                        Err(_error) => {
                            all_bytes_read = true;
                        }
                    }
                }
            }
            replace(&mut self.key_receiver, Some(key_rcvr));
            return map_bytes_to_key(keys_read);
        } else {
            panic!("mgr has no key receiver!")
        }
        None
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
        self.sender.send(Message::AddAnimation(graphic_id, anim));
    }
    pub fn start_animation(&self, graph_id: usize, anim_id: usize) {
        self.sender.send(Message::StartAnimation(graph_id, anim_id));
    }
    pub fn pause_animation(&self, graphic_id: usize) {
        self.sender.send(Message::PauseAnimation(graphic_id));
    }
    pub fn pause_animation_on_frame(&self, graphic_id: usize, frame_id: usize) {
        self.sender
            .send(Message::PauseAnimationOnFrame(graphic_id, frame_id));
    }
    pub fn stop_animation(&self, graph_id: usize) {
        self.sender.send(Message::StopAnimation(graph_id));
    }
    pub fn restart_animation(&self, anim_id: usize) {
        self.sender.send(Message::RestartAnimation(anim_id));
    }

    pub fn move_graphic(&self, gid: usize, layer: usize, offset: (isize, isize)) {
        self.sender.send(Message::MoveGraphic(gid, layer, offset));
    }
    pub fn set_invisible(&self, gid: usize, invisible: bool) {
        self.sender.send(Message::SetInvisible(gid, invisible));
    }
    pub fn set_glyph(&self, gid: usize, glyph: Glyph, col: usize, row: usize) {
        self.sender.send(Message::SetGlyph(gid, glyph, col, row));
    }
    pub fn get_glyph(&self, gid: usize, col: usize, row: usize) {
        self.sender.send(Message::GetGlyph(gid, col, row));
    }
    pub fn empty_frame(&self, gid: usize) {
        self.sender.send(Message::EmptyFrame(gid));
    }
    fn update_animations() {}

    pub fn cls() {}
    pub fn cla() {}
    pub fn new_display(&mut self, keep_existing: bool) -> usize {
        let new_id = self.next_screen_id;
        self.next_screen_id += 1;
        self.sender.send(Message::NewDisplay(new_id, keep_existing));
        new_id
    }
    pub fn restore_display(&mut self, display_id: usize, keep_existing: bool) {
        self.sender
            .send(Message::RestoreDisplay(display_id, keep_existing));
    }

    pub fn add_graphic(&mut self, gr: Graphic, layer: usize, offset: (usize, usize)) -> usize {
        let gid = self.next_id;
        self.next_id += 1;
        self.sender
            .send(Message::AddGraphic(gid, gr, layer, offset));
        gid
    }
    pub fn set_graphic(&self, gid: usize, fid: usize, force: bool) {
        self.sender.send(Message::SetGraphic(gid, fid, force));
    }
    pub fn set_graphic_color(&self, gid: usize, color: Color) {
        self.sender.send(Message::SetGraphicColor(gid, color));
    }
    pub fn set_graphic_background(&self, gid: usize, color: Color) {
        self.sender.send(Message::SetGraphicBackground(gid, color));
    }
    pub fn set_graphic_style(&self, gid: usize, glyph: Glyph) {
        self.sender.send(Message::SetGraphicStyle(gid, glyph));
    }
    pub fn delete_graphic(&self, gid: usize) {
        self.sender.send(Message::DeleteGraphic(gid));
    }

    pub fn terminate(self) {
        self.sender.send(Message::Finish);
        self.join_handle.join();
    }
}
