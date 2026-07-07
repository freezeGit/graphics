
struct BitArray {
    words: Vec<u64>,
    len: usize,
}

impl BitArray {
    fn new(len: usize) -> Self {
        let word_count = (len + 63) / 64;
        Self {
            words: vec![0; word_count],
            len,
        }
    }

    fn set(&mut self, index: usize, value: bool) {
        self.words[index / 64] |= (value as u64) << (index % 64);
    }

    fn get(&self, index: usize) -> bool {
        (self.words[index / 64] >> (index % 64)) & 1 == 1
    }
}










