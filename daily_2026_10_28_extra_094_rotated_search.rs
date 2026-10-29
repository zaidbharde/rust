/// Finds any index of `target` in a nondecreasing slice rotated at an unknown pivot.
///
/// Returns `None` when the target is absent. Duplicate values are supported; when
/// equal endpoints make the sorted half ambiguous, the search safely shrinks inward.
pub fn rotated_search<T: Ord>(values: &[T], target: &T) -> Option<usize> {
    let (mut left, mut right) = (0, values.len());
    while left < right {
        let mid = left + (right - left) / 2;
        if &values[mid] == target {
            return Some(mid);
        }
        if values[left] == values[mid] && values[mid] == values[right - 1] {
            left += 1;
            right -= 1;
        } else if values[left] <= values[mid] {
            if &values[left] <= target && target < &values[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        } else if &values[mid] < target && target <= &values[right - 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::rotated_search;

    #[test]
    fn finds_values_across_pivot_and_duplicates() {
        for (values, target) in [
            (vec![4, 5, 6, 7, 0, 1, 2], 0),
            (vec![2, 5, 6, 0, 0, 1, 2], 2),
            (vec![1, 1, 1, 3, 1], 3),
            (vec![3, 1, 1, 1, 1], 3),
        ] {
            let found = rotated_search(&values, &target).unwrap();
            assert_eq!(values[found], target);
        }
    }

    #[test]
    fn handles_empty_singleton_and_missing_targets() {
        assert_eq!(rotated_search::<i32>(&[], &1), None);
        assert_eq!(rotated_search(&[7], &7), Some(0));
        assert_eq!(rotated_search(&[7], &2), None);
        assert_eq!(rotated_search(&[3, 1, 2], &4), None);
    }
}
