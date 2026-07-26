//! Testing — unit tests, integration-style tests, benchmarks, and test helpers.

// ── Functions under test ──────────────────────────────────────────────
pub fn add(a: i32, b: i32)      -> i32  { a + b }
pub fn subtract(a: i32, b: i32) -> i32  { a - b }
pub fn multiply(a: i32, b: i32) -> i32  { a * b }

pub fn is_prime(n: u64) -> bool {
    if n < 2  { return false; }
    if n == 2 { return true;  }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 }

pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n { let c = a + b; a = b; b = c; }
            b
        }
    }
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err(String::from("Division by zero")) }
    else        { Ok(a / b) }
}

// ── Tests ─────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── Arithmetic ────────────────────────────────────────────────────
    #[test] fn test_add()      { assert_eq!(add(2, 3),      5);  }
    #[test] fn test_subtract() { assert_eq!(subtract(10, 3), 7); }
    #[test] fn test_multiply() { assert_eq!(multiply(4, 5), 20); }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -1), -2);
        assert_eq!(add(-5,  5),  0);
    }

    // ── Prime ─────────────────────────────────────────────────────────
    #[test]
    fn test_is_prime() {
        let primes     = [2, 3, 5, 7, 11, 13, 97, 101];
        let composites = [0, 1, 4, 6, 8,  9,  100];

        for &p in &primes     { assert!(is_prime(p),  "{} should be prime", p); }
        for &c in &composites { assert!(!is_prime(c), "{} should not be prime", c); }
    }

    // ── String ────────────────────────────────────────────────────────
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"),  "tsur");
        assert_eq!(reverse_string(""),      "");
        assert_eq!(reverse_string("a"),     "a");
    }

    // ── Temperature ───────────────────────────────────────────────────
    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!((celsius_to_fahrenheit(0.0)   - 32.0).abs() < 1e-9);
        assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-9);
        assert!((celsius_to_fahrenheit(-40.0) - -40.0).abs() < 1e-9);
    }

    // ── Fibonacci ─────────────────────────────────────────────────────
    #[test]
    fn test_fibonacci() {
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (i, &val) in expected.iter().enumerate() {
            assert_eq!(fibonacci(i as u32), val, "fib({}) failed", i);
        }
    }

    // ── Result ────────────────────────────────────────────────────────
    #[test]
    fn test_divide_ok() {
        let result = divide(10.0, 2.0);
        assert!(result.is_ok());
        assert!((result.unwrap() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    // ── should_panic ──────────────────────────────────────────────────
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[99];
    }
}

fn main() {
    println!("Run tests with: cargo test");
    println!("\nQuick demo:");
    println!("  fib(10) = {}", fibonacci(10));
    println!("  97 prime? {}", is_prime(97));
    println!("  reverse('Rust') = {}", reverse_string("Rust"));
    println!("  0°C = {}°F", celsius_to_fahrenheit(0.0));
}
