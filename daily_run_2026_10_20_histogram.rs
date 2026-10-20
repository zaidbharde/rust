//! Streaming histogram with deterministic bucket boundaries.

#[derive(Debug, Clone)]
pub struct Histogram {
    bounds: Vec<f64>,
    counts: Vec<u64>,
    underflow: u64,
    overflow: u64,
}

impl Histogram {
    pub fn new(mut bounds: Vec<f64>) -> Result<Self, &'static str> {
        if bounds.is_empty() || bounds.iter().any(|value| !value.is_finite()) {
            return Err("bounds must contain finite values");
        }
        bounds.sort_by(|left, right| left.partial_cmp(right).unwrap());
        if bounds.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err("bounds must be unique");
        }
        Ok(Self { counts: vec![0; bounds.len() + 1], bounds, underflow: 0, overflow: 0 })
    }

    pub fn observe(&mut self, value: f64) {
        match self.bounds.binary_search_by(|bound| bound.partial_cmp(&value).unwrap()) {
            Ok(index) | Err(index) => self.counts[index] += 1,
        }
        if value < self.bounds[0] { self.underflow += 1; }
        if value > *self.bounds.last().unwrap() { self.overflow += 1; }
    }

    pub fn counts(&self) -> &[u64] { &self.counts }
    pub fn extremes(&self) -> (u64, u64) { (self.underflow, self.overflow) }
}
