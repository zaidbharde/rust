//! A fixed-capacity, LIFO stack with checked push and pop operations.

/// Failure returned when a checked stack operation cannot be completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackError {
    Full,
    Empty,
}

/// A stack whose length can never exceed its construction-time capacity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedStack<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> BoundedStack<T> {
    /// Creates an empty stack with the given maximum number of elements.
    pub fn new(capacity: usize) -> Self {
        Self { items: Vec::with_capacity(capacity), capacity }
    }

    pub fn capacity(&self) -> usize { self.capacity }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn is_full(&self) -> bool { self.items.len() == self.capacity }

    /// Adds a value unless the stack has reached capacity.
    pub fn push(&mut self, value: T) -> Result<(), StackError> {
        if self.is_full() { return Err(StackError::Full); }
        self.items.push(value);
        Ok(())
    }

    /// Removes and returns the most recently pushed value, or `Empty`.
    pub fn pop(&mut self) -> Result<T, StackError> {
        self.items.pop().ok_or(StackError::Empty)
    }

    /// Borrows the top value without removing it.
    pub fn peek(&self) -> Result<&T, StackError> {
        self.items.last().ok_or(StackError::Empty)
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundedStack, StackError};

    #[test]
    fn checked_lifo_demo_and_capacity_edges() {
        let mut stack = BoundedStack::new(2);
        assert_eq!(stack.pop(), Err(StackError::Empty));
        assert_eq!(stack.peek(), Err(StackError::Empty));
        stack.push("first").unwrap();
        stack.push("second").unwrap();
        assert_eq!(stack.push("overflow"), Err(StackError::Full));
        assert_eq!(stack.peek(), Ok(&"second"));
        assert_eq!(stack.pop(), Ok("second"));
        assert_eq!(stack.pop(), Ok("first"));
        assert!(stack.is_empty());
    }

    #[test]
    fn zero_capacity_is_always_full() {
        let mut stack: BoundedStack<i32> = BoundedStack::new(0);
        assert!(stack.is_full());
        assert_eq!(stack.push(1), Err(StackError::Full));
        assert_eq!(stack.len(), 0);
    }
}
