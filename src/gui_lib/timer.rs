// timer.rs

//use crate::gui_lib::Context;
use crate::gui_lib::egui::Context;

#[derive(Debug)]
pub struct Timer {
    interval: f64,
    last_time: f64,
    running: bool,
}

impl Timer {
    pub fn new(interval: f64) -> Self {
        Timer {
            interval,
            last_time: 0.0,
            running: false,
        }
    }

    pub fn is_time(&mut self, ctx: &Context) -> bool {
        let mut retn = false;
        if self.running {
            let now = ctx.input(|i| i.time);
            if now - self.last_time >= self.interval {
                self.last_time = now;
                retn = true;
            }
        }
        retn
    }
    pub fn interval(&self) -> f64 {
        self.interval
    }
    pub fn set_interval(&mut self, interval: f64) {
        self.interval = interval;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    // Not using "frozen time" because ctx may not be available easily.
    pub fn run(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }
}