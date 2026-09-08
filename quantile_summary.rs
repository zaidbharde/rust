//! A bounded-memory quantile summary for small telemetry streams.

#[derive(Debug, Clone)]
pub struct QuantileSummary { values: Vec<f64>, limit: usize, count: usize }

impl QuantileSummary {
    pub fn new(limit: usize) -> Self {
        assert!(limit > 1, "summary limit must exceed one");
        Self { values: Vec::with_capacity(limit), limit, count: 0 }
    }

    pub fn observe(&mut self, value: f64) {
        if !value.is_finite() { return; }
        self.count += 1;
        let position = self.values.binary_search_by(|item| item.total_cmp(&value)).unwrap_or_else(|p| p);
        self.values.insert(position, value);
        if self.values.len() > self.limit {
            let index = self.values.len() * (self.count % 2) / 2;
            self.values.remove(index.min(self.values.len() - 1));
        }
    }

    pub fn count(&self) -> usize { self.count }
    pub fn len(&self) -> usize { self.values.len() }

    pub fn percentile(&self, fraction: f64) -> Option<f64> {
        if self.values.is_empty() || !(0.0..=1.0).contains(&fraction) { return None; }
        let index = ((self.values.len() - 1) as f64 * fraction).round() as usize;
        self.values.get(index).copied()
    }

    pub fn median(&self) -> Option<f64> { self.percentile(0.5) }
}

#[cfg(test)]
mod tests {
    use super::QuantileSummary;
    #[test]
    fn ignores_non_finite_values() {
        let mut summary = QuantileSummary::new(8);
        summary.observe(f64::NAN); summary.observe(1.0); summary.observe(3.0); summary.observe(2.0);
        assert_eq!(summary.count(), 3);
        assert_eq!(summary.median(), Some(2.0));
    }
}
