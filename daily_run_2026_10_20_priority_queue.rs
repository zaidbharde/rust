//! Stable bounded priority queue using a binary heap and sequence numbers.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Entry<T> { priority: u32, sequence: u64, value: T }

impl<T: Eq> Ord for Entry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
            .then_with(|| other.sequence.cmp(&self.sequence))
    }
}

impl<T: Eq> PartialOrd for Entry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

pub struct BoundedPriorityQueue<T> {
    heap: BinaryHeap<Entry<T>>,
    capacity: usize,
    next_sequence: u64,
}

impl<T: Eq> BoundedPriorityQueue<T> {
    pub fn new(capacity: usize) -> Self { Self { heap: BinaryHeap::new(), capacity, next_sequence: 0 } }

    pub fn push(&mut self, priority: u32, value: T) -> Option<T> {
        if self.capacity == 0 { return Some(value); }
        self.heap.push(Entry { priority, sequence: self.next_sequence, value });
        self.next_sequence += 1;
        if self.heap.len() > self.capacity { self.heap.pop().map(|entry| entry.value) } else { None }
    }

    pub fn pop(&mut self) -> Option<(u32, T)> { self.heap.pop().map(|entry| (entry.priority, entry.value)) }
    pub fn len(&self) -> usize { self.heap.len() }
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }
}
