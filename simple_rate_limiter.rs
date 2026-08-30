use std::time::{Duration, Instant};

struct RateLimiter { limit: usize, window: Duration, hits: Vec<Instant> }

impl RateLimiter {
    fn new(limit: usize, window: Duration) -> Self {
        Self { limit, window, hits: Vec::new() }
    }

    fn allow(&mut self, now: Instant) -> bool {
        self.hits.retain(|time| now.duration_since(*time) < self.window);
        if self.hits.len() >= self.limit { return false; }
        self.hits.push(now);
        true
    }
}

fn main() {
    let now = Instant::now();
    let mut limiter = RateLimiter::new(2, Duration::from_secs(1));
    println!("{} {} {}", limiter.allow(now), limiter.allow(now), limiter.allow(now));
}
