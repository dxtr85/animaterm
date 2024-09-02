use crate::helpers::map_bytes_to_key;

use super::key::Key;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct MacroSequence(bool, Vec<(Key, Duration)>);
impl MacroSequence {
    pub fn empty() -> Self {
        MacroSequence(false, vec![])
    }
    pub fn new(looped: bool, sequence: Vec<(Key, Duration)>) -> Self {
        MacroSequence(looped, sequence)
    }
    pub fn from_text(text: String, delay: Duration, looped: bool) -> Self {
        let mut sequence = Vec::with_capacity(text.len());
        for ch in text.chars() {
            if let Some(key) = map_bytes_to_key(vec![ch as u8]) {
                sequence.push((key, delay))
            }
        }
        MacroSequence(looped, sequence)
    }
}

pub struct Macros {
    pub enabled: bool,
    record_key: Option<Key>,
    pub running: Option<Key>,
    pub recording: Option<(Option<Key>, MacroSequence, Instant)>,
    macros: HashMap<Key, MacroSequence>,
    pub key_recv: Receiver<Key>,
    terminate_send: Sender<()>,
}

impl Macros {
    pub fn new(macros: Option<Vec<(Key, MacroSequence)>>) -> Self {
        let (_key_send, key_recv) = channel();
        let (terminate_send, _recv) = channel();
        if let Some(macros_vec) = macros {
            let mut macros: HashMap<Key, MacroSequence> = HashMap::with_capacity(macros_vec.len());
            let mut macros_iter = macros_vec.into_iter();
            let (macro_key, MacroSequence(looped, key_sequence)) = macros_iter.next().unwrap();
            let record_key = if key_sequence.is_empty() {
                Some(macro_key)
            } else {
                macros.insert(macro_key, MacroSequence::new(looped, key_sequence));
                None
            };
            for (macro_trigger, MacroSequence(looped, sequence)) in macros_iter {
                macros.insert(macro_trigger, MacroSequence::new(looped, sequence));
            }
            Macros {
                enabled: true,
                record_key,
                running: None,
                recording: None,
                macros,
                // key_send,
                key_recv,
                terminate_send,
                // terminate_recv,
            }
        } else {
            Macros {
                enabled: false,
                record_key: None,
                running: None,
                recording: None,
                macros: HashMap::new(),
                key_recv,
                terminate_send,
            }
        }
    }

    pub fn record(&mut self, key: &Key) {
        if self.recording.is_none() {
            if self.is_record_key(key) {
                self.recording = Some((None, MacroSequence::empty(), Instant::now()))
            } else {
                println!("Unexpected key upon recording start: {}", key);
            }
        } else if let Some((rec_key, MacroSequence(looped, mut sequence), timestamp)) =
            self.recording.take()
        {
            if self.is_record_key(key) {
                if rec_key.is_none() {
                    self.recording = Some((
                        rec_key,
                        MacroSequence::new(!looped, sequence),
                        Instant::now(),
                    ));
                } else {
                    self.macros
                        .insert(rec_key.unwrap(), MacroSequence::new(looped, sequence));
                    self.recording = None;
                }
            } else if rec_key.is_none() {
                self.recording = Some((
                    Some(key.clone()),
                    MacroSequence::new(looped, sequence),
                    Instant::now(),
                ));
            } else {
                sequence.push((key.clone(), timestamp.elapsed()));
                self.recording = Some((
                    rec_key,
                    MacroSequence::new(looped, sequence),
                    Instant::now(),
                ));
            }
        }
    }

    pub fn run(&mut self, key: &Key) -> bool {
        if self.macros.contains_key(key) {
            match self.running {
                None => {}
                Some(ref running_key) => {
                    if running_key == key {
                        self.stop();
                        return true;
                    }
                }
            }
            self.stop();
            self.running = Some(key.clone());
            let (key_send, key_recv) = channel();
            self.key_recv = key_recv;
            let MacroSequence(looped, sequence) = self.macros.get(key).cloned().unwrap();
            let (terminate_send, terminate_recv) = channel();
            self.terminate_send = terminate_send;
            thread::spawn(move || {
                if looped {
                    'external: loop {
                        for (key, sleep_time) in sequence.iter() {
                            let terminate_result = terminate_recv.try_recv();
                            match terminate_result {
                                Ok(()) => break 'external,
                                Err(std::sync::mpsc::TryRecvError::Disconnected) => break 'external,
                                Err(_) => {}
                            }
                            if key_send.send(key.clone()).is_err() {
                                break;
                            }
                            thread::sleep(*sleep_time);
                        }
                    }
                } else {
                    for (key, sleep_time) in sequence.iter() {
                        let terminate_result = terminate_recv.try_recv();
                        match terminate_result {
                            Ok(()) => break,
                            Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
                            Err(_) => {}
                        }
                        if key_send.send(key.clone()).is_err() {
                            break;
                        }
                        thread::sleep(*sleep_time);
                    }
                    drop(terminate_recv);
                }
            });
            true
        } else {
            false
        }
    }
    pub fn stop(&mut self) {
        let _ = self.terminate_send.send(());
        self.running = None;
    }
    pub fn is_record_key(&self, key: &Key) -> bool {
        match &self.record_key {
            None => false,
            Some(r_key) => r_key == key,
        }
    }
}
