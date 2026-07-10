use rand::{Rng, RngExt};

pub(crate) struct BitArray {
    words: Vec<u64>,
    len: usize,
}

impl BitArray {
    pub(crate) fn new(len: usize) -> Self {
        assert!(
            len >= 2,
            "BitArray length must be at least 2, got {len}"
        );
        
        let word_count = (len + 63) / 64;
        Self {
            words: vec![0; word_count],
            len,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn get(&self, i: usize) -> bool {
        debug_assert!(i < self.len);

        let word_index = i / 64;
        let bit_index = i % 64;

        (self.words[word_index] & (1u64 << bit_index)) != 0
    }

    pub(crate) fn set(&mut self, i: usize, value: bool) {
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
} // end of BitArray

pub(crate) fn step_bits(bits: &mut BitArray, rng: &mut impl Rng) {
    //let n = bits.len;
    let n = bits.len();

    assert!(
        n >= 2,
        "step requires at least 2 bits, got {n}"
    );

    let i = rng.random_range(0..n);

    // let mut j = rng.random_range(0..n - 1);
    // if j >= i {
    //     j += 1;
    // }

    let i = rng.random_range(0..n);
    let j = rng.random_range(0..n);

    if i == j {
        return;
    }

    let a = bits.get(i);
    let b = bits.get(j);

    let (new_a, new_b) = match (a, b) {
        (false, false) => (false, true),
        (false, true)  => (true, false),
        (true, false)  => (true, true),
        (true, true)   => (false, false),
    };

    bits.set(i, new_a);
    bits.set(j, new_b);
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









