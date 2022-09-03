use super::time::Timestamp;

#[derive(Debug)]
pub struct Animation {
    current_frame: usize,
    stop_frame: Option<usize>,
    next_frame: usize,
    pub running: bool,
    looping: bool,
    //frames: HashMap<usize, Vec<Pixel>>,
    ordering: Vec<(usize, Timestamp)>,
    ord_max: usize,
    trigger_time: Timestamp,
}

impl Animation {
    pub fn new(
        running: bool,
        looping: bool,
        ordering: Vec<(usize, Timestamp)>,
        start_time: Timestamp,
    ) -> Animation {
        let ord_max = ordering.len() - 1;
        let mut stop_frame = None;
        if !looping {
            let last_frame = ordering.last().unwrap().0;
            stop_frame = Some(last_frame);
        }
        Animation {
            current_frame: 0,
            stop_frame,
            next_frame: 0,
            running,
            looping,
            //frames,
            ordering,
            ord_max,
            trigger_time: start_time,
        }
    }

    pub fn start(&mut self, t: Timestamp) {
        if !self.running {
            self.trigger_time = t;
            self.running = true;
        }
    }

    pub fn restart(&mut self, t: Timestamp) {
        self.trigger_time = t; // + self.trigger_time;
        self.current_frame = 0;
        self.next_frame = 0;
        self.running = true;
    }

    pub fn pause(&mut self, t: Timestamp) {
        self.trigger_time = self.trigger_time - t;
        self.running = false;
    }

    pub fn freeze(&mut self, t: Timestamp) {
        self.trigger_time = self.trigger_time - t;
    }

    pub fn pause_on_frame(&mut self, frame_id: usize) {
        self.stop_frame = Some(frame_id);
    }

    pub fn stop(&mut self) {
        self.trigger_time = Timestamp::now();
        self.current_frame = 0;
        self.next_frame = 0;
        self.running = false;
    }

    pub fn new_update(&mut self, dtime: Timestamp) -> Option<(usize, bool)> {
        let mut frame = None;
        if self.running {
            if let Some(stop_frame) = self.stop_frame {
                if stop_frame == self.current_frame {
                    self.running = false;
                    self.stop_frame = None;
                    frame = Some(self.current_frame);
                }
            }
            if dtime >= self.trigger_time {
                let (current_frame, delta_time) = self.ordering[self.next_frame];
                // println!(
                //     "Next frame: {}, current frame: {}",
                //     self.next_frame, current_frame
                // );
                self.current_frame = current_frame;
                frame = Some(self.current_frame);
                self.trigger_time += delta_time;
                self.next_frame += 1;
                if self.next_frame > self.ord_max {
                    self.next_frame = 0;
                    if !self.looping {
                        self.running = false;
                        //self.trigger_time = Timestamp::now();
                    }
                }
            }
        }
        if let Some(fr) = frame {
            return Some((fr, self.running));
        }
        None
    }

    // pub fn update(&mut self, dtime: Timestamp) -> Option<Vec<Pixel>> {
    //     if !self.running || dtime < self.trigger_time {
    //         return None;
    //     } else {
    //         let frame = self.frames.get(&self.current_frame).unwrap();
    //         let (current_frame, delta_time) = self.ordering[self.next_frame];
    //         self.current_frame = current_frame;
    //         self.trigger_time += delta_time;
    //         self.next_frame += 1;
    //         if self.next_frame > self.ord_max {
    //             self.next_frame = 0;
    //             if !self.looping {
    //                 self.running = false;
    //                 self.trigger_time = Timestamp::now();
    //             }
    //         }
    //         Some(frame.to_vec().clone())
    //     }
    // }
}
