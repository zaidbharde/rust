//! Small shell-like tokenizer supporting single and double quoted segments.

pub fn tokenize(input: &str) -> Result<Vec<String>, &'static str> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for character in input.chars() {
        if escaped { current.push(character); escaped = false; continue; }
        if character == '\\' && quote != Some('\'') { escaped = true; continue; }
        match (quote, character) {
            (Some(active), value) if value == active => quote = None,
            (None, '\'') | (None, '"') => quote = Some(character),
            (None, value) if value.is_whitespace() => {
                if !current.is_empty() { tokens.push(std::mem::take(&mut current)); }
            }
            (_, value) => current.push(value),
        }
    }
    if escaped || quote.is_some() { return Err("unterminated escape or quote"); }
    if !current.is_empty() { tokens.push(current); }
    Ok(tokens)
}
