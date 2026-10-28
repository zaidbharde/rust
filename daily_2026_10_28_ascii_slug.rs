pub fn ascii_slug(s:&str)->String{let mut o=String::new();let mut sep=false;for c in s.chars(){if c.is_ascii_alphanumeric(){if sep&&!o.is_empty(){o.push('-')}o.push(c.to_ascii_lowercase());sep=false}else{sep=true}}o}
#[cfg(test)]mod tests{use super::*;#[test]fn works(){assert_eq!(ascii_slug(" Rust & Tools! "),"rust-tools")}}
