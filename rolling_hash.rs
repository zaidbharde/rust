//! Polynomial rolling hash for byte strings and constant-time substring hashes.

const BASE: u64 = 1_000_003;
const MODULUS: u64 = 4_294_967_291;

pub struct RollingHash {
    prefix: Vec<u64>,
    powers: Vec<u64>,
}

impl RollingHash {
    pub fn new(input: &[u8]) -> Self {
        let mut prefix = vec![0; input.len() + 1];
        let mut powers = vec![1; input.len() + 1];
        for (index, byte) in input.iter().enumerate() {
            prefix[index + 1] = (prefix[index] * BASE + u64::from(*byte) + 1) % MODULUS;
            powers[index + 1] = (powers[index] * BASE) % MODULUS;
        }
        Self { prefix, powers }
    }

    pub fn hash(&self, start: usize, end: usize) -> Option<u64> {
        if start > end || end >= self.prefix.len() {
            return None;
        }
        let removed = (self.prefix[start] * self.powers[end - start]) % MODULUS;
        Some((self.prefix[end] + MODULUS - removed) % MODULUS)
    }

    pub fn equal_ranges(&self, left: (usize, usize), right: (usize, usize)) -> bool {
        left.1 - left.0 == right.1 - right.0
            && self.hash(left.0, left.1) == self.hash(right.0, right.1)
    }
}

pub fn find_pattern(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() || pattern.len() > text.len() {
        return Vec::new();
    }
    let text_hash = RollingHash::new(text);
    let pattern_hash = RollingHash::new(pattern).hash(0, pattern.len()).unwrap();
    (0..=text.len() - pattern.len())
        .filter(|&start| text_hash.hash(start, start + pattern.len()) == Some(pattern_hash))
        .filter(|&start| text[start..start + pattern.len()] == *pattern)
        .collect()
}
