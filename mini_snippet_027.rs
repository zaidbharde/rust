fn main() {
    let total: i32 = (1..=3).map(|n| n * n).sum();
    println!("{total}");
}
