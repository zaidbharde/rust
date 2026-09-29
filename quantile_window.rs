use std::collections::VecDeque;

/// Computes an exact percentile over a bounded sliding window.
pub struct QuantileWindow {
    values: VecDeque<i64>,
    capacity: usize,
}

impl QuantileWindow {
    pub fn new(capacity: usize) -> Result<Self, &'static str> {
        if capacity == 0 { return Err("capacity must be positive"); }
        Ok(Self { values: VecDeque::with_capacity(capacity), capacity })
    }

    pub fn push(&mut self, value: i64) {
        if self.values.len() == self.capacity { self.values.pop_front(); }
        self.values.push_back(value);
    }

    pub fn percentile(&self, fraction: f64) -> Option<f64> {
        if self.values.is_empty() || !(0.0..=1.0).contains(&fraction) { return None; }
        let mut sorted = self.values.iter().copied().collect::<Vec<_>>();
        sorted.sort_unstable();
        let position = fraction * (sorted.len() - 1) as f64;
        let lower = position.floor() as usize;
        let upper = position.ceil() as usize;
        if lower == upper { Some(sorted[lower] as f64) }
        else {
            let weight = position - lower as f64;
            Some(sorted[lower] as f64 * (1.0 - weight) + sorted[upper] as f64 * weight)
        }
    }

    pub fn len(&self) -> usize { self.values.len() }
}
