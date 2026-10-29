//! Merge sorted slices into one stable, nondecreasing sequence using a binary heap.
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Merges sorted slices, preserving equal-value order by input slice and position.
/// Returns an error if any input slice is not sorted nondecreasingly.
pub fn k_way_merge<T: Ord + Clone>(slices: &[&[T]]) -> Result<Vec<T>, &'static str> {
    if slices.iter().any(|s| s.windows(2).any(|w| w[0] > w[1])) {
        return Err("each input slice must be sorted in nondecreasing order");
    }
    struct Entry<T: Ord> { value: T, source: usize, offset: usize }
    impl<T: Ord> PartialEq for Entry<T> {
        fn eq(&self, other: &Self) -> bool {
            (&self.value, self.source, self.offset) == (&other.value, other.source, other.offset)
        }
    }
    impl<T: Ord> Eq for Entry<T> {}
    impl<T: Ord> Ord for Entry<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.value.cmp(&self.value)
                .then_with(|| other.source.cmp(&self.source))
                .then_with(|| other.offset.cmp(&self.offset))
        }
    }
    impl<T: Ord> PartialOrd for Entry<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }
    let mut heap = BinaryHeap::new();
    let capacity: usize = slices.iter().map(|s| s.len()).sum();
    let mut merged = Vec::with_capacity(capacity);
    for (source, slice) in slices.iter().enumerate() {
        if let Some(value) = slice.first() {
            heap.push(Entry { value: value.clone(), source, offset: 0 });
        }
    }
    while let Some(entry) = heap.pop() {
        merged.push(entry.value);
        let next = entry.offset + 1;
        if let Some(value) = slices[entry.source].get(next) {
            heap.push(Entry { value: value.clone(), source: entry.source, offset: next });
        }
    }
    Ok(merged)
}

#[cfg(test)]
mod tests {
    use super::k_way_merge;

    #[test]
    fn merges_sorted_inputs_and_handles_edges() {
        let a = [1, 2, 2, 8];
        let b = [1, 3, 7];
        let c: [i32; 0] = [];
        assert_eq!(k_way_merge(&[&a, &b, &c]).unwrap(), [1, 1, 2, 2, 3, 7, 8]);
        assert_eq!(k_way_merge::<i32>(&[]).unwrap(), []);
        assert_eq!(k_way_merge(&[&c]).unwrap(), []);
    }

    #[test]
    fn rejects_unsorted_slice() {
        assert_eq!(k_way_merge(&[&[1, 3, 2]]), Err("each input slice must be sorted in nondecreasing order"));
    }
}
