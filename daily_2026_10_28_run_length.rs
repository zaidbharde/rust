pub fn encode_runs(s:&str)->Vec<(char,usize)>{let mut out:Vec<(char,usize)>=Vec::new();for c in s.chars(){if let Some((last,n))=out.last_mut(){if *last==c{*n+=1;continue}}out.push((c,1))}out}
#[cfg(test)]mod tests{use super::*;#[test]fn unicode(){assert_eq!(encode_runs("aaβ"),vec![('a',2),('β',1)])}}
