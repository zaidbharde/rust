//! Return indices of one longest strictly increasing subsequence.

/// Finds indices of one longest strictly increasing subsequence in `values`.
/// Returns an empty vector for empty input; equal values cannot extend the sequence.
pub fn longest_increasing_subsequence<T: Ord>(values: &[T]) -> Vec<usize> {
    let mut tails: Vec<usize> = Vec::new();
    let mut previous = vec![None; values.len()];

    for i in 0..values.len() {
        let mut low = 0;
        let mut high = tails.len();
        while low < high {
            let mid = low + (high - low) / 2;
            if values[tails[mid]] < values[i] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        if low > 0 {
            previous[i] = Some(tails[low - 1]);
        }
        if low == tails.len() {
            tails.push(i);
        } else {
            tails[low] = i;
        }
    }

    let mut result = Vec::with_capacity(tails.len());
    let mut cursor = tails.last().copied();
    while let Some(i) = cursor {
        result.push(i);
        cursor = previous[i];
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::longest_increasing_subsequence;

    fn is_strict_subsequence<T: Ord>(values: &[T], indices: &[usize]) -> bool {
        indices.windows(2).all(|pair| pair[0] < pair[1] && values[pair[0]] < values[pair[1]])
    }

    #[test]
    fn returns_valid_maximum_length_indices() {
        let values = [10, 9, 2, 5, 3, 7, 101, 18];
        let indices = longest_increasing_subsequence(&values);
        assert_eq!(indices.len(), 4);
        assert!(is_strict_subsequence(&values, &indices));
        let duplicate_values = [1, 2, 2, 3];
        let duplicate_indices = longest_increasing_subsequence(&duplicate_values);
        assert_eq!(duplicate_indices.len(), 3);
        assert!(is_strict_subsequence(&duplicate_values, &duplicate_indices));
    }

    #[test]
    fn handles_empty_singleton_and_nonincreasing_inputs() {
        assert_eq!(longest_increasing_subsequence::<i32>(&[]), Vec::<usize>::new());
        assert_eq!(longest_increasing_subsequence(&[4]), vec![0]);
        let values = [5, 4, 4, 1];
        let indices = longest_increasing_subsequence(&values);
        assert_eq!(indices.len(), 1);
        assert!(is_strict_subsequence(&values, &indices));
    }
}
