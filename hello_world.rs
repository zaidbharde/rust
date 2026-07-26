//! Hello World — variables, constants, shadowing, and basic types.

fn main() {
    // ── Constants ─────────────────────────────────────────────────────
    const MAX_SCORE: u32 = 100;
    const PI: f64 = 3.14159265358979;

    println!("=== Hello, Rust! ===");
    println!("MAX_SCORE : {}", MAX_SCORE);
    println!("PI        : {:.4}", PI);

    // ── Basic types ───────────────────────────────────────────────────
    let integer:  i32   = -42;
    let unsigned: u64   = 1_000_000;   // underscores for readability
    let float:    f64   = 3.14;
    let boolean:  bool  = true;
    let character: char = '🦀';
    let text:     &str  = "Rust is awesome";

    println!("\n--- Basic Types ---");
    println!("i32   : {}", integer);
    println!("u64   : {}", unsigned);
    println!("f64   : {}", float);
    println!("bool  : {}", boolean);
    println!("char  : {}", character);
    println!("&str  : {}", text);

    // ── Shadowing ─────────────────────────────────────────────────────
    let x = 5;
    let x = x * 2;       // shadow with new value
    let x = x + 3;       // shadow again
    println!("\nShadowed x = {}", x);  // 13

    // ── Tuple and array ───────────────────────────────────────────────
    let tuple: (i32, f64, &str) = (42, 3.14, "hello");
    let array: [i32; 5]         = [1, 2, 3, 4, 5];

    println!("\nTuple : ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    println!("Array : {:?}", array);
    println!("Sum   : {}", array.iter().sum::<i32>());
}
