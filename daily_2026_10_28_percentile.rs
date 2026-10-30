pub fn percentile(values: &[f64], p: f64) -> Option<f64> {
    if values.is_empty() || !(0.0..=1.0).contains(&p) || values.iter().any(|value| !value.is_finite()) {
        return None;
    }

    let position = p * (values.len() - 1) as f64;
    let lower_index = position.floor() as usize;
    let upper_index = position.ceil() as usize;
    let fraction = position - lower_index as f64;
    let lower = values[lower_index];
    let upper = values[upper_index];

    // Opposite signs can overflow in (upper - lower); weighted terms avoid that intermediate.
    let interpolated = if lower.is_sign_positive() != upper.is_sign_positive() {
        lower * (1.0 - fraction) + upper * fraction
    } else {
        lower + (upper - lower) * fraction
    };
    Some(interpolated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolates_ordinary_values() {
        assert_eq!(percentile(&[0.0, 10.0], 0.25), Some(2.5));
    }

    #[test]
    fn opposite_extremes_do_not_overflow_the_difference() {
        assert_eq!(percentile(&[-f64::MAX, f64::MAX], 0.5), Some(0.0));
    }
}
