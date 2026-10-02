//! A small circuit breaker with closed, open, and half-open states.
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    failures: u32,
    threshold: u32,
    opened_at: Option<Instant>,
    cooldown: Duration,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, cooldown: Duration) -> Self {
        assert!(threshold > 0, "threshold must be positive");
        Self { failures: 0, threshold, opened_at: None, cooldown }
    }

    pub fn state(&self, now: Instant) -> State {
        match self.opened_at {
            None => State::Closed,
            Some(when) if now.duration_since(when) >= self.cooldown => State::HalfOpen,
            Some(_) => State::Open,
        }
    }

    pub fn allow(&self, now: Instant) -> bool {
        self.state(now) != State::Open
    }

    pub fn record_success(&mut self) {
        self.failures = 0;
        self.opened_at = None;
    }

    pub fn record_failure(&mut self, now: Instant) {
        self.failures = self.failures.saturating_add(1);
        if self.failures >= self.threshold {
            self.opened_at = Some(now);
        }
    }

    pub fn failures(&self) -> u32 { self.failures }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opens_after_threshold() {
        let now = Instant::now();
        let mut breaker = CircuitBreaker::new(2, Duration::from_secs(1));
        breaker.record_failure(now);
        assert!(breaker.allow(now));
        breaker.record_failure(now);
        assert!(!breaker.allow(now));
    }
}
