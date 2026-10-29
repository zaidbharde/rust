//! Stable merge sort that returns source indices without moving source records.

use std::cmp::Ordering;

/// Returns indices in stable sorted order according to `compare`.
/// Equal elements retain their original input order; the source slice is untouched.
pub fn merge_sort_indices<T, F>(values: &[T], mut compare: F) -> Vec<usize>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = values.len();
    let mut indices: Vec<usize> = (0..len).collect();
    if len < 2 {
        return indices;
    }

    let mut scratch = indices.clone();
    let mut width = 1;
    while width < len {
        let mut start = 0;
        while start < len {
            let mid = start.saturating_add(width).min(len);
            let end = mid.saturating_add(width).min(len);
            let (mut left, mut right, mut out) = (start, mid, start);
            while left < mid && right < end {
                let take_left = compare(&values[indices[left]], &values[indices[right]]) != Ordering::Greater;
                if take_left {
                    scratch[out] = indices[left];
                    left += 1;
                } else {
                    scratch[out] = indices[right];
                    right += 1;
                }
                out += 1;
            }
            while left < mid {
                scratch[out] = indices[left];
                left += 1;
                out += 1;
            }
            while right < end {
                scratch[out] = indices[right];
                right += 1;
                out += 1;
            }
            start = end;
        }
        std::mem::swap(&mut indices, &mut scratch);
        width = width.saturating_mul(2);
        if width == usize::MAX && width < len {
            width = len;
        }
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::merge_sort_indices;

    #[test]
    fn sorts_indices_and_preserves_source() {
        let values = [4, 1, 3, 1, 2];
        assert_eq!(merge_sort_indices(&values, Ord::cmp), [1, 3, 4, 2, 0]);
        assert_eq!(values, [4, 1, 3, 1, 2]);
    }

    #[test]
    fn preserves_equal_item_order() {
        let values = [(2, "a"), (1, "b"), (2, "c"), (1, "d")];
        let order = merge_sort_indices(&values, |a, b| a.0.cmp(&b.0));
        assert_eq!(order, [1, 3, 0, 2]);
        assert_eq!(values[order[0]].1, "b");
        assert_eq!(values[order[1]].1, "d");
    }

    #[test]
    fn handles_empty_singleton_and_custom_comparison() {
        assert!(merge_sort_indices::<i32, _>(&[], Ord::cmp).is_empty());
        assert_eq!(merge_sort_indices(&[7], Ord::cmp), [0]);
        assert_eq!(merge_sort_indices(&[1, 3, 2], |a, b| b.cmp(a)), [1, 2, 0]);
    }
}
