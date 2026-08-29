fn main() {
    let word = "level";
    let reversed: String = word.chars().rev().collect();
    println!("{}", word == reversed);
}
