//! Return the bounds and sum of a maximum-sum nonempty contiguous subarray.
use std::ops::Range;

/// Finds a maximum-sum nonempty subarray, returning its half-open bounds and sum.
/// Ties keep the earliest subarray encountered. Empty input and sum overflow are errors.
pub fn kadane_subarray(values: &[i64]) -> Result<(Range<usize>, i64), &'static str> {
    let (&first, rest) = values.split_first().ok_or("input must not be empty")?;
    let (mut best_sum, mut current_sum) = (first, first);
    let (mut best_start, mut best_end, mut current_start) = (0, 1, 0);

    for (index, &value) in rest.iter().enumerate() {
        let end = index + 2;
        let extended = current_sum.checked_add(value).ok_or("subarray sum overflow")?;
        if value > extended {
            current_sum = value;
            current_start = end - 1;
        } else {
            current_sum = extended;
        }
        if current_sum > best_sum {
            best_sum = current_sum;
            best_start = current_start;
            best_end = end;
        }
    }
    Ok((best_start..best_end, best_sum))
}

#[cfg(test)]
mod tests {
    use super::kadane_subarray;

    #[test]
    fn finds_maximum_and_handles_negative_values() {
        assert_eq!(kadane_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Ok((3..7, 6)));
        assert_eq!(kadane_subarray(&[-8, -3, -6]), Ok((1..2, -3)));
        assert_eq!(kadane_subarray(&[0, 0]), Ok((0..1, 0)));
    }

    #[test]
    fn validates_empty_input_and_overflow() {
        assert_eq!(kadane_subarray(&[]), Err("input must not be empty"));
        assert_eq!(kadane_subarray(&[i64::MAX, 1]), Err("subarray sum overflow"));
    }
}
