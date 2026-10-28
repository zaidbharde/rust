pub fn chunk_ranges(n:usize,size:usize)->Vec<std::ops::Range<usize>>{if size==0{return vec![]} (0..n).step_by(size).map(|i|i..(i+size).min(n)).collect()}
#[cfg(test)]mod tests{use super::*;#[test]fn partition(){assert_eq!(chunk_ranges(7,3),vec![0..3,3..6,6..7])}}
