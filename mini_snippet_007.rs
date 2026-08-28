fn main() {
    let total: i32 = (1..=10).map(|n| n * n).sum();
    println!("{total}");
}
