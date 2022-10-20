use super::time::Timestamp;

/// This structure contains information about Animation progress with time.
/// It does not contain any frames, making it possible to reuse those frames
/// in multiple animations belonging to the same graphic.
/// Also the same animation structure can be reused by multiple graphics
/// with different frames making them appear as if they are running in sync.
#[derive(Debug)]
pub struct Animation {
    current_frame: usize,
    stop_frame: Option<usize>,
    next_frame: usize,
    pub running: bool,
    looping: bool,
    ordering: Vec<(usize, Timestamp)>,
    ord_max: usize,
    trigger_time: Timestamp,
}

impl Animation {
    /// This method creates a new Animation instance. One can decide whether it should
    /// be running immediately or only after defined moment in time. One can also decide
    /// if it should run only once, or start over again and again.
    pub fn new(
        running: bool,
        looping: bool,
        ordering: Vec<(usize, Timestamp)>,
        start_time: Timestamp,
    ) -> Animation {
        if ordering.is_empty() {
            Animation {
                current_frame: 0,
                stop_frame: None,
                next_frame: 0,
                running,
                looping,
                //frames,
                ordering,
                ord_max: 0,
                trigger_time: start_time,
            }
        } else {
            let ord_max = ordering.len() - 1;
            let next_frame = 1;
            let mut stop_frame = None;
            if !looping {
                let last_frame = ordering.last().unwrap().0;
                stop_frame = Some(last_frame);
            }
            Animation {
                current_frame: 0,
                stop_frame,
                next_frame,
                running,
                looping,
                //frames,
                ordering,
                ord_max,
                trigger_time: start_time,
            }
        }
    }

    /// This method is used to start an Animation if it is not already running.
    pub fn start(&mut self, t: Timestamp) {
        if !self.running {
            self.trigger_time = t;
            self.running = true;
        }
    }

    /// Use this method to start an Animation from beginning frame.
    pub fn restart(&mut self, t: Timestamp) {
        self.trigger_time = t; // + self.trigger_time;
        self.current_frame = 0;
        if self.ord_max > 0 {
            self.next_frame = self.ordering.first().unwrap().0;
        } else {
            self.next_frame = 0;
        }
        self.running = true;
    }

    /// In order to pause an Animation, call this method.
    pub fn pause(&mut self, t: Timestamp) {
        self.trigger_time = self.trigger_time - t;
        self.running = false;
    }

    /// Prevent an Animation from switching to the next frame for given amount of time.
    pub fn freeze(&mut self, t: Timestamp) {
        self.trigger_time = self.trigger_time - t;
    }

    /// Pause an Animation when given frame is being displayed.
    pub fn pause_on_frame(&mut self, frame_id: usize) {
        self.stop_frame = Some(frame_id);
    }

    /// Stop an Animation.
    pub fn stop(&mut self) {
        self.trigger_time = Timestamp::now();
        self.current_frame = 0;
        self.next_frame = 0;
        self.running = false;
    }

    /// This method is being called internally to check if an Animation should be updated on screen.
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
                println!(
                    "Next frame: {}, current frame: {}",
                    self.next_frame, current_frame
                );
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
