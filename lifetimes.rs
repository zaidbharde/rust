//! Lifetimes — explicit annotations, structs with references, elision rules.

// ── Basic lifetime annotation ─────────────────────────────────────────
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[..i]; }
    }
    s
}

// ── Struct holding a reference ────────────────────────────────────────
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(text: &'a str) -> Self {
        let first_sentence = text.split('.').next().unwrap_or(text);
        Self { part: first_sentence.trim() }
    }

    fn announce(&self, announcement: &str) -> &str {
        println!("  Attention: {}", announcement);
        self.part
    }
}

// ── Multiple lifetimes ────────────────────────────────────────────────
fn first_longer<'a, 'b>(s1: &'a str, s2: &'b str) -> bool {
    s1.len() > s2.len()
}

// ── Static lifetime ───────────────────────────────────────────────────
fn get_greeting() -> &'static str {
    "Hello, Rustacean!"   // string literals live for the entire program
}

fn main() {
    println!("=== Lifetimes ===");

    // ── longest ───────────────────────────────────────────────────────
    println!("\n--- longest ---");
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("  Longest of '{}' and '{}': '{}'", s1, s2, result);
    }

    // ── first_word ────────────────────────────────────────────────────
    println!("\n--- first_word ---");
    let sentence = String::from("hello world from Rust");
    let word = first_word(&sentence);
    println!("  '{}' → first word: '{}'", sentence, word);

    // ── Excerpt struct ────────────────────────────────────────────────
    println!("\n--- Excerpt ---");
    let novel = String::from(
        "Call me Ishmael. Some years ago. Never mind how long precisely."
    );
    let excerpt = Excerpt::new(&novel);
    println!("  Excerpt : {:?}", excerpt);
    let part = excerpt.announce("New chapter beginning!");
    println!("  Part    : {}", part);

    // ── Static lifetime ───────────────────────────────────────────────
    println!("\n--- Static ---");
    let greeting: &'static str = get_greeting();
    println!("  {}", greeting);

    // ── first_longer ──────────────────────────────────────────────────
    println!("\n--- first_longer ---");
    println!("  'hello' > 'hi' : {}", first_longer("hello", "hi"));
    println!("  'hi' > 'hello' : {}", first_longer("hi", "hello"));
}
