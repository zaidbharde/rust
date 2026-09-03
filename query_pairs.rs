use std::collections::BTreeMap;

fn decode(input: &str) -> Result<String, String> {
    let bytes = input.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => output.push(b' '),
            b'%' if index + 2 < bytes.len() => {
                let hex = &input[index + 1..index + 3];
                output.push(u8::from_str_radix(hex, 16).map_err(|_| "bad escape")?);
                index += 2;
            }
            b'%' => return Err("truncated escape".into()),
            byte => output.push(byte),
        }
        index += 1;
    }
    String::from_utf8(output).map_err(|_| "invalid UTF-8".into())
}

pub fn parse_query(query: &str) -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut pairs = BTreeMap::new();
    for field in query.trim_start_matches('?').split('&') {
        if field.is_empty() { continue; }
        let (key, value) = field.split_once('=').unwrap_or((field, ""));
        pairs.entry(decode(key)?).or_insert_with(Vec::new).push(decode(value)?);
    }
    Ok(pairs)
}

fn main() {
    println!("{:?}", parse_query("?tag=rust&tag=systems%20code&debug"));
}
