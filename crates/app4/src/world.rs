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
    INITIAL_BITS_NUM, INITIAL_ONES, INITIAL_RULE, INITIAL_SEQ_DISCARD, INITIAL_SEQ_LENGTH,
};
pub(crate) use crate::world::emerge::Rule;
use crate::world::emerge::{BitArray, Seq, step_bits};
use gui_lib::World;

use rand::rngs::ThreadRng;
use rand::{Rng, RngExt};

// use rand::SeedableRng; // Required trait for initialization methods
// use rand::rngs::SmallRng;
// use rand::Rng;         // Required trait to actually generate numbers

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app1 struct or the canvas struct.
//#[derive(Debug)] // TDJ: Debug is not needed
pub struct TheWorld {
    pub rng: ThreadRng,
    //pub rng: SmallRng,
    pub bits: BitArray,
    pub rule: Rule,
    pub start_ones: usize,
    pub attractor: Seq,
    pub frame_number: u64,
}

impl World for TheWorld {
    // Advance simulation by one step.
    // If the application does not include a simulation,
    // this method can be left undefined:
    // it will be automatically implemented as an empty function.
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
            //rng: rand::make_rng(),
            bits: BitArray::new(INITIAL_BITS_NUM),
            rule: Rule::new(INITIAL_RULE),
            start_ones: INITIAL_ONES,
            //attractor: Seq::new(INITIAL_SEQ_DISCARD, INITIAL_SEQ_LENGTH),
            attractor: Seq::new(INITIAL_SEQ_DISCARD),
            //attractor: Vec::new(),
            frame_number: 0,
        }
    }
} // end impl TheWorld
