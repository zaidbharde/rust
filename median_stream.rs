use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Maintains the median of inserted integers in logarithmic time per update.
pub struct MedianStream {
    lower: BinaryHeap<i64>,
    upper: BinaryHeap<Reverse<i64>>,
}

impl MedianStream {
    pub fn new() -> Self {
        Self { lower: BinaryHeap::new(), upper: BinaryHeap::new() }
    }

    pub fn push(&mut self, value: i64) {
        if self.lower.peek().map_or(true, |top| value <= *top) {
            self.lower.push(value);
        } else {
            self.upper.push(Reverse(value));
        }
        self.rebalance();
    }

    fn rebalance(&mut self) {
        while self.lower.len() > self.upper.len() + 1 {
            self.upper.push(Reverse(self.lower.pop().unwrap()));
        }
        while self.upper.len() > self.lower.len() {
            self.lower.push(self.upper.pop().unwrap().0);
        }
    }

    pub fn median(&self) -> Option<f64> {
        match (self.lower.peek(), self.upper.peek()) {
            (None, _) => None,
            (Some(&low), Some(&Reverse(high))) if self.lower.len() == self.upper.len() => {
                Some((low as f64 + high as f64) / 2.0)
            }
            (Some(&low), _) => Some(low as f64),
        }
    }
}

fn main() {
    let mut stream = MedianStream::new();
    for value in [12, 4, 9, 20, 7] { stream.push(value); }
    println!("median={:.1}", stream.median().unwrap());
}
