/// Finds the first sorted value that satisfies a monotonic predicate.
pub fn first_at_least<T: Ord>(values: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = values.len();
    while low < high {
        let middle = low + (high - low) / 2;
        if &values[middle] < target {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    (low < values.len()).then_some(low)
}

/// Returns the range of indexes containing exactly `target`.
pub fn equal_range<T: Ord>(values: &[T], target: &T) -> Option<(usize, usize)> {
    let start = first_at_least(values, target)?;
    if &values[start] != target { return None; }
    let mut end = start + 1;
    while end < values.len() && &values[end] == target { end += 1; }
    Some((start, end))
}

fn main() {
    let values = [1, 2, 2, 2, 5, 9];
    println!("{:?}", equal_range(&values, &2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn locates_duplicates() {
        assert_eq!(equal_range(&[1, 2, 2, 5], &2), Some((1, 3)));
    }
    #[test]
    fn reports_missing_values() {
        assert_eq!(first_at_least(&[1, 3, 7], &4), Some(2));
        assert_eq!(equal_range(&[1, 3, 7], &4), None);
    }
}
