//! Fixed-width histogram with within-bin percentile estimates.

#[derive(Debug, Clone)]
pub struct PercentileBins {
    min: f64,
    max: f64,
    counts: Vec<u64>,
    total: u64,
}

impl PercentileBins {
    pub fn new(min: f64, max: f64, bins: usize) -> Result<Self, &'static str> {
        if !min.is_finite() || !max.is_finite() || min > max {
            return Err("range must be finite and ordered");
        }
        if bins == 0 { return Err("at least one bin is required"); }
        Ok(Self { min, max, counts: vec![0; bins], total: 0 })
    }

    /// Adds a finite observation; non-finite values are ignored. Out-of-range values clamp to edge bins.
    pub fn observe(&mut self, value: f64) {
        if !value.is_finite() { return; }
        let index = if value <= self.min || self.min == self.max { 0 }
            else if value >= self.max { self.counts.len() - 1 }
            else { (((value - self.min) / (self.max - self.min)) * self.counts.len() as f64) as usize };
        self.counts[index] = self.counts[index].saturating_add(1);
        self.total = self.total.saturating_add(1);
    }

    pub fn counts(&self) -> &[u64] { &self.counts }
    pub fn len(&self) -> u64 { self.total }
    pub fn is_empty(&self) -> bool { self.total == 0 }

    /// Estimates a quantile in [0, 1], assuming observations are uniform within each occupied bin.
    pub fn quantile(&self, fraction: f64) -> Option<f64> {
        if self.total == 0 || !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) { return None; }
        if self.min == self.max { return Some(self.min); }
        let rank = fraction * (self.total - 1) as f64;
        let mut before = 0u64;
        for (i, &count) in self.counts.iter().enumerate() {
            if count > 0 && rank < (before + count) as f64 {
                let within = ((rank - before as f64 + 0.5) / count as f64).clamp(0.0, 1.0);
                return Some(self.min + (i as f64 + within) / self.counts.len() as f64 * (self.max - self.min));
            }
            before += count;
        }
        Some(self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::PercentileBins;

    #[test]
    fn estimates_and_clamps_observations() {
        let mut h = PercentileBins::new(0.0, 10.0, 5).unwrap();
        for x in [-3.0, 1.0, 9.0, 14.0] { h.observe(x); }
        assert_eq!(h.len(), 4);
        assert_eq!(h.counts(), &[2, 0, 0, 0, 2]);
        assert!(h.quantile(0.0).unwrap() < h.quantile(1.0).unwrap());
    }

    #[test]
    fn rejects_bad_inputs_and_handles_empty_or_constant_range() {
        assert!(PercentileBins::new(2.0, 1.0, 2).is_err());
        assert!(PercentileBins::new(0.0, 1.0, 0).is_err());
        let mut h = PercentileBins::new(4.0, 4.0, 3).unwrap();
        h.observe(f64::NAN);
        assert_eq!(h.quantile(0.5), None);
        h.observe(99.0);
        assert_eq!(h.quantile(0.5), Some(4.0));
        assert_eq!(h.quantile(1.1), None);
    }
}
