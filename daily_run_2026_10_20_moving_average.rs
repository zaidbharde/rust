//! Fixed-window moving average with O(1) updates.

use std::collections::VecDeque;

#[derive(Debug)]
pub struct MovingAverage {
    window: VecDeque<f64>,
    limit: usize,
    sum: f64,
}

impl MovingAverage {
    pub fn new(limit: usize) -> Result<Self, &'static str> {
        if limit == 0 { Err("window size must be positive") } else { Ok(Self { window: VecDeque::with_capacity(limit), limit, sum: 0.0 }) }
    }

    pub fn push(&mut self, value: f64) -> Result<f64, &'static str> {
        if !value.is_finite() { return Err("value must be finite"); }
        self.window.push_back(value);
        self.sum += value;
        if self.window.len() > self.limit { self.sum -= self.window.pop_front().unwrap(); }
        Ok(self.mean())
    }

    pub fn mean(&self) -> f64 {
        if self.window.is_empty() { 0.0 } else { self.sum / self.window.len() as f64 }
    }

    pub fn len(&self) -> usize { self.window.len() }
    pub fn is_full(&self) -> bool { self.window.len() == self.limit }
}
