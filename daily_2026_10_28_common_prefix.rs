pub fn common_prefix<'a>(s:&'a[&str])->&'a str{let Some(first)=s.first()else{return ""};let mut end=0;for(i,c)in first.char_indices(){let n=i+c.len_utf8();if s.iter().all(|x|x.get(..n)==Some(&first[..n])){end=n}else{break}}&first[..end]}
#[cfg(test)]mod tests{use super::*;#[test]fn unicode(){assert_eq!(common_prefix(&["café","cafeteria"]),"caf")}}
