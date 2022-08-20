use super::animation::Animation;
use super::color::Color;
use super::error::AnimError;
use super::pixel::Pixel;
use super::time::Timestamp;
use super::utilities::text_to_frame;
use super::Glyph;
use std::fs::File;
use std::io::{BufReader, Read};
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
            next_lib_id,
            next_anim_id,
            library,
            animations: a,
        }
    }

    pub fn from_file<P>(filename: P) -> Option<Self>
    where
        P: AsRef<Path>,
    {
        let mut result = None;
        if let Ok(file) = File::open(filename) {
            let mut read_string = String::with_capacity(1024);
            let mut br = BufReader::new(file);
            if br.read_to_string(&mut read_string).is_ok() {
                let mut cs = 0;
                let mut rs = 0;
                let mut glyph = Glyph::default();
                let mut frame = Vec::new();
                for line in read_string.lines() {
                    rs += 1;
                    let mut style_started = false;
                    let mut style_definition = String::new();
                    for char in line.chars() {
                        match char {
                            '\x1b' => {
                                if style_definition.len() > 0 {
                                    glyph.update_from_str(&style_definition);
                                    style_definition.clear();
                                }
                                style_started = true;
                                style_definition.push(char);
                            }
                            'm' => {
                                style_definition.push(char);
                                if style_started {
                                    style_started = false;
                                } else {
                                    glyph.update_from_str(&style_definition);
                                    frame.push(glyph);
                                    style_definition.clear();
                                    cs += 1;
                                }
                            }
                            '\n' => {
                                continue;
                            }
                            _ => {
                                style_definition.push(char);
                                if !style_started {
                                    glyph.update_from_str(&style_definition);
                                    frame.push(glyph);
                                    style_definition.clear();
                                    cs += 1;
                                }
                            }
                        }
                    }
                }
                // result = Some(Graphic::from_text(
                //     10,
                //     "dupadupadupadupadupadupadupadupadupadupa",
                //     Glyph::default(),
                // ));
                cs = cs / rs;
                if frame.len() > 0 {
                    let mut lib = HashMap::with_capacity(1);
                    lib.insert(0, frame);
                    result = Some(Graphic::new(cs, rs, 0, lib, None));
                } else {
                    panic!("Frame empty!")
                }
            } else {
                panic!("Unable to read file!")
            }
        }
        result
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

    pub fn set_invisible(&mut self, invisible: bool) {
        self.invisible = invisible;
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
        let mut result = None;
        self.animations.insert(self.next_anim_id, anim);
        result = Some(self.next_anim_id);
        self.next_anim_id += 1;
        result
    }

    pub fn start_animation(&mut self, anim_id: usize, when: Timestamp) {
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

    pub fn get(&self, offset: (usize, usize)) -> Vec<Pixel> {
        let mut result = Vec::with_capacity(self.rows * self.cols);
        for (i, glyph) in self
            .library
            .get(&self.current_frame)
            .unwrap()
            .iter()
            .cloned()
            .enumerate()
        {
            result.push(Pixel::new(
                1 + offset.0 + (i % self.cols),
                1 + offset.1 + (i / self.cols),
                true,
                glyph,
            ));
        }
        result
    }

    pub fn current_frame(&self) -> Vec<Glyph> {
        if self.invisible {
            vec![Glyph::transparent(); self.cols * self.rows]
        } else {
            self.library.get(&self.current_frame).unwrap().clone()
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

    pub fn set_glyph(&mut self, glyph: Glyph, col: usize, row: usize) {
        let index = self.cols * (row - 1) + col - 1;
        if index < self.rows * self.cols {
            let mut frame = self.library.remove(&self.current_frame).unwrap();
            let _r = replace(&mut frame[index], glyph);
            self.library.insert(self.current_frame, frame);
        }
    }

    pub fn get_glyph(&self, col: usize, row: usize) -> Option<Glyph> {
        let index = self.cols * (row - 1) + col - 1;
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
                        1 + offset.0 + (i % self.cols),
                        1 + offset.1 + (i / self.cols),
                        true,
                        new_glyph.clone(),
                    ));
                }
            }
            self.current_frame = *id;
        }
        changed
    }
}
