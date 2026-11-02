/// Escape text for insertion into an HTML text node.
///
/// This is not an encoder for JavaScript, CSS, or URL contexts.
pub fn escape_html_text(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for character in input.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            other => escaped.push(other),
        }
    }
    escaped
}

fn main() {
    let user_text = "<script>alert('hello')</script>";
    println!("{}", escape_html_text(user_text));
}

#[cfg(test)]
mod tests {
    use super::escape_html_text;

    #[test]
    fn escapes_html_delimiters_and_both_quotes() {
        assert_eq!(escape_html_text("<&>\"'"), "&lt;&amp;&gt;&quot;&#39;");
    }

    #[test]
    fn escapes_ampersands_before_existing_entities() {
        assert_eq!(escape_html_text("&lt;"), "&amp;lt;");
    }

    #[test]
    fn preserves_plain_unicode_text() {
        assert_eq!(escape_html_text("café 東京"), "café 東京");
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(escape_html_text(""), "");
    }
}
