use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Cursor {
    value: i32,
    stream: usize,
    offset: usize,
}

impl Ord for Cursor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.value, self.stream, self.offset).cmp(&(other.value, other.stream, other.offset))
    }
}

impl PartialOrd for Cursor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn merge_sorted(streams: &[Vec<i32>]) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for (stream, values) in streams.iter().enumerate() {
        if let Some(&value) = values.first() {
            heap.push(Reverse(Cursor { value, stream, offset: 0 }));
        }
    }
    let mut merged = Vec::new();
    while let Some(Reverse(cursor)) = heap.pop() {
        merged.push(cursor.value);
        let next_offset = cursor.offset + 1;
        if let Some(&value) = streams[cursor.stream].get(next_offset) {
            heap.push(Reverse(Cursor { value, stream: cursor.stream, offset: next_offset }));
        }
    }
    merged
}

fn main() {
    let input = vec![vec![1, 4, 9], vec![2, 3, 8], vec![0, 7, 10]];
    println!("{:?}", merge_sorted(&input));
}
