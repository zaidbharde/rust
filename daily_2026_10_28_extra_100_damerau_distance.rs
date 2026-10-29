//! Full Damerau–Levenshtein distance, including adjacent transpositions.

use std::collections::HashMap;

/// Returns the minimum number of insertions, deletions, substitutions, and
/// adjacent transpositions needed to turn `left` into `right`.
///
/// Input is compared as Unicode scalar values (`char`), not bytes. This full
/// variant also accounts for transpositions involving intervening edits.
pub fn damerau_distance(left: &str, right: &str) -> usize {
    let a: Vec<char> = left.chars().collect();
    let b: Vec<char> = right.chars().collect();
    let (n, m) = (a.len(), b.len());
    let max_distance = n + m;
    let mut table = vec![vec![0usize; m + 2]; n + 2];
    table[0][0] = max_distance;
    for i in 0..=n {
        table[i + 1][0] = max_distance;
        table[i + 1][1] = i;
    }
    for j in 0..=m {
        table[0][j + 1] = max_distance;
        table[1][j + 1] = j;
    }

    let mut last_row = HashMap::new();
    for i in 1..=n {
        let mut last_match_column = 0;
        for j in 1..=m {
            let matching_row = *last_row.get(&b[j - 1]).unwrap_or(&0);
            let matching_column = last_match_column;
            let cost = usize::from(a[i - 1] != b[j - 1]);
            if cost == 0 { last_match_column = j; }
            table[i + 1][j + 1] = (table[i][j] + cost)
                .min(table[i + 1][j] + 1)
                .min(table[i][j + 1] + 1)
                .min(table[matching_row][matching_column]
                    + (i - matching_row - 1) + 1 + (j - matching_column - 1));
        }
        last_row.insert(a[i - 1], i);
    }
    table[n + 1][m + 1]
}

#[cfg(test)]
mod tests {
    use super::damerau_distance;

    #[test]
    fn handles_basic_edits_and_adjacent_transpositions() {
        assert_eq!(damerau_distance("", "abc"), 3);
        assert_eq!(damerau_distance("same", "same"), 0);
        assert_eq!(damerau_distance("ca", "ac"), 1);
        assert_eq!(damerau_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn supports_full_damerau_cases_and_unicode() {
        assert_eq!(damerau_distance("CA", "ABC"), 2);
        assert_eq!(damerau_distance("a🙂", "🙂a"), 1);
        assert_eq!(damerau_distance("🙂", ""), 1);
    }
}
