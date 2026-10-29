//! Encode and decode canonical unsigned LEB128 varints for `u64` values.

/// Encodes a `u64` as an unsigned variable-length integer.
pub fn encode(mut value: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(10);
    loop {
        let byte = (value & 0x7F) as u8;
        value >>= 7;
        out.push(byte | if value != 0 { 0x80 } else { 0 });
        if value == 0 { return out; }
    }
}

/// Decodes exactly one canonical varint, rejecting truncation and overflow.
pub fn decode(input: &[u8]) -> Result<u64, &'static str> {
    let mut value = 0u64;
    for (i, &byte) in input.iter().enumerate() {
        if i >= 10 { return Err("varint too long"); }
        let payload = (byte & 0x7F) as u64;
        if i == 9 && (payload > 1 || byte & 0x80 != 0) { return Err("u64 overflow"); }
        value |= payload << (i * 7);
        if byte & 0x80 == 0 {
            if i + 1 != input.len() { return Err("trailing bytes"); }
            if encode(value).as_slice() != input { return Err("non-canonical varint"); }
            return Ok(value);
        }
    }
    Err("truncated varint")
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn round_trips_boundaries_and_demo() {
        for value in [0, 1, 127, 128, 16_383, 16_384, u32::MAX as u64, u64::MAX] {
            assert_eq!(decode(&encode(value)), Ok(value));
        }
        assert_eq!(encode(300), [0xAC, 0x02]);
        assert_eq!(decode(&[0xAC, 0x02]), Ok(300));
    }

    #[test]
    fn rejects_malformed_inputs() {
        for bytes in [&[][..], &[0x80][..], &[0x80, 0][..], &[0, 1][..],
                      &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 2][..],
                      &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0][..]] {
            assert!(decode(bytes).is_err(), "accepted {bytes:?}");
        }
    }
}
