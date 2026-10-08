use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A small sliding-window limiter suitable for guarding bursty work.
pub struct SlidingWindowLimiter {
    window: Duration,
    limit: usize,
    events: VecDeque<Instant>,
}

impl SlidingWindowLimiter {
    pub fn new(limit: usize, window: Duration) -> Self {
        assert!(limit > 0, "limit must be positive");
        Self { window, limit, events: VecDeque::new() }
    }

    pub fn try_acquire(&mut self, now: Instant) -> bool {
        self.expire(now);
        if self.events.len() >= self.limit { return false; }
        self.events.push_back(now);
        true
    }

    pub fn remaining(&mut self, now: Instant) -> usize {
        self.expire(now);
        self.limit.saturating_sub(self.events.len())
    }

    fn expire(&mut self, now: Instant) {
        while self.events.front().is_some_and(|stamp| now.duration_since(*stamp) >= self.window) {
            self.events.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_only_inside_window() {
        let start = Instant::now();
        let mut limiter = SlidingWindowLimiter::new(2, Duration::from_secs(1));
        assert!(limiter.try_acquire(start));
        assert!(limiter.try_acquire(start));
        assert!(!limiter.try_acquire(start));
        assert!(limiter.try_acquire(start + Duration::from_secs(1)));
    }
}
