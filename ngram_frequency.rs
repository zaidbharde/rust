use std::collections::HashMap;

fn char_ngrams(text: &str, n: usize) -> HashMap<String, u32> {
    let chars: Vec<char> = text.chars().collect();
    let mut freq = HashMap::new();

    if chars.len() < n { return freq; }

    for i in 0..=chars.len() - n {
        let ngram: String = chars[i..i + n].iter().collect();
        *freq.entry(ngram).or_insert(0) += 1;
    }
    freq
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox runs";
    let bigrams = char_ngrams(text, 3);

    let mut sorted: Vec<(&String, &u32)> = bigrams.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    println!("Top 5 most frequent 3-grams:");
    for (ngram, count) in sorted.iter().take(5) {
        println!("{:?} -> {}", ngram, count);
    }
}
