//! Program state and simulation logic.
//!
//! This module defines `TheWorld` and related types.
//! It deliberately has no dependency on gui_lib or egui.

// world.rs

#[derive(Debug)]
pub(crate) struct Gauge {
    pointer: f64,
}

impl Gauge {
    fn new() -> Self {
        Self { pointer: 0.0 }
    }

    pub(crate) fn pointer(&self) -> f64 {
        self.pointer
    }

    pub(crate) fn set_pointer(&mut self, pointer: f64) {
        self.pointer = pointer;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Signal {
    Stop,
    Go,
}
#[derive(Debug)]
pub(crate) struct TrafficLight {
    pub(crate) state: Signal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ThingState {
    StateA,
    StateB,
    StateC,
}
#[derive(Debug)]
pub(crate) struct Thing {
    pub(crate) state: ThingState,
}

#[derive(Debug)]
pub(crate) struct Person {
    pub(crate) name: String,
    pub(crate) city: String,
    pub(crate) address: String,
}


/// Encapsulates program data and logic.
/// No dependence on gui_lb
/// No dependence on the app struct or the canvas struct.
#[derive(Debug)]
pub(crate) struct TheWorld {
    pub(crate) tl: TrafficLight,
    pub(crate) thing: Thing,
    pub(crate) gauge: Gauge,
    pub(crate) name: String,
    pub(crate) person: Person,
    pub(crate) value: f64,
}

impl TheWorld {
    pub(crate) fn new() -> Self {
        Self {
            //state: 0,
            tl: TrafficLight {
                state: Signal::Stop,
            },
            thing: Thing {
                state: ThingState::StateC,
            },
            gauge: Gauge::new(),
            name: "Steve".to_string(),
            person: Person {
                name: String::from("Bill"),
                city: String::from("Birtle"),
                address: String::from("123 Main St"),
            },
            value: 0.0,
        }
    }

    pub(crate) fn advance(&mut self) {
        //self.state += 1;
        self.toggle_light();
    }

    fn toggle_light(&mut self) {
        self.tl.state = match self.tl.state {
            Signal::Stop => Signal::Go,
            Signal::Go => Signal::Stop,
        };
    }
}
