//! Resolve slash-separated pointers against a compact JSON-like value model.

#[derive(Debug, PartialEq)]
pub enum Value { Null, Bool(bool), Number(f64), String(String), Array(Vec<Value>), Object(Vec<(String, Value)>) }

#[derive(Debug, PartialEq, Eq)]
pub enum PointerError { InvalidEscape, MissingKey, InvalidIndex, OutOfBounds }

pub fn resolve<'a>(root: &'a Value, pointer: &str) -> Result<&'a Value, PointerError> {
    if pointer.is_empty() { return Ok(root); }
    if !pointer.starts_with('/') { return Err(PointerError::MissingKey); }
    pointer.split('/').skip(1).try_fold(root, |current, raw| {
        let segment = decode(raw)?;
        match current {
            Value::Object(entries) => entries.iter().find(|(key, _)| key == &segment)
                .map(|(_, value)| value).ok_or(PointerError::MissingKey),
            Value::Array(items) => {
                if segment == "-" { return Err(PointerError::InvalidIndex); }
                let index = segment.parse::<usize>().map_err(|_| PointerError::InvalidIndex)?;
                items.get(index).ok_or(PointerError::OutOfBounds)
            }
            _ => Err(PointerError::MissingKey),
        }
    })
}

fn decode(segment: &str) -> Result<String, PointerError> {
    let mut output = String::new();
    let mut chars = segment.chars();
    while let Some(ch) = chars.next() {
        if ch != '~' { output.push(ch); continue; }
        match chars.next() { Some('0') => output.push('~'), Some('1') => output.push('/'), _ => return Err(PointerError::InvalidEscape) }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolves_escaped_object_keys() {
        let value = Value::Object(vec![("a/b".into(), Value::Number(7.0))]);
        assert_eq!(resolve(&value, "/a~1b"), Ok(&Value::Number(7.0)));
    }
    #[test]
    fn resolves_array_indices() {
        let value = Value::Array(vec![Value::Bool(true)]);
        assert_eq!(resolve(&value, "/0"), Ok(&Value::Bool(true)));
    }
}
