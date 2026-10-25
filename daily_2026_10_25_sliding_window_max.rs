use std::collections::VecDeque;

/// Returns the maximum value for every contiguous window of `width` items.
pub fn sliding_window_max(values: &[i32], width: usize) -> Vec<i32> {
    if width == 0 || width > values.len() {
        return Vec::new();
    }
    let mut candidates = VecDeque::new();
    let mut result = Vec::with_capacity(values.len() - width + 1);
    for index in 0..values.len() {
        while candidates
            .front()
            .is_some_and(|&front| front + width <= index)
        {
            candidates.pop_front();
        }
        while candidates
            .back()
            .is_some_and(|&back| values[back] <= values[index])
        {
            candidates.pop_back();
        }
        candidates.push_back(index);
        if index + 1 >= width {
            result.push(values[*candidates.front().unwrap()]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::sliding_window_max;

    #[test]
    fn handles_overlapping_windows() {
        assert_eq!(sliding_window_max(&[4, 2, 12, 3, 8], 3), vec![12, 12, 12]);
    }
}
