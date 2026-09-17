/// Encodes adjacent equal values as `(value, count)` pairs.
pub fn encode(values: &[u8]) -> Vec<(u8, usize)> {
    let mut encoded = Vec::new();
    for &value in values {
        match encoded.last_mut() {
            Some((last, count)) if *last == value => *count += 1,
            _ => encoded.push((value, 1)),
        }
    }
    encoded
}

/// Expands run pairs, rejecting zero-length runs.
pub fn decode(runs: &[(u8, usize)]) -> Result<Vec<u8>, &'static str> {
    let mut output = Vec::new();
    for &(value, count) in runs {
        if count == 0 {
            return Err("run length must be positive");
        }
        output.extend(std::iter::repeat_n(value, count));
    }
    Ok(output)
}

fn main() {
    let input = b"aaabbccccd";
    let packed = encode(input);
    println!("{packed:?}");
    println!("{:?}", decode(&packed).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_preserves_bytes() {
        let input = b"aabbbcccc";
        assert_eq!(decode(&encode(input)).unwrap(), input);
    }

    #[test]
    fn zero_runs_are_rejected() {
        assert_eq!(decode(&[(7, 0)]), Err("run length must be positive"));
    }
}
