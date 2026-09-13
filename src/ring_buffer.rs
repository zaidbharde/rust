//! Fixed-capacity FIFO ring buffer with overwrite-on-full semantics.

#[derive(Debug, PartialEq, Eq)]
pub struct RingBuffer<T> {
    slots: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        Self { slots: (0..capacity).map(|_| None).collect(), head: 0, len: 0 }
    }

    pub fn push(&mut self, value: T) {
        let index = (self.head + self.len) % self.slots.len();
        self.slots[index] = Some(value);
        if self.len == self.slots.len() {
            self.head = (self.head + 1) % self.slots.len();
        } else {
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let value = self.slots[self.head].take();
        self.head = (self.head + 1) % self.slots.len();
        self.len -= 1;
        value
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn capacity(&self) -> usize { self.slots.len() }
}
