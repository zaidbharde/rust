//! Compact hexadecimal dump formatting with offsets and printable previews.

pub fn format_dump(bytes: &[u8], width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let mut output = String::new();
    for (line, chunk) in bytes.chunks(width).enumerate() {
        let offset = line * width;
        output.push_str(&format!("{offset:08x}  "));
        for index in 0..width {
            match chunk.get(index) {
                Some(byte) => output.push_str(&format!("{byte:02x} ")),
                None => output.push_str("   "),
            }
            if index + 1 == width / 2 {
                output.push(' ');
            }
        }
        output.push_str(" | ");
        output.extend(chunk.iter().map(|byte| {
            if byte.is_ascii_graphic() || *byte == b' ' {
                *byte as char
            } else {
                '.'
            }
        }));
        output.push('|');
        output.push('\n');
    }
    output
}

pub fn parse_hex(input: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = input.chars().filter(|character| !character.is_whitespace()).collect();
    if cleaned.len() % 2 != 0 {
        return Err("hex input must contain pairs of digits".to_string());
    }
    (0..cleaned.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&cleaned[index..index + 2], 16)
            .map_err(|_| format!("invalid byte at position {index}")))
        .collect()
}
