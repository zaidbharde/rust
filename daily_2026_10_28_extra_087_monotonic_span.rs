//! Compute stock-style spans using the previous strictly greater value.
/// For each item, returns how many positions back the nearest strictly greater
/// item is, or the full prefix length when no such item exists. Equal values
/// are included in the span. Runs in O(n) time and O(n) extra space.
pub fn monotonic_spans<T: Ord>(values: &[T]) -> Vec<usize> {
    let mut spans = Vec::with_capacity(values.len());
    let mut stack: Vec<usize> = Vec::new();

    for (index, value) in values.iter().enumerate() {
        while stack.last().is_some_and(|&previous| values[previous] <= *value) {
            stack.pop();
        }
        spans.push(stack.last().map_or(index + 1, |&previous| index - previous));
        stack.push(index);
    }
    spans
}

#[cfg(test)]
mod tests {
    use super::monotonic_spans;

    #[test]
    fn computes_previous_greater_spans() {
        assert_eq!(monotonic_spans(&[100, 80, 60, 70, 60, 75, 85]), [1, 1, 1, 2, 1, 4, 6]);
        assert_eq!(monotonic_spans(&[5, 5, 5]), [1, 2, 3]);
        assert_eq!(monotonic_spans(&[1, 2, 3]), [1, 2, 3]);
        assert_eq!(monotonic_spans(&[3, 2, 1]), [1, 1, 1]);
    }

    #[test]
    fn handles_empty_and_single_item_inputs() {
        assert!(monotonic_spans::<i32>(&[]).is_empty());
        assert_eq!(monotonic_spans(&[42]), [1]);
    }
}
