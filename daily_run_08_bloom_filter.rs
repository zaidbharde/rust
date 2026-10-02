//! A dependency-free Bloom filter using double hashing.
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bits: Vec<u64>,
    rounds: u32,
}

impl BloomFilter {
    pub fn new(bit_count: usize, rounds: u32) -> Self {
        assert!(bit_count > 0 && rounds > 0);
        Self { bits: vec![0; bit_count.div_ceil(64)], rounds }
    }

    fn hashes<T: Hash>(&self, value: &T) -> (u64, u64) {
        let mut first = DefaultHasher::new();
        value.hash(&mut first);
        let h1 = first.finish();
        let mut second = DefaultHasher::new();
        h1.hash(&mut second);
        (h1, second.finish() | 1)
    }

    fn position(&self, hash: u64) -> (usize, u64) {
        let bit = (hash as usize) % (self.bits.len() * 64);
        (bit / 64, 1u64 << (bit % 64))
    }

    pub fn insert<T: Hash>(&mut self, value: &T) {
        let (mut h1, h2) = self.hashes(value);
        for _ in 0..self.rounds {
            let (word, mask) = self.position(h1);
            self.bits[word] |= mask;
            h1 = h1.wrapping_add(h2);
        }
    }

    pub fn might_contain<T: Hash>(&self, value: &T) -> bool {
        let (mut h1, h2) = self.hashes(value);
        for _ in 0..self.rounds {
            let (word, mask) = self.position(h1);
            if self.bits[word] & mask == 0 { return false; }
            h1 = h1.wrapping_add(h2);
        }
        true
    }
}
