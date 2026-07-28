//! Functions — parameters, return values, closures, and higher-order functions.

// ── Regular functions ─────────────────────────────────────────────────
fn add(a: i32, b: i32) -> i32 { a + b }

fn power(base: f64, exp: u32) -> f64 {
    (0..exp).fold(1.0, |acc, _| acc * base)
}

fn min_max(numbers: &[i32]) -> Option<(i32, i32)> {
    if numbers.is_empty() { return None; }
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    Some((min, max))
}

// ── Multiple return values via tuple ──────────────────────────────────
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// ── Recursive function ────────────────────────────────────────────────
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _     => n * factorial(n - 1),
    }
}

// ── Higher-order function ─────────────────────────────────────────────
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    println!("=== Functions ===");

    println!("add(3, 4)       = {}", add(3, 4));
    println!("power(2, 10)    = {}", power(2.0, 10));

    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    if let Some((min, max)) = min_max(&nums) {
        println!("min/max {:?} = {}/{}", nums, min, max);
    }

    // ── Result handling ───────────────────────────────────────────────
    println!("\n--- Division ---");
    match divide(10.0, 3.0) {
        Ok(v)  => println!("10 / 3  = {:.4}", v),
        Err(e) => println!("Error: {}", e),
    }
    match divide(10.0, 0.0) {
        Ok(v)  => println!("10 / 0  = {}", v),
        Err(e) => println!("Error  : {}", e),
    }

    // ── Factorial ─────────────────────────────────────────────────────
    println!("\n--- Factorials ---");
    for n in 0..=10 {
        println!("{}! = {}", n, factorial(n));
    }

    // ── Closures ──────────────────────────────────────────────────────
    println!("\n--- Closures ---");
    let square   = |x: i32| x * x;
    let add_ten  = |x: i32| x + 10;
    let greeting = |name: &str| format!("Hello, {}!", name);

    println!("square(5)         = {}", square(5));
    println!("apply_twice(+10)  = {}", apply_twice(add_ten, 5));
    println!("{}", greeting("Rustacean"));

    // ── Iterator combinators ──────────────────────────────────────────
    println!("\n--- Iterators ---");
    let data: Vec<i32> = (1..=10).collect();
    let result: Vec<i32> = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("Even squares: {:?}", result);

    let total: i32 = (1..=100).sum();
    println!("Sum 1..=100 : {}", total);
}
