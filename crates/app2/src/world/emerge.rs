
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

    // fn set(&mut self, index: usize, value: bool) {
    //     self.words[index / 64] |= (value as u64) << (index % 64);
    // }
    //
    // fn get(&self, index: usize) -> bool {
    //     (self.words[index / 64] >> (index % 64)) & 1 == 1
    // }

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
}










