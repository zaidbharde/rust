use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Scored<T> {
    pub item: T,
    pub score: f64,
}

/// Return at most `k` items, ordered from highest score to lowest score.
pub fn top_k<T: Clone>(items: &[Scored<T>], k: usize) -> Vec<Scored<T>> {
    let mut ranked = items.to_vec();
    ranked.sort_by(|left, right| {
        right.score.partial_cmp(&left.score).unwrap_or(Ordering::Equal)
    });
    ranked.truncate(k);
    ranked
}

fn main() {
    let scores = vec![
        Scored { item: "cache", score: 0.91 },
        Scored { item: "queue", score: 0.87 },
        Scored { item: "index", score: 0.95 },
    ];
    for entry in top_k(&scores, 2) {
        println!("{} {:.2}", entry.item, entry.score);
    }
}
