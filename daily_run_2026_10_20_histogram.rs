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

    /// Records a finite observation. NaN and infinities are ignored.
    pub fn observe(&mut self, value: f64) {
        if !value.is_finite() {
            return;
        }
        match self.bounds.binary_search_by(|bound| bound.partial_cmp(&value).unwrap()) {
            Ok(index) | Err(index) => self.counts[index] += 1,
        }
        if value < self.bounds[0] { self.underflow += 1; }
        if value > *self.bounds.last().unwrap() { self.overflow += 1; }
    }

    pub fn counts(&self) -> &[u64] { &self.counts }
    pub fn extremes(&self) -> (u64, u64) { (self.underflow, self.overflow) }
}

#[cfg(test)]
mod tests {
    use super::Histogram;

    #[test]
    fn finite_values_preserve_bucket_and_extreme_counts() {
        let mut histogram = Histogram::new(vec![1.0, 2.0]).unwrap();
        for value in [0.0, 1.0, 1.5, 2.0, 3.0] {
            histogram.observe(value);
        }
        assert_eq!(histogram.counts(), &[2, 2, 1]);
        assert_eq!(histogram.extremes(), (1, 1));
    }

    #[test]
    fn non_finite_observations_are_ignored_without_panicking() {
        let mut histogram = Histogram::new(vec![1.0, 2.0]).unwrap();
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            histogram.observe(value);
        }
        assert_eq!(histogram.counts(), &[0, 0, 0]);
        assert_eq!(histogram.extremes(), (0, 0));
    }

    #[test]
    fn constructor_rejects_empty_or_non_finite_bounds() {
        assert!(Histogram::new(vec![]).is_err());
        assert!(Histogram::new(vec![f64::NAN]).is_err());
        assert!(Histogram::new(vec![f64::INFINITY]).is_err());
    }
}
