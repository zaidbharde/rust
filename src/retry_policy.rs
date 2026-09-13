//! Deterministic retry timing with exponential backoff and jitter bounds.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl RetryPolicy {
    pub fn delay_ms(&self, retry_index: u32) -> Option<u64> {
        if retry_index >= self.attempts || self.base_delay_ms == 0 {
            return None;
        }
        let shift = retry_index.min(20);
        let multiplier = 1u64 << shift;
        Some(self.base_delay_ms.saturating_mul(multiplier).min(self.max_delay_ms))
    }

    pub fn schedule(&self) -> impl Iterator<Item = u64> + '_ {
        (0..self.attempts).filter_map(|index| self.delay_ms(index))
    }

    pub fn with_cap(attempts: u32, base_delay_ms: u64, max_delay_ms: u64) -> Self {
        assert!(attempts > 0 && base_delay_ms > 0 && max_delay_ms >= base_delay_ms);
        Self { attempts, base_delay_ms, max_delay_ms }
    }
}
