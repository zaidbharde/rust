//! Exact floor square root for `u64`, computed without floating-point arithmetic.

/// Returns the greatest `r` such that `r * r <= n`.
///
/// Division-based comparisons avoid overflow even for `u64::MAX`.
pub fn integer_square_root(n: u64) -> u64 {
    let (mut low, mut high) = (0u64, n.min(u32::MAX as u64) + 1);
    while low + 1 < high {
        let mid = low + (high - low) / 2;
        if mid <= n / mid {
            low = mid;
        } else {
            high = mid;
        }
    }
    low
}

#[cfg(test)]
mod tests {
    use super::integer_square_root;

    #[test]
    fn demo_checks_exact_roots_and_boundaries() {
        assert_eq!(integer_square_root(0), 0);
        assert_eq!(integer_square_root(1), 1);
        assert_eq!(integer_square_root(2), 1);
        assert_eq!(integer_square_root(15), 3);
        assert_eq!(integer_square_root(16), 4);
        assert_eq!(integer_square_root(17), 4);
        assert_eq!(integer_square_root(4_294_967_295), 65_535);
        assert_eq!(integer_square_root(4_294_967_296), 65_536);
        assert_eq!(integer_square_root(u64::MAX), u32::MAX as u64);
    }

    #[test]
    fn result_brackets_input_without_overflow() {
        for n in [0, 3, 99, 1_000_000, u32::MAX as u64, u64::MAX] {
            let root = integer_square_root(n);
            assert!(root <= n / root.max(1) || root == 0);
            assert!(root == u32::MAX as u64 || (root + 1) > n / (root + 1));
        }
    }
}
