/// Returns the prime factors of `n` in nondecreasing order, retaining multiplicity.
///
/// Values below 2 (zero and one) have no prime factors, so they produce an empty vector.
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    while n % 2 == 0 && n > 1 {
        factors.push(2);
        n /= 2;
    }

    let mut divisor = 3u64;
    while divisor <= n / divisor {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 2;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::prime_factors;

    #[test]
    fn handles_values_without_prime_factors() {
        assert_eq!(prime_factors(0), Vec::<u64>::new());
        assert_eq!(prime_factors(1), Vec::<u64>::new());
    }

    #[test]
    fn retains_multiplicity_and_order() {
        assert_eq!(prime_factors(360), vec![2, 2, 2, 3, 3, 5]);
        assert_eq!(prime_factors(13), vec![13]);
    }

    #[test]
    fn handles_large_prime_and_composite_values() {
        assert_eq!(prime_factors(u64::MAX), vec![3, 5, 17, 257, 641, 65537, 6700417]);
        assert_eq!(prime_factors(1u64 << 63), vec![2; 63]);
    }
}
