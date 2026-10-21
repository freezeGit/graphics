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
//use crate::inits::INITIAL_RULE;
use crate::inits::{
    INITIAL_BITS_NUM, INITIAL_BITS_RULE, INITIAL_BITS_RULE_PROBABILITY, INITIAL_BITS_RULE_RANDOM,
    INITIAL_CXN_RULE, INITIAL_CXN_RULE_PROBABILITY, INITIAL_CXN_RULE_RANDOM, INITIAL_CXNS,
    INITIAL_ONES, INITIAL_SAMPLE_SIZE,
};
//pub(crate) use crate::world::emerge::Rule;

use crate::world::emerge::{BitArray, BitGraph, BitsRule, CxnsRule, step_bg};
//use crate::world::emerge::{BitArray, Seq, step_bits, step_bits_rand};
use gui_lib::World;

use crate::world::delta_ones::Deltas;
// use rand::rngs::ThreadRng;
// use rand::{Rng, RngExt};
// use rand::prelude::SmallRng;

use rand::SeedableRng;
use rand::rngs::SmallRng;

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app1 struct or the canvas struct.
//#[derive(Debug)] // TDJ: Debug is not needed
// pub struct TheWorld {
//     pub rng: ThreadRng,
//     pub rule: Rule,
//     pub sample_size: u32,
// }

pub struct TheWorld {
    //pub rng: ThreadRng,
    pub rng: SmallRng,
    pub bit_graph: BitGraph,
    pub bits_rule: BitsRule,
    pub cxns_rule: CxnsRule,
    pub start_ones: usize,
    pub start_cxns: usize,
    pub sample_size: u32,
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
    // pub fn new() -> Self {
    //     Self {
    //         rng: rand::rng(),
    //         rule: Rule::new(5),
    //         sample_size: 100,
    //     }
    // }

    pub fn new() -> Self {
        Self {
            rng: SmallRng::from_rng(&mut rand::rng()),
            //rng: SmallRng::seed_from_u64(12345),  // reproducable
            bit_graph: BitGraph::new(INITIAL_BITS_NUM),
            bits_rule: BitsRule::new(
                INITIAL_BITS_RULE,
                INITIAL_BITS_RULE_RANDOM,
                INITIAL_BITS_RULE_PROBABILITY,
            ),
            cxns_rule: CxnsRule::new(
                INITIAL_CXN_RULE,
                INITIAL_CXN_RULE_RANDOM,
                INITIAL_CXN_RULE_PROBABILITY,
            ),
            start_ones: INITIAL_ONES,
            start_cxns: INITIAL_CXNS,
            sample_size: INITIAL_SAMPLE_SIZE,
            frame_number: 0,
        }
    }

    pub fn recalc_deltas(&mut self) -> Deltas {
        Deltas::new(self.bits_rule, self.sample_size, &mut self.rng)
    }


} // end impl TheWorld
