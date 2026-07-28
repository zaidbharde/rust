//! Iterators & Closures — chaining, custom iterators, lazy evaluation.

// ── Custom iterator ───────────────────────────────────────────────────
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self { Self { a: 0, b: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a)   // infinite iterator — use .take(n)
    }
}

// ── Custom range iterator ─────────────────────────────────────────────
struct Counter {
    count: u32,
    max:   u32,
}

impl Counter {
    fn new(max: u32) -> Self { Self { count: 0, max } }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    println!("=== Iterators & Closures ===");

    // ── Fibonacci ─────────────────────────────────────────────────────
    println!("\n--- Fibonacci (first 10) ---");
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("  {:?}", fibs);

    let fib_sum: u64 = Fibonacci::new().take(10).sum();
    println!("  Sum: {}", fib_sum);

    // ── Counter ───────────────────────────────────────────────────────
    println!("\n--- Counter pairs ---");
    let pairs: Vec<_> = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .collect();
    println!("  Pairs    : {:?}", pairs);

    let pair_sum: u32 = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("  Pair products divisible by 3: {}", pair_sum);

    // ── Iterator combinators ──────────────────────────────────────────
    println!("\n--- Combinators ---");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<_> = data.iter()
        .filter(|&&x| x % 2 != 0)
        .map(|&x| x * x)
        .take(3)
        .collect();
    println!("  First 3 odd squares : {:?}", result);

    let flat: Vec<_> = vec![vec![1,2], vec![3,4], vec![5,6]]
        .into_iter()
        .flatten()
        .collect();
    println!("  Flattened           : {:?}", flat);

    let words = vec!["hello world", "foo bar", "rust lang"];
    let all_words: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("  Flat_map words      : {:?}", all_words);

    // ── fold / reduce ─────────────────────────────────────────────────
    println!("\n--- fold / scan ---");
    let factorial: u64 = (1..=10).fold(1, |acc, x| acc * x);
    println!("  10! = {}", factorial);

    let running_sum: Vec<u32> = (1..=5)
        .scan(0, |state, x| { *state += x; Some(*state) })
        .collect();
    println!("  Running sum: {:?}", running_sum);

    // ── Closures capturing environment ────────────────────────────────
    println!("\n--- Closures ---");
    let threshold = 5;
    let above: Vec<_> = data.iter()
        .filter(|&&x| x > threshold)
        .collect();
    println!("  Above {} : {:?}", threshold, above);

    let mut count = 0;
    let mut increment = || { count += 1; count };
    println!("  Increment: {}", increment());
    println!("  Increment: {}", increment());
    println!("  Increment: {}", increment());

    // ── partition / unzip ─────────────────────────────────────────────
    println!("\n--- partition ---");
    let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=10).partition(|x| x % 2 == 0);
    println!("  Evens : {:?}", evens);
    println!("  Odds  : {:?}", odds);
}
