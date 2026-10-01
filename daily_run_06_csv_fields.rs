pub fn parse_record(input: &str) -> Result<Vec<String>, &'static str> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                current.push('"'); chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => { fields.push(std::mem::take(&mut current)); }
            '\n' | '\r' if !quoted => return Err("record contains a newline"),
            _ => current.push(ch),
        }
    }
    if quoted { return Err("unterminated quoted field"); }
    fields.push(current);
    Ok(fields)
}

pub fn escape_field(field: &str) -> String {
    if field.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else { field.to_owned() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_commas_inside_quotes() {
        assert_eq!(parse_record("one,\"two, too\",three").unwrap(),
            vec!["one", "two, too", "three"]);
    }
}
