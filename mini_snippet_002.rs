fn main() {
    let total: i32 = (1..=5).map(|n| n * n).sum();
    println!("{total}");
}
