//! Levenshtein distance with a memory-bounded dynamic program.

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

pub fn within_distance(left: &str, right: &str, maximum: usize) -> bool {
    distance(left, right) <= maximum
}

fn main() {
    assert_eq!(distance("kitten", "sitting"), 3);
    assert_eq!(distance("same", "same"), 0);
    assert!(within_distance("catalog", "cat", 4));
    assert!(!within_distance("rust", "python", 2));
    println!("edit distance checks passed");
}
