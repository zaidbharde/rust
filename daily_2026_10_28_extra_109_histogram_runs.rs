//! Compress a sorted slice into adjacent value-count runs and expand those runs.

/// Summarizes a nondecreasing slice as `(value, count)` pairs.
/// Returns an error if the input is not sorted.
pub fn histogram_runs<T: Ord + Clone>(values: &[T]) -> Result<Vec<(T, usize)>, &'static str> {
    if values.windows(2).any(|pair| pair[0] > pair[1]) {
        return Err("input must be sorted in nondecreasing order");
    }
    let mut runs = Vec::new();
    for value in values {
        if let Some((last, count)) = runs.last_mut() {
            if last == value {
                *count += 1;
                continue;
            }
        }
        runs.push((value.clone(), 1));
    }
    Ok(runs)
}

/// Expands value-count runs, returning `None` if length arithmetic or allocation fails.
pub fn expand_histogram_runs<T: Clone>(runs: &[(T, usize)]) -> Option<Vec<T>> {
    let total = runs.iter().try_fold(0usize, |sum, (_, count)| sum.checked_add(*count))?;
    let mut values = Vec::new();
    values.try_reserve(total).ok()?;
    for (value, count) in runs {
        values.extend(std::iter::repeat(value.clone()).take(*count));
    }
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::{expand_histogram_runs, histogram_runs};

    #[test]
    fn summarizes_and_expands() {
        let input = [-3, -3, 0, 2, 2, 2];
        let runs = histogram_runs(&input).unwrap();
        assert_eq!(runs, vec![(-3, 2), (0, 1), (2, 3)]);
        assert_eq!(expand_histogram_runs(&runs), Some(input.to_vec()));
    }

    #[test]
    fn handles_empty_singleton_and_unsorted_input() {
        assert_eq!(histogram_runs::<i32>(&[]).unwrap(), Vec::<(i32, usize)>::new());
        assert_eq!(histogram_runs(&[7]).unwrap(), vec![(7, 1)]);
        assert!(histogram_runs(&[1, 3, 2]).is_err());
    }

    #[test]
    fn zero_counts_expand_to_empty() {
        assert_eq!(expand_histogram_runs::<u8>(&[(4, 0), (9, 0)]), Some(vec![]));
    }
}
