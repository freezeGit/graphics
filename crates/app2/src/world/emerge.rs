use rand::seq::SliceRandom;
use rand::{Rng, RngExt};
//use rand::Rng;

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

    pub fn new_with_random_ones(
        len: usize,
        initial_ones: usize,
        rng: &mut impl Rng,
    ) -> Self {
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
} // end of BitArray

pub fn step_bits(bits: &mut BitArray, rule: Rule, rng: &mut impl Rng) {
    let n = bits.len();

    let i = rng.random_range(0..n);
    let j = rng.random_range(0..n);

    interact(bits, rule, i, j);
}

fn interact(bits: &mut BitArray, rule: Rule, i: usize, j: usize) {
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
pub struct Rule {
    number: u8,
    flags: [bool; 4],
}

// #[derive(Debug, Clone, Copy)]
// pub(crate) struct Rule(bool, bool, bool, bool);

impl Rule {
    // pub(crate) fn new(n: u8) -> Self {
    //     assert!(n < 16, "Rule number must be less than 16, got {n}");
    //     Self(
    //         Self::bit(n, 3),
    //         Self::bit(n, 2),
    //         Self::bit(n, 1),
    //         Self::bit(n, 0),
    //     )
    // }

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
