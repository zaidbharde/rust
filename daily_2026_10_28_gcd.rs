pub fn gcd(mut a:u64,mut b:u64)->u64{while b!=0{let r=a%b;a=b;b=r}a}
#[cfg(test)]mod tests{use super::*;#[test]fn euclid(){assert_eq!(gcd(54,24),6)}}
