#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    pub fn new(start: i32, end: i32) -> Self {
        assert!(start <= end, "interval must be ordered");
        Self { start, end }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

pub fn merge(mut ranges: Vec<Interval>) -> Vec<Interval> {
    ranges.sort_by_key(|range| range.start);
    let mut merged = Vec::new();
    for range in ranges {
        match merged.last_mut() {
            Some(last) if range.start <= last.end + 1 => last.end = last.end.max(range.end),
            _ => merged.push(range),
        }
    }
    merged
}

fn main() {
    let ranges = vec![Interval::new(8, 10), Interval::new(1, 3),
        Interval::new(2, 6), Interval::new(12, 13)];
    println!("{:?}", merge(ranges));
}
