pub fn percentile(v:&[f64],p:f64)->Option<f64>{if v.is_empty()||!(0.0..=1.0).contains(&p)||v.iter().any(|x|!x.is_finite()){return None}let x=p*(v.len()-1)as f64;let(a,b)=(x.floor()as usize,x.ceil()as usize);Some(v[a]+(v[b]-v[a])*(x-a as f64))}
#[cfg(test)]mod tests{use super::*;#[test]fn interp(){assert_eq!(percentile(&[0.,10.],0.25),Some(2.5))}}
