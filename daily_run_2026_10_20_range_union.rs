//! Merge inclusive integer ranges while preserving sorted, disjoint output.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Range { pub start: i64, pub end: i64 }

impl Range {
    pub fn new(start: i64, end: i64) -> Result<Self, &'static str> {
        if start > end { Err("range start must not exceed end") } else { Ok(Self { start, end }) }
    }

    fn touches(self, other: Self) -> bool {
        self.start <= other.end.saturating_add(1) && other.start <= self.end.saturating_add(1)
    }

    fn merge(self, other: Self) -> Self {
        Self { start: self.start.min(other.start), end: self.end.max(other.end) }
    }
}

pub fn union(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|range| (range.start, range.end));
    let mut result = Vec::with_capacity(ranges.len());
    for current in ranges {
        match result.last_mut() {
            Some(previous) if previous.touches(current) => *previous = previous.merge(current),
            _ => result.push(current),
        }
    }
    result
}

pub fn covered_points(ranges: &[Range]) -> u128 {
    ranges.iter().map(|range| (range.end as i128 - range.start as i128 + 1) as u128).sum()
}
