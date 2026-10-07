//! Weighted rolling average over the most recent observations.

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Sample { value: f64, weight: f64 }

#[derive(Debug)]
pub struct WeightedWindow {
    capacity: usize,
    samples: VecDeque<Sample>,
    weighted_sum: f64,
    total_weight: f64,
}

impl WeightedWindow {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        Self { capacity, samples: VecDeque::with_capacity(capacity), weighted_sum: 0.0, total_weight: 0.0 }
    }

    pub fn add(&mut self, value: f64, weight: f64) -> Result<(), &'static str> {
        if !value.is_finite() || !weight.is_finite() || weight <= 0.0 { return Err("value and weight must be finite; weight must be positive"); }
        let sample = Sample { value, weight };
        self.samples.push_back(sample);
        self.weighted_sum += value * weight;
        self.total_weight += weight;
        if self.samples.len() > self.capacity {
            if let Some(expired) = self.samples.pop_front() {
                self.weighted_sum -= expired.value * expired.weight;
                self.total_weight -= expired.weight;
            }
        }
        Ok(())
    }

    pub fn average(&self) -> Option<f64> {
        (self.total_weight > 0.0).then(|| self.weighted_sum / self.total_weight)
    }
}
