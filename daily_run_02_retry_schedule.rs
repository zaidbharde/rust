use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPlan { pub attempts: u32, pub delay: Duration }

pub fn exponential_backoff(attempt: u32, base: Duration, cap: Duration) -> Duration {
    let multiplier = 1u32.checked_shl(attempt.min(31)).unwrap_or(u32::MAX);
    let candidate = base.checked_mul(multiplier).unwrap_or(cap);
    candidate.min(cap)
}

pub fn plan_retries(max_attempts: u32, base: Duration, cap: Duration) -> Vec<RetryPlan> {
    (0..max_attempts)
        .map(|attempt| RetryPlan {
            attempts: attempt + 1,
            delay: exponential_backoff(attempt, base, cap),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delays_grow_until_the_cap() {
        let plan = plan_retries(4, Duration::from_secs(2), Duration::from_secs(5));
        assert_eq!(plan[0].delay, Duration::from_secs(2));
        assert_eq!(plan[1].delay, Duration::from_secs(4));
        assert_eq!(plan[2].delay, Duration::from_secs(5));
        assert_eq!(plan[3].delay, Duration::from_secs(5));
    }
}
