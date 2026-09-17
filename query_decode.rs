use std::collections::BTreeMap;

/// Decodes `key=value` pairs separated by `&`, with `+` as a space.
pub fn decode_query(input: &str) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for pair in input.split('&').filter(|part| !part.is_empty()) {
        let (raw_key, raw_value) = pair.split_once('=').unwrap_or((pair, ""));
        let key = decode_component(raw_key);
        let value = decode_component(raw_value);
        result.insert(key, value);
    }
    result
}

fn decode_component(component: &str) -> String {
    let bytes = component.as_bytes();
    let mut output = String::new();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => { output.push(' '); index += 1; }
            b'%' if index + 2 < bytes.len() => {
                let hex = &component[index + 1..index + 3];
                if let Ok(value) = u8::from_str_radix(hex, 16) {
                    output.push(value as char); index += 3; continue;
                }
                output.push('%'); index += 1;
            }
            value => { output.push(value as char); index += 1; }
        }
    }
    output
}

fn main() {
    println!("{:?}", decode_query("q=rust+lang&page=2&empty"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decodes_spaces_and_missing_values() {
        let values = decode_query("q=rust+lang&empty");
        assert_eq!(values["q"], "rust lang");
        assert_eq!(values["empty"], "");
    }
}
