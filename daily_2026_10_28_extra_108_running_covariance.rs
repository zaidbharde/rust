//! Online bivariate covariance and correlation using a numerically stable update.

/// Streaming population/sample covariance state for paired finite observations.
#[derive(Clone, Copy, Debug, Default)]
pub struct RunningCovariance {
    count: u64,
    mean_x: f64,
    mean_y: f64,
    co_moment: f64,
    m2_x: f64,
    m2_y: f64,
}

impl RunningCovariance {
    /// Adds one pair. Non-finite inputs are rejected without changing the state.
    pub fn push(&mut self, x: f64, y: f64) -> bool {
        if !x.is_finite() || !y.is_finite() || self.count == u64::MAX { return false; }
        let n = (self.count + 1) as f64;
        let dx = x - self.mean_x;
        let dy = y - self.mean_y;
        self.mean_x += dx / n;
        self.mean_y += dy / n;
        self.co_moment += dx * (y - self.mean_y);
        self.m2_x += dx * (x - self.mean_x);
        self.m2_y += dy * (y - self.mean_y);
        self.count += 1;
        true
    }

    pub fn count(&self) -> u64 { self.count }
    pub fn means(&self) -> Option<(f64, f64)> {
        (self.count > 0).then_some((self.mean_x, self.mean_y))
    }
    /// Population covariance; `None` when no pairs have been observed.
    pub fn population_covariance(&self) -> Option<f64> {
        (self.count > 0).then_some(self.co_moment / self.count as f64)
    }
    /// Unbiased sample covariance; `None` until at least two pairs are observed.
    pub fn sample_covariance(&self) -> Option<f64> {
        if self.count < 2 { None } else { Some(self.co_moment / (self.count - 1) as f64) }
    }
    /// Pearson correlation; undefined for too few pairs or a constant variable.
    pub fn correlation(&self) -> Option<f64> {
        if self.count < 2 || self.m2_x <= 0.0 || self.m2_y <= 0.0 { return None; }
        let r = self.co_moment / (self.m2_x * self.m2_y).sqrt();
        r.is_finite().then_some(r.clamp(-1.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::RunningCovariance;

    #[test]
    fn computes_covariance_and_correlation_incrementally() {
        let mut s = RunningCovariance::default();
        for pair in [(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)] { assert!(s.push(pair.0, pair.1)); }
        assert_eq!(s.count(), 3);
        assert_eq!(s.means(), Some((2.0, 4.0)));
        assert!((s.population_covariance().unwrap() - 4.0 / 3.0).abs() < 1e-12);
        assert_eq!(s.sample_covariance(), Some(2.0));
        assert!((s.correlation().unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn handles_empty_constant_and_non_finite_inputs() {
        let mut s = RunningCovariance::default();
        assert_eq!(s.means(), None);
        assert_eq!(s.sample_covariance(), None);
        assert_eq!(s.correlation(), None);
        assert!(!s.push(f64::NAN, 1.0));
        assert_eq!(s.count(), 0);
        s.push(5.0, 1.0); s.push(5.0, 3.0);
        assert_eq!(s.correlation(), None);
    }
}
