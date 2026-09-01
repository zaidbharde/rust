/// Compute the minimum single-character edits needed to transform two strings.
pub fn distance(left: &str, right: &str) -> usize {
    let right_chars: Vec<char> = right.chars().collect();
    let mut previous: Vec<usize> = (0..=right_chars.len()).collect();
    for (row, left_char) in left.chars().enumerate() {
        let mut current = vec![row + 1; right_chars.len() + 1];
        for (column, right_char) in right_chars.iter().enumerate() {
            let substitution = previous[column] + usize::from(left_char != *right_char);
            let insertion = current[column] + 1;
            let deletion = previous[column + 1] + 1;
            current[column + 1] = substitution.min(insertion).min(deletion);
        }
        previous = current;
    }
    previous[right_chars.len()]
}

pub fn similarity(left: &str, right: &str) -> f64 {
    let longest = left.chars().count().max(right.chars().count());
    if longest == 0 { return 1.0; }
    1.0 - distance(left, right) as f64 / longest as f64
}

fn main() {
    println!("distance={}, similarity={:.2}", distance("kitten", "sitting"), similarity("rust", "trust"));
}
