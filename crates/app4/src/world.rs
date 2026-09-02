//! Program state and simulation logic.
//!
//! This module defines `TheWorld`.
//! It deliberately has no dependency on gui_lib or egui.

// src/demo/world.rs

// Submodules under mod world.
// Many applications will have multiple sub modules.
pub mod delta_ones;
pub mod emerge;
// ---------------------------------------------------

// use crate::inits::{
//     INITIAL_RULE, INITIAL_SEQ_DISCARD,
//     INITIAL_SEQ_LENGTH,
// };
use crate::inits::INITIAL_RULE;
pub(crate) use crate::world::emerge::Rule;
use crate::world::emerge::{BitArray, Seq, step_bits};
use gui_lib::World;

use crate::world::delta_ones::Deltas;
use rand::rngs::ThreadRng;
use rand::{Rng, RngExt};

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app1 struct or the canvas struct.
//#[derive(Debug)] // TDJ: Debug is not needed
pub struct TheWorld {
    pub rng: ThreadRng,
    pub rule: Rule,
    pub sample_size: u32,
}

// Advance simulation by one step.
// If the application does not include a simulation,
// this method can be left undefined:
// it will be automatically implemented as an empty function.
impl World for TheWorld {
    // fn advance(&mut self) {
    //     // Increment frame number each simulation step.
    //     self.frame_number += 1;
    //     // Advance simulation by one step.
    //     step_bits(&mut self.bits, self.rule, &mut self.rng);
    // }
} // end impl World

impl TheWorld {
    pub fn new() -> Self {
        Self {
            rng: rand::rng(),
            rule: Rule::new(5),
            sample_size: 100,
        }
    }

    pub fn recalc_deltas(&mut self) -> Deltas {
        Deltas::new(self.rule, self.sample_size, &mut self.rng)
    }
} // end impl TheWorld
