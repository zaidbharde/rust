use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Maintains the median of values inserted so far.
pub struct Median { lower: BinaryHeap<i64>, upper: BinaryHeap<Reverse<i64>> }

impl Median {
    pub fn new() -> Self {
        Self { lower: BinaryHeap::new(), upper: BinaryHeap::new() }
    }

    pub fn push(&mut self, value: i64) {
        if self.lower.peek().map_or(true, |&top| value <= top) {
            self.lower.push(value);
        } else {
            self.upper.push(Reverse(value));
        }
        self.rebalance();
    }

    fn rebalance(&mut self) {
        if self.lower.len() > self.upper.len() + 1 {
            self.upper.push(Reverse(self.lower.pop().unwrap()));
        } else if self.upper.len() > self.lower.len() {
            self.lower.push(self.upper.pop().unwrap().0);
        }
    }

    pub fn value(&self) -> Option<f64> {
        match (self.lower.peek(), self.upper.peek()) {
            (None, _) => None,
            (Some(&left), Some(&Reverse(right))) if self.lower.len() == self.upper.len() => {
                Some((left as f64 + right as f64) / 2.0)
            }
            (Some(&left), _) => Some(left as f64),
        }
    }
}

fn main() {
    let mut median = Median::new();
    for value in [9, 1, 7, 3, 5] { median.push(value); }
    println!("median = {:?}", median.value());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handles_even_and_odd_counts() {
        let mut m = Median::new();
        m.push(4); m.push(1); m.push(9); assert_eq!(m.value(), Some(4.0));
        m.push(7); assert_eq!(m.value(), Some(5.5));
    }
}
