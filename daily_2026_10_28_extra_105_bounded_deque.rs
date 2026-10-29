//! Fixed-capacity double-ended queue backed by a manually managed ring buffer.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapacityError { Zero, AllocationFailed }

#[derive(Debug)]
pub struct BoundedDeque<T> {
    slots: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> BoundedDeque<T> {
    pub fn try_with_capacity(capacity: usize) -> Result<Self, CapacityError> {
        if capacity == 0 { return Err(CapacityError::Zero); }
        let mut slots = Vec::new();
        slots.try_reserve_exact(capacity).map_err(|_| CapacityError::AllocationFailed)?;
        slots.resize_with(capacity, || None);
        Ok(Self { slots, head: 0, len: 0 })
    }

    fn index(&self, offset: usize) -> usize {
        let remaining = self.slots.len() - self.head;
        if offset < remaining { self.head + offset } else { offset - remaining }
    }

    pub fn capacity(&self) -> usize { self.slots.len() }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn is_full(&self) -> bool { self.len == self.capacity() }
    pub fn front(&self) -> Option<&T> { if self.is_empty() { None } else { self.slots[self.head].as_ref() } }
    pub fn back(&self) -> Option<&T> { if self.is_empty() { None } else { self.slots[self.index(self.len - 1)].as_ref() } }

    pub fn try_push_back(&mut self, value: T) -> Result<(), T> {
        if self.is_full() { return Err(value); }
        let i = self.index(self.len);
        self.slots[i] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn try_push_front(&mut self, value: T) -> Result<(), T> {
        if self.is_full() { return Err(value); }
        self.head = if self.head == 0 { self.capacity() - 1 } else { self.head - 1 };
        self.slots[self.head] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let value = self.slots[self.head].take();
        self.head = if self.head + 1 == self.capacity() { 0 } else { self.head + 1 };
        self.len -= 1;
        Some(value.expect("occupied deque slot"))
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let index = self.index(self.len - 1);
        let value = self.slots[index].take();
        self.len -= 1;
        Some(value.expect("occupied deque slot"))
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundedDeque, CapacityError};

    #[test]
    fn rejects_invalid_capacity_and_full_pushes() {
        assert_eq!(BoundedDeque::<i32>::try_with_capacity(0).err(), Some(CapacityError::Zero));
        let mut d = BoundedDeque::try_with_capacity(2).unwrap();
        d.try_push_back(1).unwrap(); d.try_push_front(0).unwrap();
        assert_eq!(d.try_push_back(2), Err(2));
        assert_eq!((d.pop_back(), d.pop_front(), d.pop_front()), (Some(1), Some(0), None));
    }

    #[test]
    fn wraps_and_preserves_both_ends() {
        let mut d = BoundedDeque::try_with_capacity(3).unwrap();
        d.try_push_back(1).unwrap(); d.try_push_back(2).unwrap();
        assert_eq!(d.pop_front(), Some(1));
        d.try_push_back(3).unwrap(); d.try_push_front(0).unwrap();
        assert_eq!((*d.front().unwrap(), *d.back().unwrap()), (0, 3));
    }
}
