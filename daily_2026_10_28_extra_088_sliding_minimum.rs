//! Compute the minimum of every fixed-width window in linear time.
/// Returns references to each window's minimum, preserving the input order.
/// A zero-width window or a width larger than the input returns an error.
pub fn sliding_minimum<T: Ord>(values: &[T], width: usize) -> Result<Vec<&T>, &'static str> {
    if width == 0 {
        return Err("window width must be positive");
    }
    if width > values.len() {
        return Err("window width exceeds input length");
    }

    let mut candidates: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    let mut minima = Vec::with_capacity(values.len() - width + 1);
    for i in 0..values.len() {
        while candidates.front().is_some_and(|&index| index + width <= i) {
            candidates.pop_front();
        }
        while candidates.back().is_some_and(|&index| values[index] >= values[i]) {
            candidates.pop_back();
        }
        candidates.push_back(i);
        if i + 1 >= width {
            minima.push(&values[*candidates.front().expect("current window is nonempty")]);
        }
    }
    Ok(minima)
}

#[cfg(test)]
mod tests {
    use super::sliding_minimum;

    #[test]
    fn computes_window_minima() {
        assert_eq!(sliding_minimum(&[4, 2, 12, 3, 1, 5], 3).unwrap(), [&2, &2, &1, &1]);
        assert_eq!(sliding_minimum(&[3, 3, 3], 2).unwrap(), [&3, &3]);
        assert_eq!(sliding_minimum(&[5, 4, 2], 1).unwrap(), [&5, &4, &2]);
    }

    #[test]
    fn validates_width_and_handles_full_window() {
        assert_eq!(sliding_minimum(&[7, 1, 9], 3).unwrap(), [&1]);
        assert!(sliding_minimum::<i32>(&[], 1).is_err());
        assert!(sliding_minimum(&[1], 0).is_err());
        assert!(sliding_minimum(&[1, 2], 3).is_err());
    }
}
