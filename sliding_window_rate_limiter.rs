use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct SlidingWindowLimiter {
    limit: usize,
    window: Duration,
    events: VecDeque<Instant>,
}

impl SlidingWindowLimiter {
    pub fn new(limit: usize, window: Duration) -> Self {
        Self { limit, window, events: VecDeque::with_capacity(limit) }
    }

    pub fn allow(&mut self, now: Instant) -> bool {
        while self.events.front().is_some_and(|&time| now.duration_since(time) >= self.window) {
            self.events.pop_front();
        }
        if self.events.len() >= self.limit {
            return false;
        }
        self.events.push_back(now);
        true
    }

    pub fn remaining(&self) -> usize { self.limit.saturating_sub(self.events.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn expires_old_events() {
        let start = Instant::now();
        let mut limiter = SlidingWindowLimiter::new(2, Duration::from_secs(1));
        assert!(limiter.allow(start));
        assert!(limiter.allow(start + Duration::from_millis(1)));
        assert!(!limiter.allow(start + Duration::from_millis(2)));
        assert!(limiter.allow(start + Duration::from_secs(1)));
    }
}

fn main() {}
