//! Fixed-capacity rotating buffer with predictable overwrite semantics.

#[derive(Debug, Clone)]
pub struct RotatingBuffer<T> {
    slots: Vec<Option<T>>,
    next: usize,
}

impl<T> RotatingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        Self { slots: (0..capacity).map(|_| None).collect(), next: 0 }
    }

    pub fn push(&mut self, value: T) -> Option<T> {
        let replaced = self.slots[self.next].take();
        self.slots[self.next] = Some(value);
        self.next = (self.next + 1) % self.slots.len();
        replaced
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.slots.iter().filter_map(Option::as_ref)
    }

    pub fn len(&self) -> usize {
        self.slots.iter().filter(|slot| slot.is_some()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::RotatingBuffer;

    #[test]
    fn overwrites_oldest_slot() {
        let mut buffer = RotatingBuffer::new(2);
        assert_eq!(buffer.push("a"), None);
        assert_eq!(buffer.push("b"), None);
        assert_eq!(buffer.push("c"), Some("a"));
        assert_eq!(buffer.len(), 2);
    }
}
