//! Find the longest contiguous substring shared by two strings.

/// Returns the longest common substring, comparing Unicode scalar values.
///
/// If several matches have the same maximum length, the earliest match in
/// `left` is returned. The result is an owned string and is empty when either
/// input is empty or no characters are shared.
pub fn longest_common_substring(left: &str, right: &str) -> String {
    let a: Vec<char> = left.chars().collect();
    let b: Vec<char> = right.chars().collect();
    let mut previous = vec![0usize; b.len() + 1];
    let mut best_len = 0;
    let mut best_start = 0;

    for (i, &ch) in a.iter().enumerate() {
        let mut current = vec![0usize; b.len() + 1];
        for (j, &other) in b.iter().enumerate() {
            if ch == other {
                current[j + 1] = previous[j] + 1;
                let start = i + 1 - current[j + 1];
                if current[j + 1] > best_len
                    || (current[j + 1] == best_len && start < best_start)
                {
                    best_len = current[j + 1];
                    best_start = start;
                }
            }
        }
        previous = current;
    }

    a[best_start..best_start + best_len].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::longest_common_substring;

    #[test]
    fn finds_contiguous_match_and_handles_no_match() {
        assert_eq!(longest_common_substring("xabcdz", "yyabcq"), "abc");
        assert_eq!(longest_common_substring("abc", "xyz"), "");
    }

    #[test]
    fn handles_empty_inputs_and_ties() {
        assert_eq!(longest_common_substring("", "abc"), "");
        assert_eq!(longest_common_substring("abc", ""), "");
        assert_eq!(longest_common_substring("abXYcd", "cdXYab"), "ab");
    }

    #[test]
    fn compares_unicode_characters() {
        assert_eq!(longest_common_substring("a🙂bc", "x🙂by"), "🙂b");
    }
}
