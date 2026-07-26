//! Traits and Generics — polymorphism the Rust way.

use std::fmt;

// ── Trait definition ──────────────────────────────────────────────────
trait Animal {
    fn name(&self)  -> &str;
    fn sound(&self) -> &str;
    fn legs(&self)  -> u8;

    // Default method
    fn describe(&self) -> String {
        format!(
            "{} says '{}' and has {} leg(s).",
            self.name(), self.sound(), self.legs()
        )
    }
}

// ── Structs implementing the trait ────────────────────────────────────
struct Dog { name: String }
struct Cat { name: String }
struct Bird { name: String, can_fly: bool }

impl Animal for Dog {
    fn name(&self)  -> &str { &self.name }
    fn sound(&self) -> &str { "Woof" }
    fn legs(&self)  -> u8   { 4 }
}

impl Animal for Cat {
    fn name(&self)  -> &str { &self.name }
    fn sound(&self) -> &str { "Meow" }
    fn legs(&self)  -> u8   { 4 }
}

impl Animal for Bird {
    fn name(&self)  -> &str { &self.name }
    fn sound(&self) -> &str { "Tweet" }
    fn legs(&self)  -> u8   { 2 }

    fn describe(&self) -> String {
        let fly = if self.can_fly { "can fly" } else { "cannot fly" };
        format!("{} says '{}', has 2 legs, and {}.", self.name(), self.sound(), fly)
    }
}

// ── Generic function with trait bound ─────────────────────────────────
fn print_animal<T: Animal>(animal: &T) {
    println!("  {}", animal.describe());
}

// ── Generic struct ────────────────────────────────────────────────────
#[derive(Debug)]
struct Pair<T> {
    first:  T,
    second: T,
}

impl<T: PartialOrd + fmt::Display> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    fn larger(&self) -> &T {
        if self.first > self.second { &self.first } else { &self.second }
    }
}

impl<T: fmt::Display> fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

// ── Generic function ──────────────────────────────────────────────────
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    max
}

// ── Trait objects (dynamic dispatch) ─────────────────────────────────
fn animal_sounds(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        println!("  {}", animal.describe());
    }
}

fn main() {
    println!("=== Traits & Generics ===");

    // ── Static dispatch ───────────────────────────────────────────────
    println!("\n--- Static Dispatch ---");
    let dog  = Dog  { name: String::from("Rex") };
    let cat  = Cat  { name: String::from("Whiskers") };
    let bird = Bird { name: String::from("Tweety"), can_fly: true };

    print_animal(&dog);
    print_animal(&cat);
    print_animal(&bird);

    // ── Dynamic dispatch ──────────────────────────────────────────────
    println!("\n--- Dynamic Dispatch (trait objects) ---");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog  { name: String::from("Buddy") }),
        Box::new(Cat  { name: String::from("Luna") }),
        Box::new(Bird { name: String::from("Polly"), can_fly: false }),
    ];
    animal_sounds(&animals);

    // ── Generic Pair ──────────────────────────────────────────────────
    println!("\n--- Generic Pair ---");
    let int_pair = Pair::new(5, 10);
    let str_pair = Pair::new("apple", "zebra");

    println!("  Int pair {} → larger: {}", int_pair, int_pair.larger());
    println!("  Str pair {} → larger: {}", str_pair, str_pair.larger());

    // ── Generic largest ───────────────────────────────────────────────
    println!("\n--- Generic Largest ---");
    let ints   = vec![34, 50, 25, 100, 65];
    let floats = vec![2.1, 8.8, 3.3, 1.0];
    let chars  = vec!['y', 'm', 'a', 'q'];

    println!("  Largest int   : {}", largest(&ints));
    println!("  Largest float : {}", largest(&floats));
    println!("  Largest char  : {}", largest(&chars));
}
