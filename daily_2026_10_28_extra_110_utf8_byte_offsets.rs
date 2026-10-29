//! Map Unicode scalar-value indices to valid UTF-8 byte offsets.

/// Returns the byte offset at which the character at `char_index` begins.
///
/// The index equal to `text.chars().count()` maps to `text.len()`, making the
/// end boundary usable for slicing. An index beyond that boundary returns `None`.
pub fn char_index_to_byte_offset(text: &str, char_index: usize) -> Option<usize> {
    if char_index == 0 {
        return Some(0);
    }
    text.char_indices().nth(char_index).map(|(offset, _)| offset)
        .or_else(|| (char_index == text.chars().count()).then_some(text.len()))
}

/// Returns byte offsets for each character boundary, including the final end boundary.
pub fn char_byte_offsets(text: &str) -> Vec<usize> {
    text.char_indices().map(|(offset, _)| offset)
        .chain(std::iter::once(text.len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{char_byte_offsets, char_index_to_byte_offset};

    #[test]
    fn maps_ascii_and_multibyte_characters() {
        let text = "aé🦀z";
        assert_eq!(char_byte_offsets(text), vec![0, 1, 3, 7, 8]);
        assert_eq!(char_index_to_byte_offset(text, 2), Some(3));
        assert_eq!(char_index_to_byte_offset(text, 4), Some(8));
        assert_eq!(char_index_to_byte_offset(text, 5), None);
    }

    #[test]
    fn handles_empty_strings_and_zero_width_characters() {
        assert_eq!(char_byte_offsets(""), vec![0]);
        assert_eq!(char_index_to_byte_offset("", 0), Some(0));
        assert_eq!(char_index_to_byte_offset("", 1), None);
        assert_eq!(char_byte_offsets("e\u{301}"), vec![0, 1, 3]);
    }
}
