use super::animation::Animation;
use super::glyph::Glyph;
use super::graphic::Graphic;
use super::helpers::map_bytes_to_key;
use super::key::Key;
use super::response::{AnimOk::*, AnimResult};
use super::screen::Screen;
use std::io;
use std::io::Read;
use std::mem::replace;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Message {
    Finish,
    EmptyFrame(usize),
    AddAnimation(usize, Animation, usize, (usize, usize)),
    StartAnimation(usize),
    NewStartAnimation(usize, usize),
    PauseAnimation(usize),
    PauseAnimationOnFrame(usize, usize, usize),
    StopAnimation(usize),
    NewStopAnimation(usize),
    RestartAnimation(usize),
    AddGraphic(usize, Graphic, usize, (usize, usize)),
    SetGlyph(usize, Glyph, usize, usize),
    SetGraphic(usize, usize, bool),
    MoveGraphic(usize, usize, (isize, isize)),
    DeleteGraphic(usize),
}

pub struct Manager {
    scrn_size: (usize, usize),
    join_handle: thread::JoinHandle<()>,
    next_id: usize,
    sender: mpsc::Sender<Message>,
    key_receiver: Option<mpsc::Receiver<u8>>,
    result_receiver: Option<mpsc::IntoIter<AnimResult>>,
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
                        Message::AddAnimation(aid, anim, layer, offset) => {
                            let id = screen.add_animation(anim, layer, offset);
                            result_sender.send(AnimResult::Ok(AnimationAdded(id)));
                        }
                        Message::StartAnimation(aid) => {
                            screen.start_animation(&aid);
                        }
                        Message::NewStartAnimation(gid, aid) => {
                            screen.new_start_animation(&gid, aid);
                        }
                        Message::PauseAnimation(aid) => {
                            screen.pause_animation(&aid);
                        }
                        Message::PauseAnimationOnFrame(gid, aid, fid) => {
                            screen.pause_animation_on_frame(&gid, aid, fid);
                        }
                        Message::StopAnimation(aid) => {
                            screen.stop_animation(&aid);
                        }
                        Message::NewStopAnimation(gid) => {
                            screen.new_stop_animation(&gid);
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
                        Message::SetGraphic(gid, fid, force) => {
                            screen.set_graphic((&gid, &fid), force);
                        }
                        Message::DeleteGraphic(gid) => {
                            screen.delete_graphic(&gid);
                        }
                        Message::EmptyFrame(gid) => screen.empty_frame(gid),
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
            // kb_join_handle,
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
            // return Some(Key::a);
        } else {
            panic!("mgr has no key receiver!")
        }
        None
    }

    pub fn set_key_iter(&mut self, receiver: mpsc::Receiver<u8>) -> Option<mpsc::Receiver<u8>> {
        replace(&mut self.key_receiver, Some(receiver))
    }

    pub fn get_result_iter(&mut self) -> Option<mpsc::IntoIter<AnimResult>> {
        replace(&mut self.result_receiver, None)
    }

    pub fn set_result_iter(
        &mut self,
        receiver: mpsc::IntoIter<AnimResult>,
    ) -> Option<mpsc::IntoIter<AnimResult>> {
        replace(&mut self.result_receiver, Some(receiver))
    }

    pub fn screen_size(&self) -> (usize, usize) {
        self.scrn_size
    }

    // pub fn add_animation(
    //     &mut self,
    //     anim: Animation,
    //     layer: usize,
    //     offset: (usize, usize),
    // ) -> usize {
    //     let anim_id = self.next_id;
    //     self.next_id += 1;
    //     self.sender
    //         .send(Message::AddAnimation(anim_id, anim, layer, offset));
    //     anim_id
    // }
    // pub fn start_animation(&self, anim_id: usize) {
    //     self.sender.send(Message::StartAnimation(anim_id));
    // }
    pub fn new_start_animation(&self, graph_id: usize, anim_id: usize) {
        self.sender
            .send(Message::NewStartAnimation(graph_id, anim_id));
    }
    pub fn pause_animation(&self, anim_id: usize) {
        self.sender.send(Message::PauseAnimation(anim_id));
    }
    pub fn pause_animation_on_frame(&self, graphic_id: usize, anim_id: usize, frame_id: usize) {
        self.sender.send(Message::PauseAnimationOnFrame(
            graphic_id, anim_id, frame_id,
        ));
    }
    pub fn stop_animation(&self, anim_id: usize) {
        self.sender.send(Message::StopAnimation(anim_id));
    }
    pub fn new_stop_animation(&self, graph_id: usize) {
        self.sender.send(Message::NewStopAnimation(graph_id));
    }
    pub fn restart_animation(&self, anim_id: usize) {
        self.sender.send(Message::RestartAnimation(anim_id));
    }

    pub fn move_graphic(&self, gid: usize, layer: usize, offset: (isize, isize)) {
        self.sender.send(Message::MoveGraphic(gid, layer, offset));
    }
    pub fn set_glyph(&self, gid: usize, glyph: Glyph, col: usize, row: usize) {
        self.sender.send(Message::SetGlyph(gid, glyph, col, row));
    }
    pub fn empty_frame(&self, gid: usize) {
        self.sender.send(Message::EmptyFrame(gid));
    }
    fn update_animations() {}

    pub fn cls() {}
    pub fn cla() {}
    pub fn new_display(keep_existing: bool) {}
    pub fn restore_display(display_id: usize, keep_existing: bool) {}

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
    pub fn delete_graphic(&self, gid: usize) {
        self.sender.send(Message::DeleteGraphic(gid));
    }

    pub fn terminate(self) {
        self.sender.send(Message::Finish);
        self.join_handle.join();
    }
}
