use crate::helpers::map_bytes_to_private_char;
use crate::macros::MacroSequence;

use super::animation::Animation;
use super::color::Color;
use super::error::AnimError;
use super::glyph::Glyph;
use super::graphic::Graphic;
use super::helpers::map_bytes_to_key;
use super::key::Key;
use super::macros::Macros;
use super::response::AnimOk::{self, *};
use super::screen::Screen;
use super::Timestamp;

use std::cmp::max;
use std::io;
use std::io::Read;
use std::mem::replace;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Manager uses this messages internally to communicate with Screen that is running in a separate thread.
pub enum Message {
    Finish,
    EmptyFrame(usize),
    ClearArea(usize, (usize, usize), (usize, usize)),
    CloneFrame(usize, Option<usize>),
    AddAnimation(usize, Animation),
    StartAnimation(usize, usize),
    EnqueueAnimation(usize, usize, Timestamp),
    PauseAnimation(usize),
    PauseAnimationOnFrame(usize, usize),
    StopAnimation(usize),
    RestartAnimation(usize, usize, Timestamp),
    AddGraphic(Graphic, usize, (isize, isize)),
    SetGlyph(usize, Glyph, usize, usize),
    GetGlyph(usize, usize, usize),
    SetGraphic(usize, usize, bool),
    SetGraphicColor(usize, Color),
    SetGraphicBackground(usize, Color),
    SetGraphicStyle(usize, Glyph),
    SetInvisible(usize, bool),
    SwapFrame(usize, usize, Vec<Glyph>),
    MoveGraphic(usize, usize, (isize, isize)),
    MoveCursor(usize, usize),
    DeleteGraphic(usize),
    NewDisplay(usize, bool),
    RestoreDisplay(usize, bool),
    PrintGraphic(usize, bool),
    PrintScreen,
    PrintScreenSection((usize, usize), usize, usize),
}

/// This object is responsible for orchestrating behavior of all screens and graphical elements defined.
/// It also allows for reading user input as char.
pub struct Manager {
    scrn_size: (usize, usize),
    join_handle: thread::JoinHandle<()>,
    //    next_id: usize,
    next_screen_id: usize,
    sender: mpsc::Sender<Message>,
    key_receiver: Option<mpsc::Receiver<u8>>,
    key_recv_timeout: Duration,
    result_receiver: Option<mpsc::IntoIter<Result<AnimOk, AnimError>>>,
    macros: Macros,
}

impl Manager {
    /// Use this method to create a new instance of Manager.
    /// One can decide should capturing user input from the keyboard be enabled.
    /// macros is used to allow user defining key macros only when capturing keyboard:
    /// First element in macros Vec defines a Key which is used to toggle Macro recording
    /// e.g. macros: Some(vec![(Key::CtrlM,vec![])]) - pressing CtrlM will
    /// toggle Macro recording mode.
    /// Following vector elements can be used to insert pre-defined macros in a form:
    /// (triggering_key, (looped, Vec<(key_n, delay_n)>)).
    /// When user enables macro recording mode it consists of three phases:
    /// 0. user enters Macro recording mode by pressing Key::CtrlM;
    ///   (Optional) if user presses CtrlM again this newly defined macro
    ///    will be defined as a looping macro - once all keys from it's sequence
    ///    have been sent to the user, it starts over again.
    /// 1. pressing first key combination defines triggering key;
    /// 2. pressing second and further key combinations records key, and time duration
    ///    between current key and following one;
    /// 3. user finishes Macro recording mode by pressing CtrlM;
    ///
    ///    To start or stop any macro user has to press it's triggering key,
    ///    Starting a new macro automatically stops previous running macro, if any.
    pub fn new(
        capture_keyboard: bool,
        cols: Option<usize>,
        rows: Option<usize>,
        glyph: Option<Glyph>,
        screen_refresh_timeout: Option<Duration>,
        macros: Option<Vec<(Key, MacroSequence)>>,
    ) -> Self {
        let mut screen = Screen::new(cols, rows, glyph);
        let cols = screen.cols;
        let rows = screen.rows;
        screen.initialize();
        screen.clear_screen();
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
                            } else if result_sender
                                .send(Result::Err(AnimError::FailAddingAnimation(gid)))
                                .is_err()
                            {
                                eprintln!(
                        "\x1b[97;41;5mERR\x1b[m Unable to send FailAddingAnimation message.")
                            }
                            // };
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
                        Message::SwapFrame(gid, fid, new_frame) => {
                            if let Some(old_frame) = screen.swap_frame(gid, fid, new_frame) {
                                let _ =
                                    result_sender.send(Result::Ok(AnimOk::FrameSwapped(old_frame)));
                            } else {
                                let _ = result_sender
                                    .send(Result::Err(AnimError::FailAddingFrame(fid)));
                            }
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
                            } else if result_sender
                                .send(Result::Err(AnimError::FailGettingGlyph(gid)))
                                .is_err()
                            {
                                eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send FailGettingGlyph message")
                            };
                        }
                        Message::SetGraphic(graphic_id, frame_id, force) => {
                            screen.set_graphic(&graphic_id, &frame_id, force);
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
                            } else if result_sender
                                .send(Result::Err(AnimError::FailAddingFrame(gid)))
                                .is_err()
                            {
                                eprintln!(
                                    "\x1b[97;41;5mERR\x1b[m Unable to send FailAddingFrame message"
                                )
                            };
                        }
                        Message::ClearArea(layer, offset, size) => {
                            let start_x: usize = max(0, offset.0) as usize;
                            let start_y: usize = max(0, offset.1) as usize;
                            screen.clear_area(layer, start_x, start_y, size.0, size.1);
                        }
                        Message::CloneFrame(gid, fid) => {
                            let result = screen.clone_frame(gid, fid);
                            if let Some(id) = result {
                                if result_sender.send(Result::Ok(FrameAdded(gid, id))).is_err() {
                                    eprintln!(
                                        "\x1b[97;41;5mERR\x1b[m Unable to send FrameAdded message"
                                    )
                                }
                            } else if result_sender
                                .send(Result::Err(AnimError::FailAddingFrame(gid)))
                                .is_err()
                            {
                                eprintln!(
                                    "\x1b[97;41;5mERR\x1b[m Unable to send FailAddingFrame message"
                                )
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
                        Message::MoveCursor(x, y) => {
                            screen.print_all(vec![(x, y, Glyph::transparent())])
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
                    if buffer[0] > 0 && key_sender.send(buffer[0]).is_err() {
                        finish = true;
                    }
                }
            });
            key_receiver = Some(key_rcver);
        }
        let macros = Macros::new(macros);
        Manager {
            scrn_size: (cols, rows),
            join_handle,
            //next_id: 0,
            next_screen_id: 1,
            sender,
            key_receiver,
            key_recv_timeout: Duration::from_millis(16),
            result_receiver: Some(result_receiver.into_iter()),
            macros,
        }
    }

    /// In case one has his own logic for serving raw user input from
    /// the keyboard, both for specific case or for all cases.
    pub fn get_key_receiver(&mut self) -> Option<mpsc::Receiver<u8>> {
        self.key_receiver.take()
    }

    /// Use this method to restore or provide your own key receiver for Manager to take
    /// responsibility for interpreting raw user input.
    pub fn set_key_receiver(&mut self, receiver: mpsc::Receiver<u8>) -> Option<mpsc::Receiver<u8>> {
        replace(&mut self.key_receiver, Some(receiver))
    }

    /// Modify how long should Manager wait for bytestream coming from keyboard.
    /// In case timeout is too short it might not get entire bytestream for interpretation and provide corrupted output.
    /// In case timeout is too long it might feel unresponsive for the user.
    pub fn set_key_receive_timeout(&mut self, t: Duration) {
        self.key_recv_timeout = t;
    }

    /// Use return value from this method to send Messages from your own codebase.
    pub fn get_message_sender(&mut self) -> mpsc::Sender<Message> {
        self.sender.clone()
    }

    fn read_bytes(&self) -> Option<Vec<u8>> {
        let mut keys_read: Vec<u8> = Vec::with_capacity(10);
        if let Some(key_rcvr) = &self.key_receiver {
            let mut all_bytes_read = false;
            if let Ok(first_byte) = key_rcvr.recv_timeout(self.key_recv_timeout) {
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
            } else {
                return None;
            }
        } else {
            eprintln!("mgr has no key receiver!")
        }
        Some(keys_read)
    }

    /// Use this method to get a Key value of what user pressed on his keyboard.
    pub fn read_key(&mut self) -> Option<Key> {
        if self.macros.running.is_some() && self.macros.recording.is_none() {
            let key_from_macro = self.macros.key_recv.try_recv();
            match key_from_macro {
                Ok(key) => return Some(key),
                Err(mpsc::TryRecvError::Disconnected) => {
                    // println!("disconnected");
                    let (_, key_recv) = std::sync::mpsc::channel();
                    self.macros.key_recv = key_recv;
                    self.macros.running = None;
                }
                Err(_e) => {
                    // println!("Other error: {}", _e);
                }
            }
        } else if self.macros.running.is_some() {
            // Stop a running macro when recording a new one
            self.macros.stop();
        }
        let read_result = self.read_bytes();
        if let Some(keys_read) = read_result {
            if let Some(key) = map_bytes_to_key(keys_read) {
                if self.macros.enabled {
                    if self.macros.is_record_key(&key) || self.macros.recording.is_some() {
                        self.macros.record(&key);
                        return Some(key);
                    } else if !self.macros.run(&key) {
                        return Some(key);
                    }
                } else {
                    return Some(key);
                }
            }
        }
        None
    }

    /// Use this method to get a String of what user has entered up to Enter key.
    pub fn read_line(&mut self) -> String {
        let mut all_bytes: Vec<u8> = Vec::with_capacity(128);
        let mut enter_pressed = false;
        while !enter_pressed {
            if let Some(mut keys_read) = self.read_bytes() {
                if keys_read.len() == 1 && keys_read[0] == 10 {
                    enter_pressed = true;
                } else if !keys_read.is_empty() {
                    all_bytes.append(&mut keys_read);
                }
            }
        }
        String::from_utf8_lossy(&all_bytes).into_owned()
    }

    /// Use this method to get a char of what user has entered on his keyboard.
    pub fn read_char(&mut self) -> Option<char> {
        if let Some(keys_read) = self.read_bytes() {
            if !keys_read.is_empty() {
                let char_str = String::from_utf8_lossy(&keys_read);
                let ch_len = char_str.len();
                if ch_len > 1 {
                    let mut ch_iter = char_str.chars();
                    let first_char = ch_iter.next();
                    if ch_iter.next().is_none() {
                        first_char
                    } else {
                        map_bytes_to_private_char(keys_read)
                    }
                } else {
                    char_str.chars().next()
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Use this method in case you want Manager to take back servicing results on his actions.
    pub fn set_result_iter(
        &mut self,
        receiver: mpsc::IntoIter<Result<AnimOk, AnimError>>,
    ) -> Option<mpsc::IntoIter<Result<AnimOk, AnimError>>> {
        replace(&mut self.result_receiver, Some(receiver))
    }

    /// Use this method to get next available result of Manager's action.
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

    /// Returns width & height of current screen.
    pub fn screen_size(&self) -> (usize, usize) {
        self.scrn_size
    }

    /// Adds a new Animation for a Graphic. Make sure Graphic has all
    /// frames required by the Animation defined.
    pub fn add_animation(&mut self, graphic_id: usize, anim: Animation) {
        if self
            .sender
            .send(Message::AddAnimation(graphic_id, anim))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send AddAnimation message")
        };
    }

    /// Start an animation for a graphic.
    pub fn start_animation(&self, graph_id: usize, anim_id: usize) {
        if self
            .sender
            .send(Message::StartAnimation(graph_id, anim_id))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send StartAnimation message")
        };
    }

    /// Start another animation for given graphic after current one ends.
    pub fn enqueue_animation(&self, graph_id: usize, anim_id: usize, when: Timestamp) {
        if self
            .sender
            .send(Message::EnqueueAnimation(graph_id, anim_id, when))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send EnqueueAnimation message")
        };
    }

    /// Pause a running animation from given graphic.
    pub fn pause_animation(&self, graphic_id: usize) {
        if self
            .sender
            .send(Message::PauseAnimation(graphic_id))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send Pauseanimation message")
        };
    }

    /// Pause a running animation from given graphic when given frame is being displayed.    
    pub fn pause_animation_on_frame(&self, graphic_id: usize, frame_id: usize) {
        if self
            .sender
            .send(Message::PauseAnimationOnFrame(graphic_id, frame_id))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send PauseAnimationOnFrame message")
        };
    }

    /// Stop animation for given graphic
    pub fn stop_animation(&self, graph_id: usize) {
        if self.sender.send(Message::StopAnimation(graph_id)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send StopAnimation message")
        };
    }

    /// Restart animation for given graphic when the right time comes.
    pub fn restart_animation(&self, graphic_id: usize, anim_id: usize, when: Timestamp) {
        if self
            .sender
            .send(Message::RestartAnimation(graphic_id, anim_id, when))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send RestartAnimation message")
        };
    }

    /// Move a graphic left or right on the screen, optionally changing which layer it is placed on.
    pub fn move_graphic(&self, graphic_id: usize, layer: usize, offset: (isize, isize)) {
        if self
            .sender
            .send(Message::MoveGraphic(graphic_id, layer, offset))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send MoveGraphic message")
        };
    }

    /// Clear an area on selected layer
    pub fn clear_area(&self, layer: usize, start: (usize, usize), size: (usize, usize)) {
        if self
            .sender
            .send(Message::ClearArea(layer, start, size))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send ClearArea message")
        };
    }
    /// Make a graphic invisible.
    pub fn set_invisible(&self, gid: usize, invisible: bool) {
        if self
            .sender
            .send(Message::SetInvisible(gid, invisible))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetInvisible message")
        };
    }

    /// Set glyph for given graphic in specified location to provided value.
    pub fn set_glyph(&self, gid: usize, glyph: Glyph, col: usize, row: usize) {
        if self
            .sender
            .send(Message::SetGlyph(gid, glyph, col, row))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGlyph message")
        };
    }

    /// Request Manager to produce what Glyph is currently set for given graphic in specified location.
    /// Use read_result to get that Glyph.
    pub fn get_glyph(&self, gid: usize, col: usize, row: usize) {
        if self.sender.send(Message::GetGlyph(gid, col, row)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send GetGlyph message")
        };
    }

    /// Use this method to load a graphic from plain text file.
    /// Each line should define a frame or an animation like following:
    /// frame 0 frame_0.txf
    /// animation loop run 0:1000 1:1000 2:1000 3:1000 4:1000 5:1000 6:1000 7:1000 8:1000 9:1000
    /// loop and run in animation definitions are optional.
    /// What follows are frame ids in order from left to right with their display duration in ms after colon.
    /// Frames are defined in separate files each. They consist of regular ASCII/UTF-8 characters with optional
    /// ANSII escape sequences that modify color, background or font style.
    /// You can preview a frame_file.txf calling from terminal: less -R frame_file.txf .
    /// You can use accompanying studio terminal application in order to define your own frames.
    pub fn load_graphic_from_file<P>(&self, filename: P) -> Result<AnimOk, AnimError>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        if let Some(graphic) = Graphic::from_file(filename) {
            Ok(AnimOk::GraphicCreated(graphic))
        } else {
            Err(AnimError::UnableToBuildGraphicFromFile)
        }
    }

    /// Add an empty frame to a graphic.
    pub fn empty_frame(&self, gid: usize) {
        if self.sender.send(Message::EmptyFrame(gid)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send EmptyFrame message")
        };
    }

    /// Add a cloned frame to a graphic.
    pub fn clone_frame(&self, graphic_id: usize, frame_id: Option<usize>) {
        if self
            .sender
            .send(Message::CloneFrame(graphic_id, frame_id))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send ClearArea message")
        };
    }

    /// Create a new clean display, optionally keeping current one.
    pub fn move_cursor(&self, x: usize, y: usize) {
        if self.sender.send(Message::MoveCursor(x, y)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send MoveCursor message")
        };
    }
    /// Create a new clean display, optionally keeping current one.
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

    /// Set display to a different one.
    pub fn restore_display(&mut self, display_id: usize, keep_existing: bool) {
        if self
            .sender
            .send(Message::RestoreDisplay(display_id, keep_existing))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send RestoreDisplay message")
        };
    }

    /// Add a graphic to current display.
    pub fn add_graphic(
        &mut self,
        gr: Graphic,
        layer: usize,
        offset: (isize, isize),
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

    /// Set a graphic to display a particular frame.
    pub fn set_graphic(&self, gid: usize, fid: usize, force: bool) {
        if self
            .sender
            .send(Message::SetGraphic(gid, fid, force))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGraphic message")
        };
    }

    /// Set color of all glyphs under current graphic's frame to given value.
    pub fn set_graphic_color(&self, gid: usize, color: Color) {
        if self
            .sender
            .send(Message::SetGraphicColor(gid, color))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGraphicColor message")
        };
    }

    /// Set background color of all glyphs under current graphic's frame to given value.
    pub fn set_graphic_background(&self, gid: usize, color: Color) {
        if self
            .sender
            .send(Message::SetGraphicBackground(gid, color))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGraphicBackground message")
        };
    }

    /// Set style of all glyphs under current graphic's frame to given value.
    pub fn set_graphic_style(&self, gid: usize, glyph: Glyph) {
        if self
            .sender
            .send(Message::SetGraphicStyle(gid, glyph))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SetGraphicStyle message")
        };
    }

    /// Change an existing Frame in Graphic's library
    pub fn swap_frame(
        &mut self,
        g_id: usize,
        f_id: usize,
        new_frame: Vec<Glyph>,
    ) -> Option<Vec<Glyph>> {
        if self
            .sender
            .send(Message::SwapFrame(g_id, f_id, new_frame))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send SwapFrame message")
        };
        let result = self.read_result();
        if let Ok(AnimOk::FrameSwapped(old_frame)) = result {
            return Some(old_frame);
        }
        None
    }
    /// Delete a graphic from current display.
    pub fn delete_graphic(&self, gid: usize) {
        if self.sender.send(Message::DeleteGraphic(gid)).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send DeleteGraphic message")
        };
    }

    /// Request Manager to provide a String of given graphic for manipulation or permanent storage.
    pub fn print_graphic(&self, gid: usize, skip_border: bool) {
        if self
            .sender
            .send(Message::PrintGraphic(gid, skip_border))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send PrintGraphic message")
        };
    }

    /// Request Manager to provide a String of entire screen for manipulation or permanent storage.
    pub fn print_screen(&self) {
        if self.sender.send(Message::PrintScreen).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send PrintScreen message")
        };
    }

    /// Request Manager to provide a String of selected screen section for manipulation or permanent storage.
    pub fn print_screen_section(&self, offset: (usize, usize), cols: usize, rows: usize) {
        if self
            .sender
            .send(Message::PrintScreenSection(offset, cols, rows))
            .is_err()
        {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send PrintScreenSection message")
        };
    }

    /// Restore terminal to regular buffer when application is about to quit.
    pub fn terminate(self) {
        if self.sender.send(Message::Finish).is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Unable to send Finish message")
        };
        if self.join_handle.join().is_err() {
            eprintln!("\x1b[97;41;5mERR\x1b[m Failed to join thread")
        };
    }
}
