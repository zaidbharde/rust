//! Strict hexadecimal encoding and decoding helpers.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexError { OddLength, InvalidDigit { position: usize, byte: u8 } }

pub fn encode(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}

pub fn decode(text: &str) -> Result<Vec<u8>, HexError> {
    let input = text.as_bytes();
    if input.len() % 2 != 0 { return Err(HexError::OddLength); }
    let mut result = Vec::with_capacity(input.len() / 2);
    for position in (0..input.len()).step_by(2) {
        let high = digit(input[position]).ok_or(HexError::InvalidDigit { position, byte: input[position] })?;
        let low = digit(input[position + 1]).ok_or(HexError::InvalidDigit { position: position + 1, byte: input[position + 1] })?;
        result.push((high << 4) | low);
    }
    Ok(result)
}

fn digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
