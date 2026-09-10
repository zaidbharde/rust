//! Token bucket limiter using caller-supplied monotonic timestamps.

#[derive(Debug, Clone)]
pub struct FractionalTokenBucket {
    capacity: f64,
    refill_per_second: f64,
    tokens: f64,
    last_seen: f64,
}

impl FractionalTokenBucket {
    pub fn new(capacity: f64, refill_per_second: f64, now: f64) -> Self {
        assert!(capacity > 0.0 && refill_per_second > 0.0);
        Self { capacity, refill_per_second, tokens: capacity, last_seen: now }
    }

    fn refill(&mut self, now: f64) {
        assert!(now >= self.last_seen, "timestamps must be monotonic");
        let elapsed = now - self.last_seen;
        self.tokens = (self.tokens + elapsed * self.refill_per_second).min(self.capacity);
        self.last_seen = now;
    }

    pub fn try_acquire(&mut self, permits: f64, now: f64) -> bool {
        assert!(permits >= 0.0, "permits cannot be negative");
        self.refill(now);
        if self.tokens < permits { return false; }
        self.tokens -= permits;
        true
    }

    pub fn available(&mut self, now: f64) -> f64 {
        self.refill(now);
        self.tokens
    }

    pub fn wait_seconds(&mut self, permits: f64, now: f64) -> f64 {
        assert!(permits <= self.capacity, "request exceeds bucket capacity");
        self.refill(now);
        if self.tokens >= permits { return 0.0; }
        (permits - self.tokens) / self.refill_per_second
    }
}
