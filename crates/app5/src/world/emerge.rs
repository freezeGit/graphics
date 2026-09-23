use rand::seq::SliceRandom;
use rand::{Rng, RngExt};

pub struct BitArray {
    words: Vec<u64>,
    len: usize,
}

impl BitArray {
    pub fn new(len: usize) -> Self {
        assert!(len >= 2, "BitArray length must be at least 2, got {len}");

        let word_count = (len + 63) / 64;
        Self {
            words: vec![0; word_count],
            len,
        }
    }

    pub fn new_with_initial_ones(len: usize, initial_ones: usize) -> Self {
        assert!(len >= 2, "BitArray length must be at least 2, got {len}");
        assert!(
            initial_ones <= len,
            "Initial ones cannot exceed total length"
        );

        let word_count = (len + 63) / 64;
        let mut bit_array = Self {
            words: vec![0; word_count],
            len,
        };

        for i in 0..initial_ones {
            bit_array.set(i, true);
        }

        bit_array
    }

    pub fn new_with_random_ones(len: usize, initial_ones: usize, rng: &mut impl Rng) -> Self {
        assert!(len >= 2, "BitArray length must be at least 2, got {len}");
        assert!(
            initial_ones <= len,
            "Initial ones cannot exceed total length"
        );

        let mut bit_array = Self::new(len);

        let mut indices: Vec<usize> = (0..len).collect();
        indices.shuffle(rng);

        for &i in &indices[..initial_ones] {
            bit_array.set(i, true);
        }

        bit_array
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, i: usize) -> bool {
        debug_assert!(i < self.len);

        let word_index = i / 64;
        let bit_index = i % 64;

        (self.words[word_index] & (1u64 << bit_index)) != 0
    }

    pub fn set(&mut self, i: usize, value: bool) {
        debug_assert!(i < self.len);

        let word_index = i / 64;
        let bit_index = i % 64;
        let mask = 1u64 << bit_index;

        if value {
            self.words[word_index] |= mask;
        } else {
            self.words[word_index] &= !mask;
        }
    }

    pub fn ones_count(&self) -> usize {
        self.words
            .iter()
            .map(|word| word.count_ones() as usize)
            .sum()
    }

    pub fn ones_fraction(&self) -> f64 {
        self.ones_count() as f64 / self.len() as f64
    }
} // end of BitArray

fn interact_bits(bits: &mut BitArray, rule: BitsRule, i: usize, j: usize) {
    if i == j {
        return;
    }

    let a = bits.get(i);
    let b = bits.get(j);

    // Symmetrical application of the rule.
    let (new_a, new_b) = rule.apply(a, b);

    bits.set(i, new_a);
    bits.set(j, new_b);
}

#[derive(Debug, Clone, Copy)]
pub struct BitsRule {
    number: u8,
    flags: [bool; 4],
}

impl BitsRule {
    pub fn new(number: u8) -> Self {
        assert!(
            number < 16,
            "Rule number must be less than 16, got {number}"
        );

        Self {
            number,
            flags: [
                Self::bit(number, 3), // (false, false)
                Self::bit(number, 2), // (false, true)
                Self::bit(number, 1), // (true,  false)
                Self::bit(number, 0), // (true,  true)
            ],
        }
    }

    pub fn number(&self) -> u8 {
        self.number
    }

    fn bit(n: u8, i: u8) -> bool {
        ((n >> i) & 1) != 0
    }

    fn response(self, this: bool, other: bool) -> bool {
        // The two bits are equal
        if this == other {
            if this { self.flags[0] } else { self.flags[1] }
        // The two bits are different
        } else if this {
            self.flags[2]
        } else {
            self.flags[3]
        }
    }

    fn apply(self, a: bool, b: bool) -> (bool, bool) {
        // symmetrically reversible rule
        (self.response(a, b), self.response(b, a))
    }
} // end of impl BitsRule

pub struct BitGraph {
    pub values: BitArray, // TDJ: Can I get rid of pub?
    connections: BitArray,
}

impl BitGraph {
    // TDJ: May want to add a new() function to initialize a graph with an initial number of nodes
    pub fn new(nodes: usize) -> Self {
        assert!(nodes >= 2);

        Self {
            values: BitArray::new(nodes),
            connections: BitArray::new(nodes * nodes),
        }
    }

    pub fn new_with_random_ones(nodes: usize, initial_ones: usize, rng: &mut impl Rng) -> Self {
        assert!(nodes >= 2, "BitArray nodes must be at least 2, got {nodes}");
        assert!(
            initial_ones <= nodes,
            "Initial ones cannot exceed total number of nodes"
        );

        let mut grph = Self::new(nodes);

        let mut indices: Vec<usize> = (0..nodes).collect();
        indices.shuffle(rng);

        for &i in &indices[..initial_ones] {
            grph.set_node(i, true);
        }

        grph
    }

    pub fn nodes(&self) -> usize {
        self.values.len()
    }

    pub fn get_node(&self, i: usize) -> bool {
        debug_assert!(i < self.nodes());

        let word_index = i / 64;
        let bit_index = i % 64;

        (self.values.words[word_index] & (1u64 << bit_index)) != 0
    }

    pub fn set_node(&mut self, i: usize, value: bool) {
        debug_assert!(i < self.nodes());

        let word_index = i / 64;
        let bit_index = i % 64;
        let mask = 1u64 << bit_index;

        if value {
            //self.words[word_index] |= mask;
            self.values.words[word_index] |= mask;
        } else {
            //self.words[word_index] &= !mask;
            self.values.words[word_index] &= !mask;
        }
    }

    pub fn ones_count(&self) -> usize {
        self.values.ones_count()
    }

    // pub fn ones_fraction(&self) -> f64 {
    //     self.values.ones_fraction()
    // }

    pub fn cnctns_count(&self) -> usize {
        // TDJ: ? count only first half
        self.connections.ones_count() / 2
    }

    fn node_index(&self, a: usize, b: usize) -> usize {
        a * self.nodes() + b
    }

    fn is_connected(&self, a: usize, b: usize) -> bool {
        self.connections.get(self.node_index(a, b))
    }

    fn set_connected(&mut self, a: usize, b: usize, connected: bool) {
        let ab = a * self.nodes() + b;
        let ba = b * self.nodes() + a;

        self.connections.set(ab, connected);
        self.connections.set(ba, connected);
    }
} // end Impl BitGraph

fn change_cnctn(bg: &mut BitGraph, rule: CnctnsRule, i: usize, j: usize) {
    match rule.number {
        // No change
        0 => {}
        // Toggle connection
        1 => {
            if bg.is_connected(i, j) {
                bg.set_connected(i, j, false);
            } else {
                bg.set_connected(i, j, true);
            }
        }
        // Connect
        2 => {
            bg.set_connected(i, j, true);
        }
        // Disconnect
        3 => {
            bg.set_connected(i, j, false);
        }
        _ => {
            panic!("Connections Rule number must be less than 4, got {}", rule.number);
        }
    }
}

pub fn step_bg(bg: &mut BitGraph, bits_rule: BitsRule, cnctns_rule: CnctnsRule, rng: &mut impl Rng) {
    let n = bg.nodes();

    let i = rng.random_range(0..n);
    let j = rng.random_range(0..n);

    interact_bits(&mut bg.values, bits_rule, i, j);
    change_cnctn(bg, cnctns_rule, i, j);
    //println!("Connections: {}", bg.cnctns_count());

    //println!("Connection: {}", bg.is_connected(i, j));
}

#[derive(Debug, Clone, Copy)]
//pub struct Rulecnctns {
pub struct CnctnsRule {
    number: u8,
    //flags: [bool; 4],
}

impl CnctnsRule {
    pub fn new(number: u8) -> Self {
        assert!(
            number < 4,
            "Connections Rule number must be less than 4, got {number}"
        );

        Self { number }
    }

    pub fn number(&self) -> u8 {
        self.number
    }
}

// --------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_array_works() {
        //  BitArray::new(10) sets all 10 elements to false
        let mut array = BitArray::new(10);
        assert_eq!(array.len(), 10);
        assert!(!array.get(5));
        // set element 5 to true
        array.set(5, true);
        assert!(array.get(5));
        // set element 5 to false
        array.set(5, false);
        assert!(!array.get(5));
    }
} // end of tests
