//! A bounded work queue with fair worker rotation and explicit shutdown.
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum PushError<T> { Full(T), Closed(T) }

pub struct FairQueue<T> {
    items: VecDeque<T>,
    capacity: usize,
    closed: bool,
    next_worker: usize,
}

impl<T> FairQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self { items: VecDeque::with_capacity(capacity), capacity, closed: false, next_worker: 0 }
    }

    pub fn push(&mut self, item: T) -> Result<(), PushError<T>> {
        if self.closed { return Err(PushError::Closed(item)); }
        if self.items.len() == self.capacity { return Err(PushError::Full(item)); }
        self.items.push_back(item);
        Ok(())
    }

    pub fn pop_for(&mut self, worker_count: usize, worker: usize) -> Option<T> {
        if worker_count == 0 || worker >= worker_count || self.items.is_empty() { return None; }
        if worker != self.next_worker { return None; }
        self.next_worker = (self.next_worker + 1) % worker_count;
        self.items.pop_front()
    }

    pub fn close(&mut self) { self.closed = true; }
    pub fn is_closed(&self) -> bool { self.closed }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn workers_take_turns() {
        let mut queue = FairQueue::new(3);
        queue.push(10).unwrap();
        queue.push(20).unwrap();
        assert_eq!(queue.pop_for(2, 0), Some(10));
        assert_eq!(queue.pop_for(2, 0), None);
        assert_eq!(queue.pop_for(2, 1), Some(20));
    }
}
