#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: i64,
    pub end: i64,
}

impl Interval {
    pub fn new(start: i64, end: i64) -> Result<Self, &'static str> {
        if start > end {
            return Err("interval start cannot exceed end");
        }
        Ok(Self { start, end })
    }
}

/// Merge overlapping or directly touching intervals into a sorted, minimal set.
pub fn merge(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return intervals;
    }
    intervals.sort_by_key(|interval| (interval.start, interval.end));
    let mut merged = Vec::with_capacity(intervals.len());
    for current in intervals {
        match merged.last_mut() {
            Some(previous) if current.start <= previous.end.saturating_add(1) => {
                previous.end = previous.end.max(current.end);
            }
            _ => merged.push(current),
        }
    }
    merged
}

fn main() {
    let input = vec![
        Interval::new(8, 10).unwrap(),
        Interval::new(1, 3).unwrap(),
        Interval::new(3, 5).unwrap(),
        Interval::new(12, 14).unwrap(),
    ];
    println!("{:?}", merge(input));
}

#[cfg(test)]
mod tests {
    use super::{merge, Interval};

    #[test]
    fn touching_ranges_are_combined() {
        let ranges = vec![Interval::new(4, 6).unwrap(), Interval::new(1, 3).unwrap()];
        assert_eq!(merge(ranges), vec![Interval::new(1, 6).unwrap()]);
    }
}
