use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct EventWindow {
    width: u64,
    events: VecDeque<u64>,
}

impl EventWindow {
    pub fn new(width: u64) -> Self {
        assert!(width > 0, "window width must be positive");
        Self { width, events: VecDeque::new() }
    }

    pub fn record(&mut self, timestamp: u64) {
        while self.events.front().is_some_and(|old| timestamp.saturating_sub(*old) >= self.width) {
            self.events.pop_front();
        }
        self.events.push_back(timestamp);
    }

    pub fn count_at(&mut self, timestamp: u64) -> usize {
        while self.events.front().is_some_and(|old| timestamp.saturating_sub(*old) >= self.width) {
            self.events.pop_front();
        }
        self.events.len()
    }
}

fn main() {
    let mut window = EventWindow::new(60);
    for second in [10, 20, 55, 75] { window.record(second); }
    println!("{}", window.count_at(80));
}
