pub fn parse_bool(s:&str)->Option<bool>{match s.trim().to_ascii_lowercase().as_str(){"true"|"yes"|"1"|"on"=>Some(true),"false"|"no"|"0"|"off"=>Some(false),_=>None}}
#[cfg(test)]mod tests{use super::*;#[test]fn forms(){assert_eq!(parse_bool(" YES "),Some(true));assert_eq!(parse_bool("maybe"),None)}}
