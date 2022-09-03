use super::animation::Animation;
use super::color::Color;
use super::error::AnimError;
use super::frame::from_file as frame_from_file;
use super::pixel::Pixel;
use super::time::Timestamp;
use super::utilities::text_to_frame;
use super::Glyph;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;
use std::mem::replace;

#[derive(Debug)]
pub struct Graphic {
    pub rows: usize,
    pub cols: usize,
    pub current_frame: usize,
    pub invisible: bool,
    pub running_anim: Option<usize>,
    pub awaiting_anim: Option<(usize, Timestamp)>,
    next_lib_id: usize,
    next_anim_id: usize,
    library: HashMap<usize, Vec<Glyph>>,
    pub animations: HashMap<usize, Animation>,
}

impl Graphic {
    pub fn new(
        cols: usize,
        rows: usize,
        start_frame: usize,
        library: HashMap<usize, Vec<Glyph>>,
        animations: Option<HashMap<usize, Animation>>,
    ) -> Self {
        let next_lib_id = library.keys().max().unwrap() + 1;
        let mut next_anim_id = 0;
        let a = if animations.is_some() {
            let anim = animations.unwrap();
            next_anim_id = anim.keys().max().unwrap() + 1;
            anim
        } else {
            HashMap::new()
        };

        Graphic {
            rows,
            cols,
            current_frame: start_frame,
            invisible: false,
            running_anim: None,
            awaiting_anim: None,
            next_lib_id,
            next_anim_id,
            library,
            animations: a,
        }
    }

    pub fn from_file<P>(filename: P) -> Option<Self>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        let mut create_graphic = false;
        let mut running_anim = None;
        let mut invisible = false;
        let mut current_frame = 0;
        let mut rows = 0;
        let mut cols = 0;
        let hash = '#';
        let colon = ':';
        let mut library: HashMap<usize, Vec<Glyph>> = HashMap::new();
        let mut next_lib_id = 0;
        let mut next_anim_id = 0;

        let mut animations: HashMap<usize, Animation> = HashMap::new();
        let mut names_mapping: HashMap<String, usize> = HashMap::new();

        let mut read_lines = vec![];
        let base_path = Path::new(filename.as_ref().parent().to_owned().unwrap());

        if let Ok(file) = File::open(&filename) {
            for line in io::BufReader::new(file).lines() {
                read_lines.push(line);
            }
        }
        for line in read_lines {
            if let Ok(line) = line {
                if line.trim().starts_with(hash) {
                    eprintln!("hashowa");
                    continue;
                }
                if line.is_empty() {
                    eprintln!("pusta");
                    continue;
                }
                let tokens: Vec<&str> = line.split_whitespace().collect();
                match tokens[0] {
                    "invisible" => {
                        invisible = true;
                    }
                    "frame" => {
                        if tokens.len() > 2 {
                            let frame_name = tokens[1];
                            let frame_file = Path::new(tokens[2]);
                            let frame_result = if frame_file.is_absolute() {
                                frame_from_file(&frame_file)
                            } else {
                                frame_from_file(&base_path.join(frame_file))
                            };
                            if let Some((cs, frame)) = frame_result {
                                if cols > 0 {
                                    if cols != cs {
                                        eprintln!("Unable to add frame that has cols: {}, when expected is {}", cs,cols);
                                        continue;
                                    }
                                } else {
                                    cols = cs;
                                    rows = frame.len() / cols;
                                }

                                names_mapping.insert(frame_name.to_owned(), next_lib_id);
                                library.insert(next_lib_id, frame);
                                create_graphic = true;
                                next_lib_id += 1;
                            }
                        } else {
                            eprintln!("Incorrect line(should be 'frame name filepath #maybe comment'): {} while building Graphic from file", line);
                        }
                    }
                    "animation" => {
                        let mut looping = false;
                        let mut running = false;
                        let start_time = Timestamp::now();
                        let mut ordering: Vec<(usize, Timestamp)> = Vec::new();
                        if tokens.len() > 2 {
                            for t in &tokens[1..] {
                                match t {
                                    &"loop" => {
                                        looping = true;
                                    }
                                    &"run" => {
                                        if running_anim.is_none() {
                                            running = true;
                                        }
                                    }
                                    _ => {
                                        if t.contains(hash) {
                                            break;
                                        }
                                        if t.contains(colon) {
                                            let frame_time: Vec<&str> = t.split(colon).collect();
                                            if frame_time.len() != 2 {
                                                eprint!("Unable to read animation definition from file, {} should be frame_id:time_ms  ",t);
                                            } else {
                                                if let Some(frame_id) =
                                                    names_mapping.get(frame_time[0])
                                                {
                                                    if let Ok(msec) =
                                                        u32::from_str_radix(frame_time[1], 10)
                                                    {
                                                        ordering.push((
                                                            *frame_id,
                                                            Timestamp::new(0, msec),
                                                        ))
                                                    } else {
                                                        eprint!(
                                                        "Unable to read integer from {} (in {}) ",
                                                        frame_time[1], t
                                                    );
                                                    }
                                                } else {
                                                    eprint!(
                                                        "Unable to find frame with id {} ",
                                                        frame_time[0]
                                                    );
                                                }
                                            }
                                        } else {
                                            eprint!("Unable to read animation definition from file, {} is missing ':' ",t);
                                        }
                                    }
                                }
                            }
                            if running {
                                current_frame = ordering.last().unwrap().0;
                                running_anim = Some(next_anim_id);
                            }
                            let a = Animation::new(running, looping, ordering, start_time);
                            animations.insert(next_anim_id, a);
                            next_anim_id += 1;
                        } else {
                            eprintln!("Incorrect line(should be 'animation [loop] [run] {{frame_name:duration=}}+ #maybe comment'): {} while building Graphic from file", line);
                        }
                    }
                    &_ => {}
                }
            }
        }
        if create_graphic {
            return Some(Graphic {
                rows,
                cols,
                current_frame,
                invisible,
                running_anim,
                awaiting_anim: None,
                next_lib_id,
                next_anim_id,
                library,
                animations,
            });
        }
        None
    }

    pub fn from_frame(cols: usize, frame: Vec<Glyph>) -> Self {
        let mut library = HashMap::with_capacity(1);
        let rows = frame.len() / cols;
        library.insert(0, frame);

        Graphic {
            rows,
            cols,
            current_frame: 0,
            invisible: false,
            running_anim: None,
            awaiting_anim: None,
            next_lib_id: 1,
            next_anim_id: 0,
            library,
            animations: HashMap::new(),
        }
    }

    pub fn from_text(cols: usize, text: &str, glyph: Glyph) -> Self {
        let mut library = HashMap::with_capacity(1);
        // TODO fix this
        let rows = text.len() / cols;
        library.insert(0, text_to_frame(text, glyph));
        Graphic {
            rows,
            cols: cols,
            current_frame: 0,
            invisible: false,
            running_anim: None,
            awaiting_anim: None,
            next_lib_id: 1,
            next_anim_id: 0,
            library: library,
            animations: HashMap::new(),
        }
    }

    pub fn from_texts(cols: usize, texts: Vec<(&str, Glyph)>) -> Self {
        let mut library = HashMap::with_capacity(1);
        for (i, (text, glyph)) in texts.iter().enumerate() {
            library.insert(i, text_to_frame(text, *glyph));
        }
        Graphic {
            rows: 1,
            cols: cols,
            current_frame: 0,
            invisible: false,
            running_anim: None,
            awaiting_anim: None,
            next_lib_id: 1,
            next_anim_id: 0,
            library: library,
            animations: HashMap::new(),
        }
    }
    pub fn add_to_library(&mut self, item: Vec<Glyph>) -> Option<usize> {
        let mut result = None;
        if item.len() == self.rows * self.cols {
            self.library.insert(self.next_lib_id, item);
            result = Some(self.next_lib_id);
            self.next_lib_id += 1;
        }
        result
    }

    pub fn set_invisible(&mut self, invisible: bool, offset: (usize, usize)) -> Vec<Pixel> {
        if invisible == self.invisible {
            return Vec::new();
        }
        let mut changed = Vec::with_capacity(self.rows * self.rows);
        self.invisible = invisible;
        if self.invisible {
            let transparent = Glyph::transparent();
            for c in offset.0..offset.0 + self.cols {
                for r in offset.1..offset.1 + self.rows {
                    changed.push(Pixel::new(c, r, transparent));
                }
            }
        } else {
            changed = self.get(offset);
        }
        // if changed.len() != self.rows * self.cols {
        //     println!(
        //         "{} coś tu nie gra {} a powinno być {}( {}x{})",
        //         invisible,
        //         changed.len(),
        //         self.cols * self.rows,
        //         self.cols,
        //         self.rows
        //     );
        // }
        changed
    }

    pub fn empty_frame(&mut self) -> Option<usize> {
        self.add_to_library(vec![Glyph::plain(); self.rows * self.cols])
    }

    pub fn clone_frame(&mut self, frame_id: usize) -> Option<usize> {
        let mut result = None;
        if let Some(frame) = self.library.get(&frame_id) {
            result = self.add_to_library(frame.to_owned());
        }
        result
    }

    pub fn add_animation(&mut self, anim: Animation) -> Option<usize> {
        self.animations.insert(self.next_anim_id, anim);
        let result = Some(self.next_anim_id);
        self.next_anim_id += 1;
        result
    }

    pub fn start_animation(&mut self, anim_id: usize, when: Timestamp) {
        if let Some(anim_id) = self.running_anim {
            let old_animation = self.animations.get_mut(&anim_id).unwrap();
            old_animation.stop();
        }
        if let Some(animation) = self.animations.get_mut(&anim_id) {
            animation.start(when);
            self.running_anim = Some(anim_id);
        }
    }

    pub fn stop_animation(&mut self) {
        if let Some(anim_id) = self.running_anim {
            let animation = self.animations.get_mut(&anim_id).unwrap();
            animation.stop();
            self.running_anim = None;
        }
    }

    pub fn pause_animation(&mut self, anim_id: usize, when: Timestamp) {
        if let Some(animation) = self.animations.get_mut(&anim_id) {
            animation.pause(when);
            self.running_anim = None;
        }
    }

    pub fn pause_animation_on_frame(&mut self, anim_id: usize, frame_id: usize) {
        if let Some(animation) = self.animations.get_mut(&anim_id) {
            animation.pause_on_frame(frame_id);
            //self.running_anim = None;
        }
    }

    pub fn restart_animation(&mut self, anim_id: usize, when: Timestamp) {
        if let Some(animation) = self.animations.get_mut(&anim_id) {
            animation.restart(when);
            self.running_anim = Some(anim_id);
        }
    }

    pub fn enqueue_animation(&mut self, anim_id: usize, when: Timestamp) {
        if self.animations.contains_key(&anim_id) {
            if let Some(running) = self.running_anim {
                if anim_id != running {
                    self.awaiting_anim = Some((anim_id, when));
                }
            } else {
                self.start_animation(anim_id, when);
            }
        }
    }

    pub fn get(&self, offset: (usize, usize)) -> Vec<Pixel> {
        let mut result = Vec::with_capacity(self.rows * self.cols);
        for (i, glyph) in self.current_frame().iter().cloned().enumerate() {
            result.push(Pixel::new(
                offset.0 + (i % self.cols),
                offset.1 + (i / self.cols),
                glyph,
            ));
        }
        result
    }

    pub fn current_frame(&self) -> Vec<Glyph> {
        if self.invisible {
            vec![Glyph::transparent(); self.cols * self.rows]
        } else {
            let wframe = self.library.get(&self.current_frame);
            if let Some(frame) = wframe {
                return frame.clone();
            } else {
                panic!(
                    "Unable to retrieve frame {}, available: {:?} (c: {}, r: {})",
                    self.current_frame,
                    self.library.keys().len(),
                    self.cols,
                    self.rows
                );
            }
        }
    }

    pub fn get_frame(&self, frame_id: usize) -> Result<Vec<Glyph>, AnimError> {
        if self.invisible {
            Ok(vec![Glyph::transparent(); self.cols * self.rows])
        } else {
            if let Some(frame) = self.library.get(&frame_id) {
                return Ok(frame.clone());
            } else {
                return Err(AnimError::FrameNotFound);
            }
        }
    }

    pub fn set_glyph(
        &mut self,
        glyph: Glyph,
        col: usize,
        row: usize,
        offset: (usize, usize),
    ) -> Vec<Pixel> {
        let mut changed = Vec::with_capacity(1);
        let index = self.cols * (row) + col;
        if index < self.rows * self.cols {
            let mut frame = self.library.remove(&self.current_frame).unwrap();
            let _r = replace(&mut frame[index], glyph);
            self.library.insert(self.current_frame, frame);
            changed.push(Pixel::new(col + offset.0, row + offset.1, glyph));
        }
        changed
    }

    pub fn get_glyph(&self, col: usize, row: usize) -> Option<Glyph> {
        let index = self.cols * (row) + col;
        if index < self.rows * self.cols {
            let frame = self.current_frame();
            return frame.get(index).cloned();
        }
        None
    }

    pub fn set_current_frame_color(&mut self, color: Color) {
        let mut frame = self.library.remove(&self.current_frame).unwrap();
        for g in frame.iter_mut() {
            g.set_color(color);
        }
        self.library.insert(self.current_frame, frame);
    }

    pub fn set_current_frame_background(&mut self, color: Color) {
        let mut frame = self.library.remove(&self.current_frame).unwrap();
        for g in frame.iter_mut() {
            g.set_background(color);
        }
        self.library.insert(self.current_frame, frame);
    }

    pub fn set_current_frame_style(&mut self, mut style: Glyph) {
        let mut new_frame = Vec::with_capacity(self.cols * self.rows);
        let mut frame = self.library.remove(&self.current_frame).unwrap();
        for g in frame.iter_mut() {
            style.set_char(g.character);
            style.set_color(g.color);
            style.set_background(g.background);
            new_frame.push(style)
        }
        self.library.insert(self.current_frame, new_frame);
    }

    pub fn set_frame(&mut self, id: &usize, offset: (usize, usize), force: bool) -> Vec<Pixel> {
        let mut changed = Vec::with_capacity(self.cols);
        if let Ok(glyphs) = self.get_frame(*id) {
            //let glyphs = self.get_frame(*id);
            for (i, (old_glyph, new_glyph)) in self
                .current_frame()
                .iter()
                .zip(glyphs.into_iter())
                .enumerate()
            {
                if force || new_glyph != *old_glyph {
                    changed.push(Pixel::new(
                        offset.0 + (i % self.cols),
                        offset.1 + (i / self.cols),
                        new_glyph.clone(),
                    ));
                }
            }
            self.current_frame = *id;
        }
        changed
    }
}
