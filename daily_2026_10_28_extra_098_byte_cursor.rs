//! Checked sequential reads of big-endian integers from a byte slice.

/// A cursor over immutable bytes; failed reads leave its position unchanged.
#[derive(Debug, Clone)]
pub struct ByteCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> ByteCursor<'a> {
    /// Creates a cursor positioned at the start of `bytes`.
    pub fn new(bytes: &'a [u8]) -> Self { Self { bytes, position: 0 } }

    /// Current byte offset from the beginning of the input.
    pub fn position(&self) -> usize { self.position }

    /// Number of unread bytes.
    pub fn remaining(&self) -> usize { self.bytes.len() - self.position }

    fn take<const N: usize>(&mut self) -> Result<[u8; N], &'static str> {
        let end = self.position.checked_add(N).ok_or("cursor position overflow")?;
        let slice = self.bytes.get(self.position..end).ok_or("unexpected end of input")?;
        let value = slice.try_into().map_err(|_| "unexpected end of input")?;
        self.position = end;
        Ok(value)
    }

    /// Reads one unsigned byte.
    pub fn read_u8(&mut self) -> Result<u8, &'static str> { Ok(self.take::<1>()?[0]) }

    /// Reads an unsigned 16-bit big-endian integer.
    pub fn read_u16_be(&mut self) -> Result<u16, &'static str> { Ok(u16::from_be_bytes(self.take()?)) }

    /// Reads an unsigned 32-bit big-endian integer.
    pub fn read_u32_be(&mut self) -> Result<u32, &'static str> { Ok(u32::from_be_bytes(self.take()?)) }

    /// Reads an unsigned 64-bit big-endian integer.
    pub fn read_u64_be(&mut self) -> Result<u64, &'static str> { Ok(u64::from_be_bytes(self.take()?)) }
}

#[cfg(test)]
mod tests {
    use super::ByteCursor;

    #[test]
    fn reads_big_endian_values_sequentially() {
        let mut c = ByteCursor::new(&[0xAB, 0x12, 0x34, 0x01, 0x02]);
        assert_eq!(c.read_u8(), Ok(0xAB));
        assert_eq!(c.read_u16_be(), Ok(0x1234));
        assert_eq!(c.read_u32_be(), Err("unexpected end of input"));
        assert_eq!(c.position(), 3);
        assert_eq!(c.remaining(), 2);
    }

    #[test]
    fn handles_empty_and_exact_width_inputs() {
        let mut empty = ByteCursor::new(&[]);
        assert!(empty.read_u64_be().is_err());
        assert_eq!(empty.position(), 0);
        let mut full = ByteCursor::new(&[0xFF; 8]);
        assert_eq!(full.read_u64_be(), Ok(u64::MAX));
        assert_eq!(full.remaining(), 0);
    }
}
