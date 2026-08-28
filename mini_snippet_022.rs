fn main() {
    let total: i32 = (1..=7).map(|n| n * n).sum();
    println!("{total}");
}
