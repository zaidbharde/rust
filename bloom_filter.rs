use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A compact probabilistic set for fast membership checks.
pub struct BloomFilter {
    bits: Vec<bool>,
    hashes: usize,
}

impl BloomFilter {
    pub fn new(bit_count: usize, hashes: usize) -> Self {
        assert!(bit_count > 0 && hashes > 0);
        Self { bits: vec![false; bit_count], hashes }
    }

    fn locations<T: Hash>(&self, value: &T) -> impl Iterator<Item = usize> {
        let mut first = DefaultHasher::new();
        value.hash(&mut first);
        let h1 = first.finish();
        let mut second = DefaultHasher::new();
        (h1 ^ 0x9e3779b97f4a7c15).hash(&mut second);
        value.hash(&mut second);
        let h2 = second.finish() | 1;
        (0..self.hashes).map(move |i| {
            h1.wrapping_add((i as u64).wrapping_mul(h2)) as usize % self.bits.len()
        })
    }

    pub fn insert<T: Hash>(&mut self, value: &T) {
        for index in self.locations(value) {
            self.bits[index] = true;
        }
    }

    pub fn probably_contains<T: Hash>(&self, value: &T) -> bool {
        self.locations(value).all(|index| self.bits[index])
    }
}

fn main() {
    let mut filter = BloomFilter::new(128, 4);
    filter.insert(&"rust");
    println!("rust={} java={}", filter.probably_contains(&"rust"), filter.probably_contains(&"java"));
}
