fn main() {
    let total: i32 = (1..=8).map(|n| n * n).sum();
    println!("{total}");
}
