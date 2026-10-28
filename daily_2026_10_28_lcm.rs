pub fn lcm(a:u64,b:u64)->Option<u64>{if a==0||b==0{return Some(0)}(a/gcd(a,b)).checked_mul(b)}fn gcd(mut a:u64,mut b:u64)->u64{while b!=0{let r=a%b;a=b;b=r}a}
#[cfg(test)]mod tests{use super::*;#[test]fn safe(){assert_eq!(lcm(6,8),Some(24));assert_eq!(lcm(u64::MAX,2),None)}}
