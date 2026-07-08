//! Program state and simulation logic.
//!
//! This module defines `TheWorld`.
//! It deliberately has no dependency on gui_lib or egui.

// src/demo/world.rs

// Sub modules under mod world.
// Many applications will have multiple sub modules.
pub(crate) mod world_demo; // demo program data and logic
pub(crate) mod emerge;
// ---------------------------------------------------

use crate::world::world_demo::{Gauge, Person, Signal, Thing, ThingState, TrafficLight};
use crate::world::emerge::BitArray;
//use rand::Rng;
use rand::{Rng, RngExt};
use gui_lib::World;

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app1 struct or the canvas struct.
//#[derive(Debug)] // TDJ: Debug is not needed
pub(crate) struct TheWorld {
    bits: BitArray,
    pub(crate) frame_number: u64, // TDJ: for batching
    pub(crate) tl: TrafficLight,
    pub(crate) thing: Thing,
    pub(crate) gauge: Gauge,
    //pub(crate) name: String,
    pub(crate) person: Person,
    pub(crate) value: f64,
}

fn step(bits: &mut BitArray, rng: &mut impl Rng) {
    //let n = bits.len;
    let n = bits.len();

    assert!(
        n >= 2,
        "step requires at least 2 bits, got {n}"
    );

    let i = rng.random_range(0..n);

    let mut j = rng.random_range(0..n - 1);
    if j >= i {
        j += 1;
    }

    let a = bits.get(i);
    let b = bits.get(j);

    let (new_a, new_b) = match (a, b) {
        (false, false) => (false, true),
        (false, true)  => (true, false),
        (true, false)  => (true, true),
        (true, true)   => (false, false),
    };

    bits.set(i, new_a);
    bits.set(j, new_b);
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
            bits: BitArray::new(10),

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
