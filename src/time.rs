use std::ops::{Add, AddAssign, Sub};
use std::time::Instant;

#[derive(PartialOrd, Debug, Clone, Copy)]
pub struct Timestamp(u64, u32, Instant);
impl Timestamp {
    pub fn now() -> Self {
        Timestamp(0, 0, Instant::now())
    }
    pub fn tick(&mut self) -> Self {
        let now = Instant::now();
        let dif = now - self.2;
        *self += Timestamp(dif.as_secs(), dif.subsec_millis(), now);
        *self
    }
    pub fn new(sec: u64, msec: u32) -> Self {
        Timestamp(sec, msec, Instant::now())
    }
}
impl PartialEq for Timestamp {
    fn eq(&self, other: &Timestamp) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Add for Timestamp {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut next_one = self.1 + other.1;
        let next_zero = self.0 + other.0 + (next_one / 1000) as u64;
        next_one %= 1000;
        Self(next_zero, next_one, Instant::now())
    }
}

impl AddAssign for Timestamp {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}

impl Sub for Timestamp {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let my_msec = self.0 * 1000 + self.1 as u64;
        let other_msec = other.0 * 1000 + other.1 as u64;
        if other_msec > my_msec {
            return Self(0, 0, Instant::now());
        }
        let sub_msec = my_msec - other_msec;
        let next_one = (sub_msec % 1000) as u32;
        let next_zero = sub_msec / 1000;
        Self(next_zero, next_one, Instant::now())
    }
}
