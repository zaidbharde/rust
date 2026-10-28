pub fn binary_gap(mut n:u64)->usize{let(mut best,mut run)=(0,None);while n>0{if n&1==1{if let Some(k)=run{best=best.max(k)}run=Some(0)}else if let Some(k)=run{run=Some(k+1)}n>>=1}best}
#[cfg(test)]mod tests{use super::*;#[test]fn gap(){assert_eq!(binary_gap(9),2);assert_eq!(binary_gap(32),0)}}
