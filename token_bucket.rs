//! A token-bucket limiter suitable for deterministic request admission checks.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decision { pub allowed: bool, pub retry_after_ms: u64 }

#[derive(Debug)]
pub struct TokenBucket { capacity: u64, tokens: f64, refill_per_ms: f64, last_ms: u64 }

impl TokenBucket {
    pub fn new(capacity: u64, refill_per_second: u64, now_ms: u64) -> Self {
        assert!(capacity > 0 && refill_per_second > 0);
        Self { capacity, tokens: capacity as f64, refill_per_ms: refill_per_second as f64 / 1000.0, last_ms: now_ms }
    }

    fn refill(&mut self, now_ms: u64) {
        let elapsed = now_ms.saturating_sub(self.last_ms);
        self.tokens = (self.tokens + elapsed as f64 * self.refill_per_ms).min(self.capacity as f64);
        self.last_ms = now_ms;
    }

    pub fn try_take(&mut self, amount: u64, now_ms: u64) -> Decision {
        if amount == 0 { return Decision { allowed: true, retry_after_ms: 0 }; }
        self.refill(now_ms);
        if self.tokens >= amount as f64 {
            self.tokens -= amount as f64;
            return Decision { allowed: true, retry_after_ms: 0 };
        }
        let deficit = amount as f64 - self.tokens;
        let wait = (deficit / self.refill_per_ms).ceil() as u64;
        Decision { allowed: false, retry_after_ms: wait }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refills_after_time_passes() {
        let mut bucket = TokenBucket::new(2, 1_000, 0);
        assert!(bucket.try_take(2, 0).allowed);
        assert!(!bucket.try_take(1, 0).allowed);
        assert!(bucket.try_take(1, 1).allowed);
    }
}
