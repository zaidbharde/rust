pub fn unique_by<T,K:Eq+std::hash::Hash,F:FnMut(&T)->K>(v:impl IntoIterator<Item=T>,mut key:F)->Vec<T>{use std::collections::HashSet;let mut seen=HashSet::new();v.into_iter().filter(|x|seen.insert(key(x))).collect()}
#[cfg(test)]mod tests{use super::*;#[test]fn first_key(){assert_eq!(unique_by(["ant","ape","bee"],|s|s.chars().next().unwrap()),vec!["ant","bee"])}}
