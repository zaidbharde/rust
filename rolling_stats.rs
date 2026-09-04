//! Numerically stable rolling mean and variance over a bounded sample window.
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RollingStats {
    limit: usize,
    values: VecDeque<f64>,
    sum: f64,
    sum_squares: f64,
}

impl RollingStats {
    pub fn new(limit: usize) -> Self {
        assert!(limit > 0, "window limit must be positive");
        Self { limit, values: VecDeque::with_capacity(limit), sum: 0.0, sum_squares: 0.0 }
    }

    pub fn push(&mut self, value: f64) {
        assert!(value.is_finite(), "samples must be finite");
        self.values.push_back(value);
        self.sum += value;
        self.sum_squares += value * value;
        if self.values.len() > self.limit {
            let expired = self.values.pop_front().unwrap();
            self.sum -= expired;
            self.sum_squares -= expired * expired;
        }
    }

    pub fn len(&self) -> usize { self.values.len() }
    pub fn mean(&self) -> Option<f64> { (!self.values.is_empty()).then(|| self.sum / self.values.len() as f64) }

    pub fn variance(&self) -> Option<f64> {
        if self.values.len() < 2 { return None; }
        let count = self.values.len() as f64;
        let result = (self.sum_squares - self.sum * self.sum / count) / (count - 1.0);
        Some(result.max(0.0))
    }

    pub fn latest(&self) -> Option<f64> { self.values.back().copied() }
}
