//! Program state and simulation logic.
//!
//! This module defines `TheWorld`.
//! It deliberately has no dependency on gui_lib or egui.

// src/demo/world.rs

// Sub modules under mod world.
// Many applications will have multiple sub modules.
pub(crate) mod world_demo; // demo program data and logic
// ---------------------------------------------------

use gui_lib::World;
use crate::world::world_demo::{Gauge, ThingState, Thing, Signal, TrafficLight, Person};

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app struct or the canvas struct.
#[derive(Debug)]
pub(crate) struct TheWorld {
    pub(crate) frame_number: u32,
    pub(crate) tl: TrafficLight,
    pub(crate) thing: Thing,
    pub(crate) gauge: Gauge,
    //pub(crate) name: String,
    pub(crate) person: Person,
    pub(crate) value: f64,
}

impl World for TheWorld {
    /// Advance simulation by one step.
    /// If the application does not include a simulation,
    /// this method can be left undefined:
    /// it will be automatically implemented as an empty function.
    fn advance(&mut self) {
        // Increment frame number each simulation step.
        self.frame_number += 1;
        // Traffic light alternates between Go and Stop while simulation is running.
        self.toggle_light();
    }
}

impl TheWorld {
    pub(crate) fn new() -> Self {
        Self {
            frame_number: 0,
            tl: TrafficLight {
                state: Signal::Stop,
            },
            thing: Thing {
                state: ThingState::StateC,
            },
            gauge: Gauge::new(),
            //name: "Steve".to_string(),
            person: Person {
                //name: String::from("Bill"),
                name: String::from("Steve"),
                city: String::from("Birtle"),
                address: String::from("123 Main St"),
            },
            value: 0.0,
        }
    }

    fn toggle_light(&mut self) {
        self.tl.state = match self.tl.state {
            Signal::Stop => Signal::Go,
            Signal::Go => Signal::Stop,
        };
    }
}
