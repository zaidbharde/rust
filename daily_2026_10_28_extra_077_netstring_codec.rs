//! Encode and decode byte strings in the `length:payload,` netstring format.

/// Encodes arbitrary bytes as one netstring.
pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut out = input.len().to_string().into_bytes();
    out.push(b':');
    out.extend_from_slice(input);
    out.push(b',');
    out
}

/// Decodes exactly one netstring, rejecting malformed lengths and trailing data.
pub fn decode(input: &[u8]) -> Result<Vec<u8>, &'static str> {
    let colon = input.iter().position(|&b| b == b':').ok_or("missing colon")?;
    let digits = &input[..colon];
    if digits.is_empty() || (digits.len() > 1 && digits[0] == b'0') {
        return Err("invalid length");
    }
    let mut len = 0usize;
    for &b in digits {
        if !b.is_ascii_digit() { return Err("invalid length"); }
        len = len.checked_mul(10).and_then(|n| n.checked_add((b - b'0') as usize))
            .ok_or("length overflow")?;
    }
    let end = colon.checked_add(1).and_then(|n| n.checked_add(len)).ok_or("length overflow")?;
    if input.len() != end + 1 || input.get(end) != Some(&b',') { return Err("invalid payload or terminator"); }
    Ok(input[colon + 1..end].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_arbitrary_bytes_and_empty() {
        for bytes in [&b"hi,\0"[..], &[][..]] { assert_eq!(decode(&encode(bytes)), Ok(bytes.to_vec())); }
        assert_eq!(encode(b"cat"), b"3:cat,");
    }

    #[test]
    fn rejects_malformed_netstrings() {
        for bad in [&b""[..], b":x," as &[u8], b"01:x,", b"2:x,", b"1:x", b"1:x,z", b"x:a,"] {
            assert!(decode(bad).is_err(), "accepted {bad:?}");
        }
        assert_eq!(decode(b"999999999999999999999999999999999999:a,"), Err("length overflow"));
    }

    #[test]
    fn demonstration() { assert_eq!(decode(&encode(b"netstring")), Ok(b"netstring".to_vec())); }
}
