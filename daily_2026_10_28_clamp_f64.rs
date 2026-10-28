pub fn clamp_finite(x:f64,lo:f64,hi:f64)->Option<f64>{if !x.is_finite()||!lo.is_finite()||!hi.is_finite()||lo>hi{None}else{Some(x.max(lo).min(hi))}}
#[cfg(test)]mod tests{use super::*;#[test]fn bounds(){assert_eq!(clamp_finite(9.,0.,4.),Some(4.));assert_eq!(clamp_finite(f64::NAN,0.,1.),None)}}
