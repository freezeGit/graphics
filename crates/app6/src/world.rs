//! Program state and simulation logic.
//!
//! This module defines `TheWorld`.
//! It deliberately has no dependency on gui_lib or egui.

// src/demo/world.rs

// Submodules under mod world.
// Many applications will have multiple sub modules.
pub mod emerge;
// ---------------------------------------------------

// use crate::inits::{
//     INITIAL_BITS_NUM, INITIAL_BITS_RULE, INITIAL_CXN_RULE, INITIAL_CXNS, INITIAL_ONES,
//     INITIAL_SEQ_DISCARD, INITIAL_SEQ_LENGTH,
// };
use crate::inits::{
    INITIAL_BITS_NUM, INITIAL_BITS_RULE, INITIAL_CXN_RULE, INITIAL_CXNS, INITIAL_ONES,
};
//pub(crate) use crate::world::emerge::BitsRule;
use crate::world::emerge::{step_bg, BitArray, BitGraph, BitsRule, CxnsRule};
use gui_lib::World;
use rand::rngs::ThreadRng;
use rand::{Rng, RngExt};

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app1 struct or the canvas struct.

pub struct TheWorld {
    pub rng: ThreadRng,
    pub bit_graph: BitGraph,
    pub bits_rule: BitsRule,
    pub cxns_rule: CxnsRule,
    pub start_ones: usize,
    pub start_cxns: usize,
    pub frame_number: u64,
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
        step_bg(
            &mut self.bit_graph,
            self.bits_rule,
            self.cxns_rule,
            &mut self.rng,
        );
    }
}

impl TheWorld {
    pub fn new() -> Self {
        Self {
            rng: rand::rng(),
            bit_graph: BitGraph::new(INITIAL_BITS_NUM),
            bits_rule: BitsRule::new(INITIAL_BITS_RULE),
            cxns_rule: CxnsRule::new(INITIAL_CXN_RULE),
            start_ones: INITIAL_ONES,
            start_cxns: INITIAL_CXNS,
            frame_number: 0,
        }
    }
}
