//! Error Handling — Result, Option, custom errors, and the ? operator.

use std::fmt;
use std::num::ParseIntError;

// ── Custom error type ─────────────────────────────────────────────────
#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    DivisionByZero,
    NegativeNumber(i64),
    OutOfRange { value: i64, min: i64, max: i64 },
    EmptyInput,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ParseError(e) =>
                write!(f, "Parse error: {}", e),
            AppError::DivisionByZero =>
                write!(f, "Cannot divide by zero"),
            AppError::NegativeNumber(n) =>
                write!(f, "Expected positive number, got {}", n),
            AppError::OutOfRange { value, min, max } =>
                write!(f, "Value {} is out of range [{}, {}]", value, min, max),
            AppError::EmptyInput =>
                write!(f, "Input cannot be empty"),
        }
    }
}

// Automatic conversion from ParseIntError
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

// ── Functions using Result ────────────────────────────────────────────
fn parse_positive(s: &str) -> Result<u64, AppError> {
    if s.trim().is_empty() { return Err(AppError::EmptyInput); }

    let n: i64 = s.trim().parse().map_err(AppError::ParseError)?;
    if n < 0 { return Err(AppError::NegativeNumber(n)); }

    Ok(n as u64)
}

fn safe_divide(a: f64, b: f64) -> Result<f64, AppError> {
    if b == 0.0 { Err(AppError::DivisionByZero) } else { Ok(a / b) }
}

fn clamp_range(value: i64, min: i64, max: i64) -> Result<i64, AppError> {
    if value < min || value > max {
        Err(AppError::OutOfRange { value, min, max })
    } else {
        Ok(value)
    }
}

// ── Chaining with ? operator ──────────────────────────────────────────
fn process(input: &str) -> Result<String, AppError> {
    let n      = parse_positive(input)?;
    let clamped = clamp_range(n as i64, 1, 1000)?;
    let divided = safe_divide(clamped as f64, 3.0)?;
    Ok(format!("{} / 3 = {:.4}", clamped, divided))
}

// ── Option handling ───────────────────────────────────────────────────
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

fn safe_head(v: &[i32]) -> Option<&i32> {
    v.first()
}

fn main() {
    println!("=== Error Handling ===");

    // ── Result matching ───────────────────────────────────────────────
    println!("\n--- Result ---");
    let inputs = ["42", "-5", "abc", "", "1500", "100"];

    for input in &inputs {
        match process(input) {
            Ok(msg)  => println!("  ✅ {:>6} → {}", input, msg),
            Err(e)   => println!("  ❌ {:>6} → {}", input, e),
        }
    }

    // ── unwrap_or / unwrap_or_else ────────────────────────────────────
    println!("\n--- unwrap_or ---");
    let result = safe_divide(10.0, 0.0).unwrap_or(f64::INFINITY);
    println!("  10 / 0 = {}", result);

    let parsed = "not_a_number"
        .parse::<i32>()
        .unwrap_or_else(|_| -1);
    println!("  Parsed fallback: {}", parsed);

    // ── Option ────────────────────────────────────────────────────────
    println!("\n--- Option ---");
    let odds   = vec![1, 3, 5, 7, 9];
    let mixed  = vec![1, 3, 4, 7, 8];

    println!("  First even in {:?}: {:?}", odds,  find_first_even(&odds));
    println!("  First even in {:?}: {:?}", mixed, find_first_even(&mixed));

    let empty: Vec<i32> = vec![];
    println!("  Head of [1,2,3] : {:?}", safe_head(&[1, 2, 3]));
    println!("  Head of []      : {:?}", safe_head(&empty));

    // ── Option chaining ───────────────────────────────────────────────
    println!("\n--- Option chaining ---");
    let names = vec!["Alice", "Bob", "Charlie"];
    let result = names.get(1)
        .map(|n| n.to_uppercase())
        .filter(|n| n.len() > 2)
        .unwrap_or_else(|| String::from("unknown"));
    println!("  names[1] upper: {}", result);
}
