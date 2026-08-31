use std::collections::VecDeque;

pub struct RingBuffer<T> {
    capacity: usize,
    values: VecDeque<T>,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        Self { capacity, values: VecDeque::with_capacity(capacity) }
    }

    pub fn push(&mut self, value: T) {
        if self.values.len() == self.capacity {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }

    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
    pub fn oldest(&self) -> Option<&T> { self.values.front() }
    pub fn newest(&self) -> Option<&T> { self.values.back() }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }
}

fn main() {
    let mut history = RingBuffer::new(3);
    for event in ["open", "write", "flush", "close"] {
        history.push(event);
    }
    println!("{} events: {:?} -> {:?}", history.len(), history.oldest(), history.newest());
    println!("{:?}", history.iter().collect::<Vec<_>>());
}
