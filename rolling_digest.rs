const BASE: u64 = 1_000_003;

/// Prefix hashes support O(1) extraction of a byte-slice fingerprint.
pub struct RollingDigest {
    prefix: Vec<u64>,
    powers: Vec<u64>,
}

impl RollingDigest {
    pub fn new(text: &[u8]) -> Self {
        let mut prefix = vec![0];
        let mut powers = vec![1];
        for (index, &byte) in text.iter().enumerate() {
            prefix.push(prefix[index].wrapping_mul(BASE).wrapping_add(byte as u64 + 1));
            powers.push(powers[index].wrapping_mul(BASE));
        }
        Self { prefix, powers }
    }

    pub fn digest(&self, start: usize, end: usize) -> Option<u64> {
        if start > end || end >= self.prefix.len() {
            return None;
        }
        Some(self.prefix[end].wrapping_sub(
            self.prefix[start].wrapping_mul(self.powers[end - start]),
        ))
    }

    pub fn equal_ranges(&self, left: (usize, usize), right: (usize, usize)) -> bool {
        left.1 - left.0 == right.1 - right.0
            && self.digest(left.0, left.1) == self.digest(right.0, right.1)
    }
}

fn main() {
    let text = b"abracadabra";
    let digest = RollingDigest::new(text);
    println!("repeated slice: {}", digest.equal_ranges((0, 3), (7, 10)));
}

#[cfg(test)]
mod tests {
    use super::RollingDigest;

    #[test]
    fn equal_slices_have_equal_digests() {
        let digest = RollingDigest::new(b"abcabc");
        assert!(digest.equal_ranges((0, 3), (3, 6)));
    }
}
