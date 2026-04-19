//! ## Module timer contains the Timer struct.
//!
// timer_gl

use crate::Context;

use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Timer {
    interval: Duration,
    last_tick: Instant,
    running: bool,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_tick: Instant::now(),
            running: false,
        }
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }

    pub fn run(&mut self) {
        if !self.running {
            self.running = true;
            self.last_tick = Instant::now();
        }
    }

    //pub fn stop(&mut self) {
    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self) {
        self.last_tick = Instant::now();
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn ready_count(&mut self) -> u32 {
        if !self.running || self.interval.is_zero() {
            return 0;
        }

        let now = Instant::now();
        let elapsed = now.saturating_duration_since(self.last_tick);

        let count = (elapsed.as_nanos() / self.interval.as_nanos()) as u32;

        if count > 0 {
            self.last_tick += self.interval * count;
        }

        count
    }

    pub fn remaining(&self) -> Duration {
        if !self.running {
            return self.interval;
        }

        let now = Instant::now();
        self.interval
            .saturating_sub(now.saturating_duration_since(self.last_tick))
    }
}

// #[derive(Debug)]
// pub struct Timer {
//     interval: f64,
//     last_time: f64,
//     running: bool,
// }
//
// impl Timer {
//     pub fn new(interval: f64) -> Self {
//         Timer {
//             interval,
//             last_time: 0.0,
//             running: false,
//         }
//     }
//
//     pub fn is_time(&mut self, ctx: &Context) -> bool {
//         let mut retn = false;
//         if self.running {
//             let now = ctx.input(|i| i.time);
//             if now - self.last_time >= self.interval {
//                 self.last_time = now;
//                 retn = true;
//             }
//         }
//         retn
//     }
//     pub fn interval(&self) -> f64 {
//         self.interval
//     }
//     pub fn set_interval(&mut self, interval: f64) {
//         self.interval = interval;
//     }
//
//     pub fn is_running(&self) -> bool {
//         self.running
//     }
//
//     // Not using "frozen time" because ctx may not be available easily.
//     pub fn run(&mut self) {
//         self.running = true;
//     }
//
//     pub fn pause(&mut self) {
//         self.running = false;
//     }
// }
