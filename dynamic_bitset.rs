/// Compact growable bitset backed by machine words.
pub struct BitSet {
    words: Vec<u64>,
    length: usize,
}

impl BitSet {
    pub fn with_len(length: usize) -> Self {
        Self { words: vec![0; length.div_ceil(64)], length }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.length, "bit index outside set");
        let word = index / 64;
        let mask = 1u64 << (index % 64);
        if value { self.words[word] |= mask; } else { self.words[word] &= !mask; }
    }

    pub fn contains(&self, index: usize) -> bool {
        assert!(index < self.length, "bit index outside set");
        self.words[index / 64] & (1u64 << (index % 64)) != 0
    }

    pub fn count(&self) -> usize {
        self.words.iter().map(|word| word.count_ones() as usize).sum()
    }

    pub fn union_with(&mut self, other: &Self) {
        assert_eq!(self.length, other.length, "bitsets must have equal length");
        for (left, right) in self.words.iter_mut().zip(&other.words) { *left |= right; }
    }
}

fn main() {
    let mut bits = BitSet::with_len(130);
    bits.set(2, true); bits.set(128, true);
    println!("{} {}", bits.contains(128), bits.count());
}
