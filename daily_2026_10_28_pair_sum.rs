use std::collections::HashSet;pub fn has_pair_sum(v:&[i64],target:i64)->bool{let mut seen=HashSet::new();for &x in v{if let Some(y)=target.checked_sub(x){if seen.contains(&y){return true}}seen.insert(x);}false}
#[cfg(test)]mod tests{use super::*;#[test]fn distinct_indices(){assert!(has_pair_sum(&[4,6],10));assert!(!has_pair_sum(&[5],10))}}
