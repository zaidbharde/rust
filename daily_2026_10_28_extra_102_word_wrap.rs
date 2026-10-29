//! Wrap text by words, splitting overlong tokens at Unicode character boundaries.

/// Wraps whitespace-separated words to at most `width` Unicode scalar values per
/// line, splitting longer words as needed. A zero width returns no lines;
/// whitespace-only input also returns an empty vector.
pub fn wrap_words_hard(s: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }

    let mut lines = Vec::new();
    let mut line = String::new();
    for word in s.split_whitespace() {
        let mut chunk = String::new();
        for ch in word.chars() {
            if chunk.chars().count() == width {
                append_chunk(&mut lines, &mut line, &chunk, width);
                chunk.clear();
            }
            chunk.push(ch);
        }
        if !chunk.is_empty() {
            append_chunk(&mut lines, &mut line, &chunk, width);
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn append_chunk(lines: &mut Vec<String>, line: &mut String, chunk: &str, width: usize) {
    let separator = usize::from(!line.is_empty());
    if !line.is_empty() && line.chars().count() + separator + chunk.chars().count() > width {
        lines.push(std::mem::take(line));
    }
    if !line.is_empty() {
        line.push(' ');
    }
    line.push_str(chunk);
}

#[cfg(test)]
mod tests {
    use super::wrap_words_hard;

    #[test]
    fn wraps_words_and_splits_long_tokens() {
        assert_eq!(wrap_words_hard("one two three", 7), vec!["one two", "three"]);
        assert_eq!(wrap_words_hard("abcdefgh ij", 4), vec!["abcd", "efgh", "ij"]);
    }

    #[test]
    fn handles_width_edges_and_unicode() {
        assert!(wrap_words_hard("  \n ", 5).is_empty());
        assert!(wrap_words_hard("abc", 0).is_empty());
        assert_eq!(wrap_words_hard("🙂éx", 2), vec!["🙂é", "x"]);
        assert_eq!(wrap_words_hard("a b", 1), vec!["a", "b"]);
    }
}
