//! Program state and simulation logic.
//!
//! This module defines `TheWorld`.
//! It deliberately has no dependency on gui_lib or egui.

// src/demo/world.rs

// Submodules under mod world.
// Many applications will have multiple sub modules.
pub(crate) mod emerge;
// ---------------------------------------------------

use crate::world::emerge::{BitArray, step_bits};
pub(crate) use crate::world::emerge::Rule;
use crate::inits::{INITIAL_RULE, INITIAL_BITS_NUM};
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
    pub(crate) frame_number: u64,
}

impl World for TheWorld {
    /// Advance simulation by one step.
    /// If the application does not include a simulation,
    /// this method can be left undefined:
    /// it will be automatically implemented as an empty function.
    fn advance(&mut self) {
        // Increment frame number each simulation step.
        self.frame_number += 1;
        // Advance simulation by one step.
        step_bits(&mut self.bits, self.rule, &mut self.rng);
    }
}

impl TheWorld {
    pub(crate) fn new() -> Self {
        Self {
            rng: rand::rng(),
            bits: BitArray::new(INITIAL_BITS_NUM),
            rule: Rule::new(INITIAL_RULE),
            frame_number: 0,
        }
    }
}

