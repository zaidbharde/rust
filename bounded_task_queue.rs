//! A priority-aware bounded queue suitable for small worker pools.
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
pub struct Task { pub priority: u8, pub sequence: u64, pub name: String }

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).then_with(|| other.sequence.cmp(&self.sequence))
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

#[derive(Debug)]
pub struct TaskQueue { capacity: usize, next_sequence: u64, tasks: BinaryHeap<Task> }

impl TaskQueue {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        Self { capacity, next_sequence: 0, tasks: BinaryHeap::new() }
    }

    pub fn push(&mut self, priority: u8, name: impl Into<String>) -> Result<(), Task> {
        if self.tasks.len() == self.capacity {
            return Err(Task { priority, sequence: self.next_sequence, name: name.into() });
        }
        self.tasks.push(Task { priority, sequence: self.next_sequence, name: name.into() });
        self.next_sequence += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Task> { self.tasks.pop() }
    pub fn len(&self) -> usize { self.tasks.len() }
    pub fn is_empty(&self) -> bool { self.tasks.is_empty() }
    pub fn remaining(&self) -> usize { self.capacity - self.tasks.len() }
}
