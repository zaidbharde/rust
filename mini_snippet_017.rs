fn main() {
    let total: i32 = (1..=11).map(|n| n * n).sum();
    println!("{total}");
}
