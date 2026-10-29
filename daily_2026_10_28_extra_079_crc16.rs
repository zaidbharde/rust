//! Compute CRC-16/CCITT-FALSE checksums over byte slices.

/// Computes CRC-16/CCITT-FALSE (poly `0x1021`, init `0xFFFF`, xorout `0`).
///
/// Processes each byte most-significant bit first; the empty slice returns `0xFFFF`.
pub fn crc16_ccitt(input: &[u8]) -> u16 {
    let mut crc = 0xFFFFu16;
    for &byte in input {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x1021
            } else {
                crc << 1
            };
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::crc16_ccitt;

    #[test]
    fn known_standard_check_value() {
        assert_eq!(crc16_ccitt(b"123456789"), 0x29B1);
    }

    #[test]
    fn handles_empty_and_binary_input() {
        assert_eq!(crc16_ccitt(b""), 0xFFFF);
        assert_eq!(crc16_ccitt(&[0x00, 0xFF, 0x80, 0x01]), 0x401A);
    }

    #[test]
    fn demonstration() {
        let checksum = crc16_ccitt(b"hello");
        assert_eq!(checksum, 0xD26E);
    }
}
