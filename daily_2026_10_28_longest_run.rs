pub fn longest_run<T:PartialEq>(v:&[T])->usize{let(mut best,mut n)=(0,0);for i in 0..v.len(){if i==0||v[i]!=v[i-1]{n=1}else{n+=1}best=best.max(n)}best}
#[cfg(test)]mod tests{use super::*;#[test]fn run(){assert_eq!(longest_run(&[1,1,2,3,3,3]),3)}}
