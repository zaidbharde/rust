//! Select the largest `k` values while keeping equal values in input order.

/// Returns references to at most `k` greatest values, descending by value.
/// Equal values retain their original order; `k == 0` returns an empty vector.
pub fn stable_top_k<T: Ord>(values: &[T], k: usize) -> Vec<&T> {
    let mut indices: Vec<usize> = (0..values.len()).collect();
    indices.sort_by(|&left, &right| values[right].cmp(&values[left]));
    indices.truncate(k);
    indices.into_iter().map(|index| &values[index]).collect()
}

#[cfg(test)]
mod tests {
    use super::stable_top_k;
    use std::cmp::Ordering;

    #[derive(Debug, PartialEq, Eq)]
    struct Entry {
        score: i32,
        label: &'static str,
    }

    impl Ord for Entry {
        fn cmp(&self, other: &Self) -> Ordering {
            self.score.cmp(&other.score)
        }
    }

    impl PartialOrd for Entry {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[test]
    fn selects_largest_in_descending_order() {
        let values = [4, 9, 2, 7, 5];
        assert_eq!(stable_top_k(&values, 3), [&9, &7, &5]);
    }

    #[test]
    fn preserves_input_order_for_equal_values() {
        let values = [
            Entry { score: 5, label: "first" },
            Entry { score: 8, label: "high" },
            Entry { score: 5, label: "second" },
            Entry { score: 5, label: "third" },
        ];
        let selected = stable_top_k(&values, 3);
        assert_eq!(selected.iter().map(|entry| entry.label).collect::<Vec<_>>(),
            ["high", "first", "second"]);
    }

    #[test]
    fn handles_empty_zero_and_oversized_requests() {
        let values = [3, 1];
        assert!(stable_top_k::<i32>(&[], 4).is_empty());
        assert!(stable_top_k(&values, 0).is_empty());
        assert_eq!(stable_top_k(&values, usize::MAX), [&3, &1]);
    }
}
