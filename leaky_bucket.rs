//! Leaky-bucket admission control with deterministic drain timing.

#[derive(Debug)]
pub struct LeakyBucket {
    capacity: f64,
    leak_per_second: f64,
    queued: f64,
    last_update: f64,
}

impl LeakyBucket {
    pub fn new(capacity: u32, leak_per_second: f64, now: f64) -> Self {
        assert!(capacity > 0 && leak_per_second > 0.0 && now.is_finite());
        Self { capacity: capacity as f64, leak_per_second, queued: 0.0, last_update: now }
    }

    fn drain(&mut self, now: f64) -> bool {
        if !now.is_finite() || now < self.last_update { return false; }
        self.queued = (self.queued - (now - self.last_update) * self.leak_per_second).max(0.0);
        self.last_update = now;
        true
    }

    pub fn enqueue(&mut self, amount: u32, now: f64) -> bool {
        if !self.drain(now) { return false; }
        let requested = amount as f64;
        if self.queued + requested > self.capacity { return false; }
        self.queued += requested;
        true
    }

    pub fn queued(&self) -> u32 { self.queued.ceil() as u32 }
}
