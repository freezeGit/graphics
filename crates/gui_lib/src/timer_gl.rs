//! ## Module timer contains the Timer struct.
//!
// timer_gl

#[derive(Debug, Clone, Copy, PartialEq)]
enum TimerState {
    Stopped,        // Call to ready(): returns false (Timer not active).
    WaitingForSync, // Call to ready(): sync to current time and return false.
    Running,        // Call to ready(): returns true if interval has elapsed since last call.
}

#[derive(Debug)]
pub struct SimTimer {
    interval: f64,
    last_time: f64,
    fast: bool,
    batch_size: u32,
    state: TimerState,
}

impl SimTimer {
    pub fn new(interval: f64, batch_size: u32) -> Self {
        Self {
            interval,
            last_time: 0.0,
            fast: false,
            batch_size,
            state: TimerState::Stopped,
        }
    }

    pub fn run(&mut self) {
        if self.state == TimerState::Stopped {
            self.state = TimerState::WaitingForSync;
        }
    }

    pub fn pause(&mut self) {
        self.state = TimerState::Stopped;
    }

    pub fn resync(&mut self) {
        if self.state != TimerState::Stopped {
            self.state = TimerState::WaitingForSync;
        }
    }

    pub fn is_running(&self) -> bool {
        self.state != TimerState::Stopped
    }

    pub fn set_interval(&mut self, interval: f64) {
        self.interval = interval;
    }

    pub fn interval(&self) -> f64 {
        self.interval
    }

    pub fn ready(&mut self, now: f64) -> bool {
        match self.state {
            TimerState::Stopped => false,

            // First call after starting: sync to current time
            TimerState::WaitingForSync => {
                self.last_time = now;
                self.state = TimerState::Running;
                false
            }

            TimerState::Running => {
                if now - self.last_time >= self.interval {
                    self.last_time += self.interval;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn remaining(&self, now: f64) -> f64 {
        match self.state {
            TimerState::Stopped | TimerState::WaitingForSync => self.interval,

            TimerState::Running => (self.interval - (now - self.last_time)).max(0.0),
        }
    }
    // pub fn set_normal_speed(&mut self) {
    //     self.fast = false;
    // }
    pub fn set_fast_forward(&mut self) {
        self.fast = true;
    }
    pub fn fast_forward(&self) -> bool {
        self.fast
    }
    pub fn exit_fast_forward(&mut self) {
        if self.fast {
            self.fast = false;
            self.resync();
        }
    }
    pub fn set_batch_size(&mut self, batch_size: u32) {
        self.batch_size = batch_size;
    }
    pub fn batch_size(&self) -> u32 {
        self.batch_size
    }
} // SimTimer
