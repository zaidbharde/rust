fn main() {
    let total: i32 = (1..=6).map(|n| n * n).sum();
    println!("{total}");
}
