pub fn rotate_right<T>(v:&mut[T],n:usize){if !v.is_empty(){let k=n%v.len();v.rotate_right(k)}}
#[cfg(test)]mod tests{use super::*;#[test]fn rotates(){let mut v=[1,2,3,4];rotate_right(&mut v,6);assert_eq!(v,[3,4,1,2])}}
