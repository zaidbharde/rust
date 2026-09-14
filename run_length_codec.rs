pub fn encode(input: &[u8]) -> Vec<(u8, u8)> {
    let mut output = Vec::new();
    for &byte in input {
        match output.last_mut() {
            Some((value, count)) if *value == byte && *count < u8::MAX => *count += 1,
            _ => output.push((byte, 1)),
        }
    }
    output
}

pub fn decode(runs: &[(u8, u8)]) -> Vec<u8> {
    let mut output = Vec::new();
    for &(byte, count) in runs {
        output.extend(std::iter::repeat_n(byte, count as usize));
    }
    output
}

fn main() {
    let source = b"aaabbcccccc";
    let packed = encode(source);
    println!("{:?} -> {:?}", packed, decode(&packed));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trip_preserves_bytes() {
        let source = b"aabbbbbccddda";
        assert_eq!(decode(&encode(source)), source);
    }
    #[test]
    fn empty_input_has_no_runs() { assert!(encode(&[]).is_empty()); }
}
