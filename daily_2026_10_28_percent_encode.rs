pub fn percent_encode(s:&str)->String{let mut o=String::new();for b in s.bytes(){if b.is_ascii_alphanumeric()||b"-_.~".contains(&b){o.push(b as char)}else{o.push_str(&format!("%{b:02X}"))}}o}
#[cfg(test)]mod tests{use super::*;#[test]fn utf8(){assert_eq!(percent_encode("a b/é"),"a%20b%2F%C3%A9")}}
