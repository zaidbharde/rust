//! Select workers according to stable weighted round-robin scheduling.

#[derive(Debug, Clone)]
pub struct WeightedPicker {
    weights: Vec<u32>,
    current: Vec<i64>,
    total: i64,
}

impl WeightedPicker {
    pub fn new(weights: Vec<u32>) -> Result<Self, &'static str> {
        if weights.is_empty() || weights.iter().any(|&weight| weight == 0) {
            return Err("weights must be non-empty and positive");
        }
        let total = weights.iter().map(|&weight| weight as i64).sum();
        Ok(Self { current: vec![0; weights.len()], weights, total })
    }

    pub fn next(&mut self) -> usize {
        for (score, &weight) in self.current.iter_mut().zip(&self.weights) {
            *score += weight as i64;
        }
        let winner = self.current.iter().enumerate()
            .max_by_key(|&(index, score)| (*score, -(index as i64)))
            .map(|(index, _)| index)
            .expect("validated weights are non-empty");
        self.current[winner] -= self.total;
        winner
    }
}

fn main() {
    let mut picker = WeightedPicker::new(vec![5, 3, 2]).unwrap();
    let sequence: Vec<usize> = (0..10).map(|_| picker.next()).collect();
    assert_eq!(sequence.iter().filter(|&&id| id == 0).count(), 5);
    assert_eq!(sequence.iter().filter(|&&id| id == 1).count(), 3);
    assert_eq!(sequence.iter().filter(|&&id| id == 2).count(), 2);
    println!("weighted schedule: {sequence:?}");
}
