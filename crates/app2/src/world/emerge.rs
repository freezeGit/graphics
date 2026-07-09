
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

    pub(crate)fn get(&self, i: usize) -> bool {
        debug_assert!(i < self.len);

        let word_index = i / 64;
        let bit_index = i % 64;

        (self.words[word_index] & (1u64 << bit_index)) != 0
    }

    pub(crate)fn set(&mut self, i: usize, value: bool) {
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









