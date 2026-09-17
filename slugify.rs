/// Converts free-form text into a lowercase dash-separated slug.
pub fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;
    for character in input.chars() {
        if character.is_alphanumeric() {
            if pending_dash && !slug.is_empty() { slug.push('-'); }
            pending_dash = false;
            for lower in character.to_lowercase() { slug.push(lower); }
        } else if !slug.is_empty() {
            pending_dash = true;
        }
    }
    slug
}

fn main() {
    for title in ["Rust: Safe & Fast", "  Parsing--Patterns  ", "Version 2.0"] {
        println!("{} -> {}", title, slugify(title));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collapses_punctuation_runs() {
        assert_eq!(slugify("Rust: Safe & Fast"), "rust-safe-fast");
    }
    #[test]
    fn trims_edges_and_handles_unicode() {
        assert_eq!(slugify("  Café au lait!  "), "café-au-lait");
    }
}
