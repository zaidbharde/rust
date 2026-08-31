const POLYNOMIAL: u32 = 0xEDB88320;

pub fn crc32(data: &[u8]) -> u32 {
    let mut checksum = u32::MAX;
    for &byte in data {
        checksum ^= byte as u32;
        for _ in 0..8 {
            let bit_set = checksum & 1 != 0;
            checksum >>= 1;
            if bit_set { checksum ^= POLYNOMIAL; }
        }
    }
    !checksum
}

fn hex(value: u32) -> String {
    format!("{value:08x}")
}

fn main() {
    let payload = b"repository verification";
    println!("CRC-32: {}", hex(crc32(payload)));
}

#[cfg(test)]
mod tests {
    use super::crc32;

    #[test]
    fn known_vector_matches() {
        assert_eq!(crc32(b"123456789"), 0xcbf43926);
    }
}
