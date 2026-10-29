//! Find the smallest positive integer absent from a mutable slice.
/// Reorders `values` in place and returns the smallest missing positive integer.
/// Runs in O(n) time with O(1) auxiliary space; values outside `1..=len` are ignored.
pub fn missing_positive(values: &mut [i32]) -> usize {
    let len = values.len();
    for index in 0..len {
        loop {
            let Ok(value) = usize::try_from(values[index]) else {
                break;
            };
            if value == 0 || value > len || values[value - 1] == values[index] {
                break;
            }
            values.swap(index, value - 1);
        }
    }
    values
        .iter()
        .enumerate()
        .position(|(index, &value)| usize::try_from(value).ok() != Some(index + 1))
        .unwrap_or(len) + 1
}
#[cfg(test)]
mod tests {
    use super::missing_positive;

    #[test]
    fn finds_the_first_missing_positive() {
        let mut values = [3, 4, -1, 1];
        assert_eq!(missing_positive(&mut values), 2);
        let mut values = [1, 2, 0];
        assert_eq!(missing_positive(&mut values), 3);
        let mut values = [7, 8, 9, 11, 12];
        assert_eq!(missing_positive(&mut values), 1);
    }

    #[test]
    fn handles_empty_duplicates_and_integer_extremes() {
        let mut empty = [];
        assert_eq!(missing_positive(&mut empty), 1);
        let mut values = [1, 1, 2, 2];
        assert_eq!(missing_positive(&mut values), 3);
        let mut values = [i32::MIN, i32::MAX, 2, 1];
        assert_eq!(missing_positive(&mut values), 3);
    }
}
