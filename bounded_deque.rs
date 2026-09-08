//! A double-ended queue with a hard item limit and explicit eviction reporting.

#[derive(Debug, Default)]
pub struct BoundedDeque<T> {
    items: std::collections::VecDeque<T>,
    limit: usize,
}

impl<T> BoundedDeque<T> {
    pub fn new(limit: usize) -> Self {
        assert!(limit > 0, "limit must be positive");
        Self { items: std::collections::VecDeque::new(), limit }
    }

    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn front(&self) -> Option<&T> { self.items.front() }
    pub fn back(&self) -> Option<&T> { self.items.back() }

    pub fn push_back(&mut self, value: T) -> Option<T> {
        self.items.push_back(value);
        if self.items.len() > self.limit { self.items.pop_front() } else { None }
    }

    pub fn push_front(&mut self, value: T) -> Option<T> {
        self.items.push_front(value);
        if self.items.len() > self.limit { self.items.pop_back() } else { None }
    }

    pub fn pop_front(&mut self) -> Option<T> { self.items.pop_front() }
    pub fn pop_back(&mut self) -> Option<T> { self.items.pop_back() }

    pub fn retain<F>(&mut self, predicate: F) where F: FnMut(&T) -> bool {
        self.items.retain(predicate);
    }

    pub fn into_vec(self) -> Vec<T> { self.items.into_iter().collect() }
}

#[cfg(test)]
mod tests {
    use super::BoundedDeque;
    #[test]
    fn reports_evicted_end() {
        let mut deque = BoundedDeque::new(2);
        deque.push_back("a"); deque.push_back("b");
        assert_eq!(deque.push_front("c"), Some("b"));
        assert_eq!(deque.into_vec(), vec!["c", "a"]);
    }
}
