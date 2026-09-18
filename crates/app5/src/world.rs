//! Program state and simulation logic.
//!
//! This module defines `TheWorld`.
//! It deliberately has no dependency on gui_lib or egui.

// src/demo/world.rs

// Submodules under mod world.
// Many applications will have multiple sub modules.
pub mod emerge;
// ---------------------------------------------------

use crate::inits::{
    INITIAL_BITS_NUM, INITIAL_ONES, INITIAL_RULE, INITIAL_SEQ_DISCARD, INITIAL_SEQ_LENGTH,
};
pub(crate) use crate::world::emerge::BitsRule;
//use crate::world::emerge::{step_bits, step_bg, BitArray, BitGraph, Seq, CntnsRule};
use crate::world::emerge::{step_bg, BitArray, BitGraph, Seq, CntnsRule};
use gui_lib::World;
use rand::rngs::ThreadRng;
use rand::{Rng, RngExt};

/// TheWorld struct encapsulates application data and logic.
/// It has no dependence on gui_lib and no dependence on egui.
/// It has no dependence on the app1 struct or the canvas struct.
//#[derive(Debug)] // TDJ: Debug is not needed pub struct

pub struct TheWorld {
    pub rng: ThreadRng,
    //pub bits: BitArray,
    pub bit_graph: BitGraph,
    pub bits_rule: BitsRule,
    pub cntns_rule: CntnsRule,
    pub start_ones: usize,
    pub attractor: Seq,
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
        //step_bits(&mut self.bits, self.rule, &mut self.rng);
        // TDJ: Needs work
        //step_bits(&mut self.bit_graph.values, self.rule, &mut self.rng);
        step_bg(&mut self.bit_graph, self.bits_rule, self.cntns_rule, &mut self.rng);
    }
}

impl TheWorld {
    pub fn new() -> Self {
        Self {
            rng: rand::rng(),
            //bits: BitArray::new(INITIAL_BITS_NUM),
            bit_graph: BitGraph::new(INITIAL_BITS_NUM),
            bits_rule: BitsRule::new(INITIAL_RULE),
            cntns_rule: CntnsRule::new(0),
            start_ones: INITIAL_ONES,
            //attractor: Seq::new(INITIAL_SEQ_DISCARD, INITIAL_SEQ_LENGTH),
            attractor: Seq::new(INITIAL_SEQ_DISCARD),
            //attractor: Vec::new(),
            frame_number: 0,
        }
    }
}

// pub struct TheWorld {
//     pub rng: ThreadRng,
//     //pub bits: BitArray,
//     pub bit_graph: BitGraph,
//     pub rule: BitsRule,
//     pub start_ones: usize,
//     pub attractor: Seq,
//     pub frame_number: u64,
// }
//
// impl World for TheWorld {
//     /// Advance simulation by one step.
//     /// If the application does not include a simulation,
//     /// this method can be left undefined:
//     /// it will be automatically implemented as an empty function.
//     fn advance(&mut self) {
//         // Increment frame number each simulation step.
//         self.frame_number += 1;
//         // Advance simulation by one step.
//         //step_bits(&mut self.bits, self.rule, &mut self.rng);
//         // TDJ: Needs work
//         //step_bits(&mut self.bit_graph.values, self.rule, &mut self.rng);
//         step_bg(&mut self.bit_graph, self.rule, &mut self.rng);
//     }
// }
//
// impl TheWorld {
//     pub fn new() -> Self {
//         Self {
//             rng: rand::rng(),
//             //bits: BitArray::new(INITIAL_BITS_NUM),
//             bit_graph: BitGraph::new(INITIAL_BITS_NUM),
//             rule: BitsRule::new(INITIAL_RULE),
//             start_ones: INITIAL_ONES,
//             //attractor: Seq::new(INITIAL_SEQ_DISCARD, INITIAL_SEQ_LENGTH),
//             attractor: Seq::new(INITIAL_SEQ_DISCARD),
//             //attractor: Vec::new(),
//             frame_number: 0,
//         }
//     }
// }
