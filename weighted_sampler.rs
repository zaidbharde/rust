//! Weighted selection using a simple linear-congruential pseudo-random source.

pub struct Sampler {
    state: u64,
}

impl Sampler {
    pub fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        self.state
    }

    pub fn choose(&mut self, weights: &[u64]) -> Option<usize> {
        if weights.is_empty() || weights.iter().all(|weight| *weight == 0) {
            return None;
        }
        let total = weights.iter().try_fold(0u64, |sum, weight| sum.checked_add(*weight))?;
        let mut ticket = self.next() % total;
        for (index, weight) in weights.iter().enumerate() {
            if ticket < *weight {
                return Some(index);
            }
            ticket -= weight;
        }
        None
    }
}

pub fn normalized_percentages(weights: &[u64]) -> Vec<f64> {
    let total: u64 = weights.iter().sum();
    if total == 0 {
        return vec![0.0; weights.len()];
    }
    weights.iter().map(|weight| *weight as f64 * 100.0 / total as f64).collect()
}
