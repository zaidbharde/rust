use std::collections::HashMap;

/// Resolve a slash-separated pointer against a flat key/value configuration map.
pub fn pointer_lookup<'a>(config: &'a HashMap<String, String>, pointer: &str) -> Option<&'a str> {
    let key = pointer
        .strip_prefix('/')?
        .split('/')
        .map(unescape_segment)
        .collect::<Vec<_>>()
        .join(".");
    config.get(&key).map(String::as_str)
}

fn unescape_segment(segment: &str) -> String {
    let mut decoded = String::with_capacity(segment.len());
    let mut chars = segment.chars();
    while let Some(character) = chars.next() {
        if character == '~' {
            match chars.next() {
                Some('0') => decoded.push('~'),
                Some('1') => decoded.push('/'),
                Some(other) => { decoded.push('~'); decoded.push(other); }
                None => decoded.push('~'),
            }
        } else {
            decoded.push(character);
        }
    }
    decoded
}

fn main() {
    let values = HashMap::from([(String::from("service.retry.limit"), String::from("5"))]);
    println!("{:?}", pointer_lookup(&values, "/service/retry/limit"));
}
