use std::cmp::Ordering;

/// A reduced rational number with a positive denominator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rational {
    pub numerator: i128,
    pub denominator: i128,
}

impl Rational {
    /// Builds a reduced value; returns `None` for a zero denominator.
    pub fn new(numerator: i64, denominator: i64) -> Option<Self> {
        if denominator == 0 {
            return None;
        }
        let (mut n, mut d) = (i128::from(numerator), i128::from(denominator));
        if d < 0 {
            n = -n;
            d = -d;
        }
        let divisor = gcd(n.unsigned_abs(), d as u128) as i128;
        Some(Self { numerator: n / divisor, denominator: d / divisor })
    }

    /// Compares values using widened products (i64 input products fit in i128).
    pub fn compare(self, other: Self) -> Ordering {
        (self.numerator * other.denominator)
            .cmp(&(other.numerator * self.denominator))
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_sign_and_reduces() {
        assert_eq!(Rational::new(6, -8), Some(Rational { numerator: -3, denominator: 4 }));
        assert_eq!(Rational::new(0, i64::MIN), Some(Rational { numerator: 0, denominator: 1 }));
    }

    #[test]
    fn handles_extreme_values_and_compares_without_overflow() {
        let a = Rational::new(i64::MAX, i64::MAX - 1).unwrap();
        let b = Rational::new(i64::MAX - 1, i64::MAX).unwrap();
        assert_eq!(a.compare(b), Ordering::Greater);
        assert_eq!(Rational::new(i64::MIN, -1).unwrap().numerator, 1i128 << 63);
    }

    #[test]
    fn rejects_zero_denominator() {
        assert_eq!(Rational::new(1, 0), None);
    }
}
