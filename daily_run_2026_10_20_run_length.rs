//! Run-length encoding for byte slices with overflow-safe decoding.

pub fn encode(input: &[u8]) -> Vec<(u8, u32)> {
    let mut encoded = Vec::new();
    let mut index = 0;
    while index < input.len() {
        let value = input[index];
        let mut length = 1u32;
        while index + length as usize < input.len()
            && input[index + length as usize] == value
            && length < u32::MAX
        {
            length += 1;
        }
        encoded.push((value, length));
        index += length as usize;
    }
    encoded
}

pub fn decode(encoded: &[(u8, u32)]) -> Result<Vec<u8>, &'static str> {
    let total = encoded.iter().try_fold(0usize, |sum, (_, count)| {
        if *count == 0 { return Err("run lengths must be positive"); }
        sum.checked_add(*count as usize).ok_or("decoded output is too large")
    })?;
    let mut output = Vec::with_capacity(total);
    for (value, count) in encoded { output.extend(std::iter::repeat_n(*value, *count as usize)); }
    Ok(output)
}
