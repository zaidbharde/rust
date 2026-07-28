//! Ownership — Rust's core memory safety model.
//! Move semantics, cloning, borrowing, and slices.

fn takes_ownership(s: String) -> usize {
    println!("  Got: {}", s);
    s.len()   // s is dropped here
}

fn borrows(s: &String) -> usize {
    println!("  Borrowed: {}", s);
    s.len()   // s is NOT dropped — we only borrowed it
}

fn borrows_slice(s: &str) -> &str {
    // Works with both String and &str — prefer &str in function signatures
    let first_word_end = s.find(' ').unwrap_or(s.len());
    &s[..first_word_end]
}

fn mutable_borrow(v: &mut Vec<i32>) {
    v.push(100);
    v.sort();
}

fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list {
        if item > max { max = item; }
    }
    max
}

fn main() {
    println!("=== Ownership ===");

    // ── Move semantics ────────────────────────────────────────────────
    println!("\n--- Move ---");
    let s1 = String::from("hello");
    let len = takes_ownership(s1);
    // println!("{}", s1);   // ERROR: s1 was moved
    println!("  Length was: {}", len);

    // ── Clone ─────────────────────────────────────────────────────────
    println!("\n--- Clone ---");
    let s2 = String::from("world");
    let s3 = s2.clone();
    println!("  s2 = {}, s3 = {}", s2, s3);   // both valid

    // ── Borrow ────────────────────────────────────────────────────────
    println!("\n--- Borrow ---");
    let s4 = String::from("Rust");
    let len = borrows(&s4);
    println!("  s4 still valid: {} (len={})", s4, len);

    // ── Slices ────────────────────────────────────────────────────────
    println!("\n--- Slices ---");
    let sentence = String::from("hello world");
    let word = borrows_slice(&sentence);
    println!("  First word: {}", word);

    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4];
    println!("  Array slice [1..4]: {:?}", slice);

    // ── Mutable references ────────────────────────────────────────────
    println!("\n--- Mutable Borrow ---");
    let mut numbers = vec![5, 3, 1, 4, 2];
    println!("  Before: {:?}", numbers);
    mutable_borrow(&mut numbers);
    println!("  After : {:?}", numbers);

    // ── Largest in slice ──────────────────────────────────────────────
    println!("\n--- Largest ---");
    let data = vec![34, 50, 25, 100, 65];
    println!("  Largest in {:?} = {}", data, largest(&data));
}
