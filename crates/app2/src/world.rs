//! Program state and simulation logic.
//!
//! This module defines `TheWorld`.
//! It deliberately has no dependency on gui_lib or egui.

// src/demo/world.rs

// Submodules under mod world.
// Many applications will have multiple sub modules.
//pub(crate) mod world_demo; // demo program data and logic
pub(crate) mod emerge;
// ---------------------------------------------------

use crate::world::emerge::{BitArray, step_bits};
pub(crate) use crate::world::emerge::Rule;
use rand::{Rng, RngExt};
use rand::rngs::ThreadRng;
use gui_lib::World;

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app1 struct or the canvas struct.
//#[derive(Debug)] // TDJ: Debug is not needed
pub(crate) struct TheWorld {
    rng: ThreadRng,
    pub bits: BitArray,
    pub rule: Rule,
    pub current_rule: u8,
    pub(crate) frame_number: u64,

    pub test_string: String,
}

impl World for TheWorld {
    /// Advance simulation by one step.
    /// If the application does not include a simulation,
    /// this method can be left undefined:
    /// it will be automatically implemented as an empty function.
    fn advance(&mut self) {
        //println!("{:?}", self.rule);
        // Increment frame number each simulation step.
        self.frame_number += 1;

        // Advance simulation by one step.
        step_bits(&mut self.bits, self.rule, &mut self.rng);

        // Traffic light alternates between Go and Stop while simulation is running.
        //self.toggle_light();
    }
}

impl TheWorld {
    pub(crate) fn new() -> Self {
        const INITIAL_RULE: u8 = 13;
        let mut bits = BitArray::new(6000);

        // TDJ: Example: change some bits before storing the BitArray.
        // Replace these with the actual BitArray methods from your emerge module.
        bits.set(0, true);
        bits.set(10, true);
        bits.set(42, true);

        Self {
            rng: rand::rng(),
            bits,
            rule: Rule::new(INITIAL_RULE),
            current_rule: INITIAL_RULE,
            frame_number: 0,

            test_string: "Hello".to_string(),
            //test_int: 42,
        }
    }

    fn set_rule(&mut self, n: u8) {
        self.rule = Rule::new(n);
    }
}

