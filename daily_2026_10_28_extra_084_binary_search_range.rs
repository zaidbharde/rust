//! Find the half-open range occupied by a target in a sorted slice.
use std::ops::Range;

/// Returns `start..end` for all values equal to `target`, or an empty insertion
/// range when absent. Returns an error if `values` is not sorted nondecreasingly.
pub fn binary_search_range<T: Ord>(values: &[T], target: &T) -> Result<Range<usize>, &'static str> {
    if values.windows(2).any(|pair| pair[0] > pair[1]) {
        return Err("input must be sorted in nondecreasing order");
    }
    let lower = |mut lo: usize, mut hi: usize, upper: bool| {
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if values[mid] < *target || (upper && values[mid] == *target) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    };
    let start = lower(0, values.len(), false);
    let end = lower(start, values.len(), true);
    Ok(start..end)
}

#[cfg(test)]
mod tests {
    use super::binary_search_range;

    #[test]
    fn demo_finds_duplicates_and_absent_insertion_points() {
        let values = [1, 2, 2, 2, 4, 7];
        assert_eq!(binary_search_range(&values, &2), Ok(1..4));
        assert_eq!(binary_search_range(&values, &3), Ok(4..4));
        assert_eq!(binary_search_range(&values, &0), Ok(0..0));
        assert_eq!(binary_search_range(&values, &9), Ok(6..6));
    }

    #[test]
    fn handles_empty_singleton_and_unsorted_inputs() {
        assert_eq!(binary_search_range::<i32>(&[], &5), Ok(0..0));
        assert_eq!(binary_search_range(&[5], &5), Ok(0..1));
        assert_eq!(binary_search_range(&[5], &3), Ok(0..0));
        assert_eq!(binary_search_range(&[1, 3, 2], &2), Err("input must be sorted in nondecreasing order"));
    }
}
