//! Priority queue whose equal-priority items retain insertion order.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Entry<T> {
    priority: i32,
    sequence: u64,
    value: T,
}

impl<T> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool { self.priority == other.priority && self.sequence == other.sequence }
}
impl<T> Eq for Entry<T> {}
impl<T> PartialOrd for Entry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for Entry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).then_with(|| other.sequence.cmp(&self.sequence))
    }
}

#[derive(Debug, Default)]
pub struct StablePriorityQueue<T> {
    heap: BinaryHeap<Entry<T>>,
    next_sequence: u64,
}

impl<T> StablePriorityQueue<T> {
    pub fn push(&mut self, priority: i32, value: T) {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.heap.push(Entry { priority, sequence, value });
    }

    pub fn pop(&mut self) -> Option<(i32, T)> {
        self.heap.pop().map(|entry| (entry.priority, entry.value))
    }

    pub fn peek_priority(&self) -> Option<i32> { self.heap.peek().map(|entry| entry.priority) }
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }
    pub fn len(&self) -> usize { self.heap.len() }
}
