//! A LIFO stack with constant-time minimum and maximum queries.

/// Stack entries retain the extrema of the prefix ending at that entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinMaxStack<T> {
    entries: Vec<(T, T, T)>, // value, minimum, maximum
}

impl<T: Ord + Clone> MinMaxStack<T> {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }

    pub fn push(&mut self, value: T) {
        let (minimum, maximum) = match self.entries.last() {
            Some(entry) => (
                if value < entry.1 { value.clone() } else { entry.1.clone() },
                if value > entry.2 { value.clone() } else { entry.2.clone() },
            ),
            None => (value.clone(), value.clone()),
        };
        self.entries.push((value, minimum, maximum));
    }

    pub fn pop(&mut self) -> Option<T> { self.entries.pop().map(|entry| entry.0) }
    pub fn peek(&self) -> Option<&T> { self.entries.last().map(|entry| &entry.0) }
    pub fn min(&self) -> Option<&T> { self.entries.last().map(|entry| &entry.1) }
    pub fn max(&self) -> Option<&T> { self.entries.last().map(|entry| &entry.2) }
}

impl<T: Ord + Clone> Default for MinMaxStack<T> {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::MinMaxStack;

    #[test]
    fn demo_tracks_extrema_across_push_and_pop() {
        let mut stack = MinMaxStack::new();
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.min(), None);
        assert_eq!(stack.max(), None);
        for value in [3, 1, 5, 1] { stack.push(value); }
        assert_eq!((stack.len(), stack.peek(), stack.min(), stack.max()), (4, Some(&1), Some(&1), Some(&5)));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!((stack.min(), stack.max()), (Some(&1), Some(&5)));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!((stack.min(), stack.max()), (Some(&1), Some(&3)));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!((stack.min(), stack.max()), (Some(&3), Some(&3)));
        assert_eq!(stack.pop(), Some(3));
        assert!(stack.is_empty());
        stack.push(7);
        assert_eq!((stack.min(), stack.max()), (Some(&7), Some(&7)));
    }
}
