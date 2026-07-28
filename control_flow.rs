//! Control Flow — if, loop, while, for, match, and ranges.

fn classify_number(n: i32) -> &'static str {
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    }
}

fn day_name(day: u8) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day",
    }
}

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _      => n.to_string(),
    }
}

fn main() {
    println!("=== Control Flow ===");

    // ── if expressions ────────────────────────────────────────────────
    for n in [-5, 0, 7] {
        println!("{} is {}", n, classify_number(n));
    }

    // ── match ─────────────────────────────────────────────────────────
    println!("\n--- Days ---");
    for d in 1..=7 {
        println!("Day {} = {}", d, day_name(d));
    }

    // ── loop with break value ─────────────────────────────────────────
    println!("\n--- Loop with value ---");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 { break counter * 2; }
    };
    println!("Loop result: {}", result);

    // ── while ─────────────────────────────────────────────────────────
    println!("\n--- While ---");
    let mut n = 1;
    while n <= 16 {
        print!("{} ", n);
        n *= 2;
    }
    println!();

    // ── for with range ────────────────────────────────────────────────
    println!("\n--- FizzBuzz 1-20 ---");
    for i in 1..=20 {
        print!("{} ", fizzbuzz(i));
    }
    println!();

    // ── Pattern matching with guards ──────────────────────────────────
    println!("\n--- Match guards ---");
    let numbers = [1, 13, 25, 42, 99, 100];
    for &num in &numbers {
        let label = match num {
            n if n < 0          => "negative",
            0                   => "zero",
            n if n % 2 == 0     => "positive even",
            n if n > 50         => "large odd",
            _                   => "small odd",
        };
        println!("  {} → {}", num, label);
    }
}
