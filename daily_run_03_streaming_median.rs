use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct StreamingMedian {
    lower: BinaryHeap<i64>,
    upper: BinaryHeap<Reverse<i64>>,
}

impl StreamingMedian {
    pub fn new() -> Self { Self { lower: BinaryHeap::new(), upper: BinaryHeap::new() } }

    pub fn push(&mut self, value: i64) {
        if self.lower.peek().map_or(true, |largest| value <= *largest) {
            self.lower.push(value);
        } else { self.upper.push(Reverse(value)); }
        self.rebalance();
    }

    pub fn median(&self) -> Option<f64> {
        match (self.lower.peek(), self.upper.peek()) {
            (None, None) => None,
            (Some(left), None) => Some(*left as f64),
            (None, Some(Reverse(right))) => Some(*right as f64),
            (Some(left), Some(Reverse(right))) if self.lower.len() == self.upper.len() => {
                Some((*left as f64 + *right as f64) / 2.0)
            }
            (Some(left), _) => Some(*left as f64),
        }
    }

    fn rebalance(&mut self) {
        while self.lower.len() > self.upper.len() + 1 {
            self.upper.push(Reverse(self.lower.pop().unwrap()));
        }
        while self.upper.len() > self.lower.len() {
            self.lower.push(self.upper.pop().unwrap().0);
        }
    }
}
