//! Structs, Enums, impl blocks, and associated methods.

use std::fmt;

// ── Struct ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
struct Rectangle {
    width:  f64,
    height: f64,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: f64, height: f64) -> Self {
        if width <= 0.0 || height <= 0.0 {
            panic!("Dimensions must be positive.");
        }
        Self { width, height }
    }

    fn square(size: f64) -> Self {
        Self::new(size, size)
    }

    fn area(&self)      -> f64  { self.width * self.height }
    fn perimeter(&self) -> f64  { 2.0 * (self.width + self.height) }
    fn diagonal(&self)  -> f64  { (self.width.powi(2) + self.height.powi(2)).sqrt() }
    fn is_square(&self) -> bool { (self.width - self.height).abs() < f64::EPSILON }

    fn scale(&mut self, factor: f64) {
        self.width  *= factor;
        self.height *= factor;
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle({}×{})", self.width, self.height)
    }
}

// ── Enum with data ────────────────────────────────────────────────────
#[derive(Debug)]
enum Shape {
    Circle    { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle  { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle    { radius }          => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height }   => width * height,
            Shape::Triangle  { base, height }    => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle    { .. } => "Circle",
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Triangle  { .. } => "Triangle",
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (area: {:.2})", self.name(), self.area())
    }
}

// ── Option-like enum ──────────────────────────────────────────────────
#[derive(Debug)]
enum Direction {
    North, South, East, West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
        }
    }
}

fn main() {
    println!("=== Structs & Enums ===");

    // ── Rectangle ─────────────────────────────────────────────────────
    println!("\n--- Rectangle ---");
    let mut rect = Rectangle::new(10.0, 5.0);
    println!("  {}", rect);
    println!("  Area      : {}", rect.area());
    println!("  Perimeter : {}", rect.perimeter());
    println!("  Diagonal  : {:.4}", rect.diagonal());
    println!("  Is square : {}", rect.is_square());

    rect.scale(2.0);
    println!("  After scale(2): {}", rect);

    let sq = Rectangle::square(7.0);
    println!("  Square: {}, is_square={}", sq, sq.is_square());

    // ── Shapes ────────────────────────────────────────────────────────
    println!("\n--- Shapes ---");
    let shapes = vec![
        Shape::Circle    { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle  { base: 3.0, height: 8.0 },
    ];

    for shape in &shapes {
        println!("  {}", shape);
    }

    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("  Total area: {:.2}", total);

    // ── Direction ─────────────────────────────────────────────────────
    println!("\n--- Directions ---");
    for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
        println!("  {:?} → {:?}", dir, dir.opposite());
    }
}
