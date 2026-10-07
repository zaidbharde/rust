//! Small CSV-like key-value parser supporting quoted values and escapes.

use std::collections::BTreeMap;

pub fn parse(input: &str) -> Result<BTreeMap<String, String>, &'static str> {
    let mut output = BTreeMap::new();
    for field in input.split(',') {
        let (raw_key, raw_value) = field.split_once('=').ok_or("missing '=' separator")?;
        let key = raw_key.trim();
        if key.is_empty() { return Err("key cannot be empty"); }
        let value = decode_value(raw_value.trim())?;
        if output.insert(key.to_string(), value).is_some() { return Err("duplicate key"); }
    }
    Ok(output)
}

fn decode_value(raw: &str) -> Result<String, &'static str> {
    if !raw.starts_with('"') { return Ok(raw.to_string()); }
    if !raw.ends_with('"') || raw.len() < 2 { return Err("unterminated quoted value"); }
    let body = &raw[1..raw.len() - 1];
    let mut value = String::with_capacity(body.len());
    let mut escaped = false;
    for character in body.chars() {
        if escaped { value.push(character); escaped = false; }
        else if character == '\\' { escaped = true; }
        else { value.push(character); }
    }
    if escaped { return Err("dangling escape"); }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::parse;
    #[test]
    fn parses_quoted_commas() {
        assert_eq!(parse("mode=fast,name=\"a,b\"").unwrap()["name"], "a,b");
    }
}
