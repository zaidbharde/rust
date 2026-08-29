fn main() {
    let values = [3, 1, 4, 1, 5];
    let best = values.windows(2).map(|w| w[0] + w[1]).max().unwrap();
    println!("best_pair={best}");
}
