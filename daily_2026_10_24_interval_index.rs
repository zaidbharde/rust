//! Query overlapping closed intervals with a compact sorted index.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
    pub id: usize,
}

pub fn build_index(mut intervals: Vec<Interval>) -> Result<Vec<Interval>, &'static str> {
    if intervals.iter().any(|item| item.start > item.end) {
        return Err("interval start cannot exceed end");
    }
    intervals.sort_by_key(|item| (item.start, item.end, item.id));
    Ok(intervals)
}

pub fn overlaps(index: &[Interval], start: i32, end: i32) -> Vec<usize> {
    if start > end {
        return Vec::new();
    }
    index.iter()
        .take_while(|item| item.start <= end)
        .filter(|item| item.end >= start)
        .map(|item| item.id)
        .collect()
}

fn main() {
    let index = build_index(vec![
        Interval { start: 10, end: 14, id: 2 },
        Interval { start: 2, end: 5, id: 1 },
        Interval { start: 6, end: 9, id: 3 },
    ]).unwrap();
    assert_eq!(overlaps(&index, 4, 7), vec![1, 3]);
    assert!(overlaps(&index, 20, 22).is_empty());
    println!("indexed intervals: {index:?}");
}
