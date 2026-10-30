/// Return inclusive prefix sums, panicking with a clear message if an i64 sum overflows.
pub fn prefix_sums(values: &[i64]) -> Vec<i64> {
    let mut sum = 0_i64;
    values
        .iter()
        .map(|&value| {
            sum = sum.checked_add(value).expect("prefix sum overflow");
            sum
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inclusive() {
        assert_eq!(prefix_sums(&[2, -1, 4]), vec![2, 1, 5]);
    }

    #[test]
    #[should_panic(expected = "prefix sum overflow")]
    fn positive_overflow_is_explicit() {
        prefix_sums(&[i64::MAX, 1]);
    }

    #[test]
    #[should_panic(expected = "prefix sum overflow")]
    fn negative_overflow_is_explicit() {
        prefix_sums(&[i64::MIN, -1]);
    }
}
