//! Fixed-capacity FIFO ring buffer with overwrite-on-full semantics.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RingBuffer<T> {
    slots: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0, "ring buffer capacity must be positive");
        Self { slots: (0..capacity).map(|_| None).collect(), head: 0, len: 0 }
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn capacity(&self) -> usize { self.slots.len() }

    pub fn push(&mut self, value: T) -> Option<T> {
        let index = (self.head + self.len) % self.capacity();
        let evicted = self.slots[index].replace(value);
        if self.len == self.capacity() {
            self.head = (self.head + 1) % self.capacity();
        } else {
            self.len += 1;
        }
        evicted
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let value = self.slots[self.head].take();
        self.head = (self.head + 1) % self.capacity();
        self.len -= 1;
        value
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.len).filter_map(move |offset| {
            let index = (self.head + offset) % self.capacity();
            self.slots[index].as_ref()
        })
    }
}
