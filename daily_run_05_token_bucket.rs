use std::time::{Duration, Instant};

pub struct TokenBucket {
    capacity: f64,
    tokens: f64,
    refill_per_second: f64,
    updated_at: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u32, refill_per_second: u32) -> Self {
        assert!(capacity > 0 && refill_per_second > 0);
        Self { capacity: capacity as f64, tokens: capacity as f64,
            refill_per_second: refill_per_second as f64, updated_at: Instant::now() }
    }

    pub fn try_take(&mut self, requested: u32) -> bool {
        self.refill();
        let requested = requested as f64;
        if requested > self.tokens { return false; }
        self.tokens -= requested;
        true
    }

    pub fn available(&mut self) -> u32 { self.refill(); self.tokens.floor() as u32 }

    fn refill(&mut self) {
        let elapsed = self.updated_at.elapsed().as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_per_second).min(self.capacity);
        self.updated_at = Instant::now();
    }

    pub fn wait_hint(&mut self, requested: u32) -> Option<Duration> {
        if self.try_take(requested) { return None; }
        let missing = requested as f64 - self.tokens;
        Some(Duration::from_secs_f64(missing / self.refill_per_second))
    }
}
