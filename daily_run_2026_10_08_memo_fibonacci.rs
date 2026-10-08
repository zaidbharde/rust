use std::collections::HashMap;

/// Computes Fibonacci values with an explicit cache and checked arithmetic.
pub fn fibonacci(n: u32, cache: &mut HashMap<u32, u128>) -> Option<u128> {
    if let Some(&value) = cache.get(&n) { return Some(value); }
    let value = match n {
        0 => 0,
        1 => 1,
        _ => {
            let left = fibonacci(n - 1, cache)?;
            let right = fibonacci(n - 2, cache)?;
            left.checked_add(right)?
        }
    };
    cache.insert(n, value);
    Some(value)
}

pub fn sequence(count: usize) -> Option<Vec<u128>> {
    let mut cache = HashMap::new();
    (0..count as u32).map(|n| fibonacci(n, &mut cache)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builds_prefix() {
        assert_eq!(sequence(7).unwrap(), vec![0, 1, 1, 2, 3, 5, 8]);
    }

    #[test]
    fn reports_overflow() {
        let mut cache = HashMap::new();
        assert!(fibonacci(2000, &mut cache).is_none());
    }
}
