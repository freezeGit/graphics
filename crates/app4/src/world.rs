//! Program state and simulation logic.
//!
//! This module defines `TheWorld`.
//! It deliberately has no dependency on gui_lib or egui.

// src/demo/world.rs

// Submodules under mod world.
// Many applications will have multiple sub modules.
pub mod emerge;
pub mod delta_ones;
// ---------------------------------------------------

use crate::inits::{
    INITIAL_BITS_NUM, INITIAL_ONES, INITIAL_RULE, INITIAL_SEQ_DISCARD,
    INITIAL_SEQ_LENGTH,
};
pub(crate) use crate::world::emerge::Rule;
use crate::world::emerge::{BitArray, Seq, step_bits};
use gui_lib::World;

use rand::rngs::ThreadRng;
use rand::{Rng, RngExt};
use crate::world::delta_ones::Deltas;
// use rand::SeedableRng; // Required trait for initialization methods
// use rand::rngs::SmallRng;
// use rand::Rng;         // Required trait to actually generate numbers

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app1 struct or the canvas struct.
//#[derive(Debug)] // TDJ: Debug is not needed
pub struct TheWorld {
    pub rng: ThreadRng,
    pub rule: Rule,
    pub sample_size: u32,

    pub bits: BitArray,
    pub start_ones: usize,
    pub attractor: Seq,
    pub frame_number: u64,
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

            bits: BitArray::new(INITIAL_BITS_NUM),
            start_ones: INITIAL_ONES,
            //attractor: Seq::new(INITIAL_SEQ_DISCARD, INITIAL_SEQ_LENGTH),
            attractor: Seq::new(INITIAL_SEQ_DISCARD),
            //attractor: Vec::new(),
            frame_number: 0,
        }
    }

    //let d = Deltas::new(Rule::new(5), 100, &mut self.world.rng);

    pub fn recalc_deltas (&mut self) -> Deltas {
        Deltas::new(self.rule, self.sample_size, &mut self.rng)
    }
} // end impl TheWorld
