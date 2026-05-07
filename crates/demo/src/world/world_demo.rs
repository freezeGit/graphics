// src/demo/world/demo

use super::*;

// ----------- struct Gauge _________________
#[derive(Debug)]
pub(crate) struct Gauge {
    pointer: f64,
}

impl Gauge {
    pub(super) fn new() -> Self {
        Self { pointer: 0.0 }
    }
    pub(crate) fn pointer(&self) -> f64 {
        self.pointer
    }
    pub(crate) fn set_pointer(&mut self, pointer: f64) {
        self.pointer = pointer;
    }
} // Gauge

// ----------- enum Signal _________________
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Signal {
    Stop,
    Go,
}

// ----------- struct TrafficLight _________________
#[derive(Debug)]
pub(crate) struct TrafficLight {
    pub(crate) state: Signal,
}

// ----------- enum ThingState _________________

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ThingState {
    StateA,
    StateB,
    StateC,
}

// ----------- struct Thing _________________
#[derive(Debug)]
pub(crate) struct Thing {
    pub(crate) state: ThingState,
}
// ----------- struct Person ________________

#[derive(Debug)]
pub(crate) struct Person {
    pub(crate) name: String,
    pub(crate) city: String,
    pub(crate) address: String,
}
