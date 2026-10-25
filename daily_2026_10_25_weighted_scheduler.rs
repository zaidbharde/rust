#[derive(Debug, Clone)]
pub struct WeightedScheduler {
    weights: Vec<u32>,
    current: Vec<i64>,
    total: i64,
}

impl WeightedScheduler {
    pub fn new(weights: Vec<u32>) -> Option<Self> {
        if weights.is_empty() || weights.iter().any(|&weight| weight == 0) {
            return None;
        }
        let total = weights.iter().map(|&weight| weight as i64).sum();
        Some(Self { current: vec![0; weights.len()], total, weights })
    }

    pub fn next(&mut self) -> usize {
        let mut selected = 0;
        for index in 0..self.weights.len() {
            self.current[index] += self.weights[index] as i64;
            if self.current[index] > self.current[selected] {
                selected = index;
            }
        }
        self.current[selected] -= self.total;
        selected
    }
}

#[cfg(test)]
mod tests {
    use super::WeightedScheduler;

    #[test]
    fn preserves_weighted_counts() {
        let mut scheduler = WeightedScheduler::new(vec![1, 2]).unwrap();
        let picks: Vec<_> = (0..6).map(|_| scheduler.next()).collect();
        assert_eq!(picks.iter().filter(|&&pick| pick == 1).count(), 4);
    }
}
