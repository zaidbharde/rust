//! A small, deterministic xorshift64 pseudo-random number generator.

/// Xorshift64 generator with a fixed nonzero fallback for a zero seed.
///
/// The all-zero state is absorbing for xorshift, so `new(0)` uses state 1.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    /// Creates a generator from `seed`; zero is replaced with one.
    pub fn new(seed: u64) -> Self {
        Self { state: if seed == 0 { 1 } else { seed } }
    }

    /// Advances the generator and returns the next 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.state = value;
        value
    }

    /// Returns the current internal state without advancing the generator.
    pub fn state(&self) -> u64 {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::XorShift64;

    #[test]
    fn matches_known_sequence_for_seed_one() {
        let mut rng = XorShift64::new(1);
        assert_eq!(rng.next_u64(), 1_082_269_761);
        assert_eq!(rng.next_u64(), 1_152_992_998_833_853_505);
        assert_eq!(rng.next_u64(), 11_177_516_664_432_764_457);
        assert_eq!(rng.next_u64(), 17_678_023_832_001_937_445);
    }

    #[test]
    fn same_seed_repeats_and_different_seeds_diverge() {
        let mut first = XorShift64::new(42);
        let mut second = XorShift64::new(42);
        let mut other = XorShift64::new(43);
        for _ in 0..8 {
            assert_eq!(first.next_u64(), second.next_u64());
        }
        assert_ne!(XorShift64::new(42).next_u64(), other.next_u64());
    }

    #[test]
    fn zero_seed_uses_nonzero_state() {
        let mut zero = XorShift64::new(0);
        let mut one = XorShift64::new(1);
        assert_eq!(zero.state(), 1);
        assert_eq!(zero.next_u64(), one.next_u64());
        assert_ne!(zero.state(), 0);
    }
}
