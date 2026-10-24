//! Estimate an exact percentile over a bounded stream using an ordered sample.

#[derive(Debug, Default)]
pub struct PercentileWindow {
    width: usize,
    values: std::collections::VecDeque<f64>,
}

impl PercentileWindow {
    pub fn new(width: usize) -> Result<Self, &'static str> {
        if width == 0 { return Err("window width must be positive"); }
        Ok(Self { width, values: std::collections::VecDeque::new() })
    }

    pub fn push(&mut self, value: f64) {
        self.values.push_back(value);
        if self.values.len() > self.width {
            self.values.pop_front();
        }
    }

    pub fn percentile(&self, fraction: f64) -> Result<f64, &'static str> {
        if self.values.is_empty() || !(0.0..=1.0).contains(&fraction) {
            return Err("need values and a fraction between zero and one");
        }
        let mut sorted: Vec<f64> = self.values.iter().copied().collect();
        sorted.sort_by(|left, right| left.partial_cmp(right).unwrap());
        let rank = fraction * (sorted.len() - 1) as f64;
        let lower = rank.floor() as usize;
        let upper = rank.ceil() as usize;
        let blend = rank - lower as f64;
        Ok(sorted[lower] + (sorted[upper] - sorted[lower]) * blend)
    }
}

fn main() {
    let mut window = PercentileWindow::new(4).unwrap();
    for value in [10.0, 20.0, 30.0, 40.0, 50.0] { window.push(value); }
    assert_eq!(window.percentile(0.5).unwrap(), 35.0);
    assert_eq!(window.percentile(1.0).unwrap(), 50.0);
    println!("percentile window: {:?}", window.values);
}
