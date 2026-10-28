pub fn dedupe_sorted<T:PartialEq>(v:&mut Vec<T>){v.dedup()}
#[cfg(test)]mod tests{use super::*;#[test]fn adjacent(){let mut v=vec![1,1,2,2,4];dedupe_sorted(&mut v);assert_eq!(v,vec![1,2,4])}}
