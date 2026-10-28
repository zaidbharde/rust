pub fn prefix_sums(v:&[i64])->Vec<i64>{let mut s=0;v.iter().map(|&x|{s+=x;s}).collect()}
#[cfg(test)]mod tests{use super::*;#[test]fn inclusive(){assert_eq!(prefix_sums(&[2,-1,4]),vec![2,1,5])}}
