//! Deterministic retry schedule for services with bounded backoff.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryDelay { pub attempt: u32, pub milliseconds: u64 }

#[derive(Debug, Clone, Copy)]
pub struct RetrySchedule {
    max_attempts: u32,
    base_ms: u64,
    cap_ms: u64,
}

impl RetrySchedule {
    pub fn new(max_attempts: u32, base_ms: u64, cap_ms: u64) -> Self {
        assert!(max_attempts > 0 && base_ms > 0 && base_ms <= cap_ms);
        Self { max_attempts, base_ms, cap_ms }
    }

    pub fn delays(&self) -> impl Iterator<Item = RetryDelay> + '_ {
        (1..=self.max_attempts).map(move |attempt| RetryDelay {
            attempt,
            milliseconds: self.delay_for(attempt),
        })
    }

    pub fn delay_for(&self, attempt: u32) -> u64 {
        if attempt == 0 { return 0; }
        let shift = attempt.saturating_sub(1).min(63);
        self.base_ms.saturating_mul(1u64 << shift).min(self.cap_ms)
    }
}
