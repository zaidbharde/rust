use std::collections::HashMap;

/// Count alphabetic words case-insensitively and return them by descending frequency.
pub fn ranked_words(text: &str) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut word = String::new();
    let flush = |word: &mut String, counts: &mut HashMap<String, usize>| {
        if !word.is_empty() {
            *counts.entry(word.to_lowercase()).or_default() += 1;
            word.clear();
        }
    };
    for character in text.chars() {
        if character.is_alphabetic() || character == '\'' {
            word.push(character);
        } else {
            flush(&mut word, &mut counts);
        }
    }
    flush(&mut word, &mut counts);
    let mut ranked: Vec<_> = counts.into_iter().collect();
    ranked.sort_by(|left, right| {
        right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0))
    });
    ranked
}

fn main() {
    let report = "Rust makes systems programming practical; Rust makes speed practical.";
    for (word, count) in ranked_words(report) {
        println!("{word}: {count}");
    }
}

#[cfg(test)]
mod tests {
    use super::ranked_words;

    #[test]
    fn ranking_breaks_ties_alphabetically() {
        assert_eq!(ranked_words("B a b"), vec![("b".into(), 2), ("a".into(), 1)]);
    }
}
