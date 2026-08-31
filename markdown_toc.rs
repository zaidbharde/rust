#[derive(Debug, PartialEq, Eq)]
struct Heading {
    level: usize,
    title: String,
    anchor: String,
}

fn slugify(title: &str) -> String {
    title.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

fn headings(markdown: &str) -> Vec<Heading> {
    markdown.lines().filter_map(|line| {
        let level = line.chars().take_while(|c| *c == '#').count();
        if (1..=6).contains(&level) && line.chars().nth(level) == Some(' ') {
            let title = line[level + 1..].trim().to_string();
            Some(Heading { anchor: slugify(&title), level, title })
        } else {
            None
        }
    }).collect()
}

fn render_toc(markdown: &str) -> String {
    headings(markdown).iter().map(|heading| {
        let indent = "  ".repeat(heading.level.saturating_sub(1));
        format!("{}- [{}](#{})", indent, heading.title, heading.anchor)
    }).collect::<Vec<_>>().join("\n")
}

fn main() {
    let document = "# Guide\n## Install Tools\n### Verify Output\n## Next Steps";
    println!("{}", render_toc(document));
}
