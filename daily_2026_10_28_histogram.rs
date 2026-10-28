use std::collections::BTreeMap;pub fn histogram(v:&[i64])->BTreeMap<i64,usize>{let mut m=BTreeMap::new();for &x in v{*m.entry(x).or_insert(0)+=1}m}
#[cfg(test)]mod tests{use super::*;#[test]fn counts(){assert_eq!(histogram(&[3,1,3])[&3],2)}}
