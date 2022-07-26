use super::animation::Animation;
use super::pixel::Pixel;
use super::time::Timestamp;
use super::Glyph;
use std::collections::HashMap;
use std::mem::replace;

pub struct Graphic {
    pub rows: usize,
    pub cols: usize,
    pub current_frame: usize,
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
            running_anim: None,
            next_lib_id,
            next_anim_id,
            library,
            animations: a,
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

    pub fn pause_animation_on_frame(&mut self, anim_id: usize, frame: usize) {
        if let Some(animation) = self.animations.get_mut(&anim_id) {
            animation.pause_on_frame(frame);
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
        self.library.get(&self.current_frame).unwrap().clone()
    }

    pub fn set_glyph(&mut self, glyph: Glyph, col: usize, row: usize) {
        let index = self.cols * (row - 1) + col - 1;
        if index < self.rows * self.cols {
            let mut frame = self.library.remove(&self.current_frame).unwrap();
            replace(&mut frame[index], glyph);
            self.library.insert(self.current_frame, frame);
        }
    }

    pub fn set_graphic(&mut self, id: &usize, offset: (usize, usize), force: bool) -> Vec<Pixel> {
        // if empty {
        // let size = self.cols * self.rows;
        // let gplain = Glyph::plain();
        // let mut result = Vec::with_capacity(size);
        // for i in 0..size {
        //     self.glyphs.insert(i, gplain.clone());
        //     result.push(Pixel::new(
        //         1 + offset.0 + (i % self.cols),
        //         1 + offset.1 + (i / self.cols),
        //         true,
        //         gplain.clone(),
        //     ));
        // }
        // return result;
        //}
        let mut changed = Vec::with_capacity(self.cols);
        if let Some(glyphs) = self.library.get(id) {
            for (i, (old_glyph, new_glyph)) in self
                .library
                .get(&self.current_frame)
                .unwrap()
                .iter()
                .zip(glyphs.into_iter())
                .enumerate()
            {
                if force || *new_glyph != *old_glyph {
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
