//! Sorted interval index with overlap merging and point lookup.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: i64,
    pub end: i64,
}

impl Interval {
    pub fn new(start: i64, end: i64) -> Self {
        assert!(start <= end, "interval bounds must be ordered");
        Self { start, end }
    }

    pub fn contains(&self, point: i64) -> bool {
        self.start <= point && point <= self.end
    }
}

#[derive(Debug, Default)]
pub struct IntervalIndex {
    ranges: Vec<Interval>,
}

impl IntervalIndex {
    pub fn insert(&mut self, incoming: Interval) {
        self.ranges.push(incoming);
        self.ranges.sort_by_key(|range| range.start);
        let mut merged = Vec::with_capacity(self.ranges.len());
        for range in self.ranges.drain(..) {
            match merged.last_mut() {
                Some(last) if range.start <= last.end + 1 => last.end = last.end.max(range.end),
                _ => merged.push(range),
            }
        }
        self.ranges = merged;
    }

    pub fn contains(&self, point: i64) -> bool {
        self.ranges.binary_search_by(|range| {
            if range.contains(point) { std::cmp::Ordering::Equal }
            else if point < range.start { std::cmp::Ordering::Greater }
            else { std::cmp::Ordering::Less }
        }).is_ok()
    }

    pub fn ranges(&self) -> &[Interval] { &self.ranges }
}
