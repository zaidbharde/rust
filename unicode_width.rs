//! Estimate terminal display width for common Unicode character classes.

pub fn char_width(ch: char) -> usize {
    let code = ch as u32;
    if ch == '\u{0000}' || (code < 32) || (0x7f..=0x9f).contains(&code) { return 0; }
    if is_combining(code) { return 0; }
    if is_wide(code) { return 2; }
    1
}

pub fn display_width(text: &str) -> usize { text.chars().map(char_width).sum() }

fn is_combining(code: u32) -> bool {
    (0x0300..=0x036f).contains(&code)
        || (0x1ab0..=0x1aff).contains(&code)
        || (0x1dc0..=0x1dff).contains(&code)
        || (0x20d0..=0x20ff).contains(&code)
        || (0xfe20..=0xfe2f).contains(&code)
}

fn is_wide(code: u32) -> bool {
    (0x1100..=0x115f).contains(&code)
        || (0x2329..=0x232a).contains(&code)
        || (0x2e80..=0xa4cf).contains(&code)
        || (0xac00..=0xd7a3).contains(&code)
        || (0xf900..=0xfaff).contains(&code)
        || (0xfe10..=0xfe19).contains(&code)
        || (0xff01..=0xff60).contains(&code)
        || (0x1f300..=0x1faff).contains(&code)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combines_marks_without_extra_width() {
        assert_eq!(display_width("e\u{301}"), 1);
        assert_eq!(display_width("A界"), 3);
    }
    #[test]
    fn controls_have_no_width() { assert_eq!(display_width("a\n\tb"), 2); }
}
