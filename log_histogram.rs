/// Counts non-negative values into logarithmic buckets.
pub fn histogram(values: &[u64], bucket_count: usize) -> Vec<usize> {
    if bucket_count == 0 { return Vec::new(); }
    let mut buckets = vec![0; bucket_count];
    for &value in values {
        let exponent = if value == 0 { 0 } else { (u64::BITS - value.leading_zeros() - 1) as usize + 1 };
        let index = exponent.min(bucket_count - 1);
        buckets[index] += 1;
    }
    buckets
}

/// Returns the inclusive lower bound represented by a bucket.
pub fn lower_bound(bucket: usize) -> u64 {
    if bucket == 0 { 0 } else { 1u64 << (bucket - 1).min(63) }
}

fn main() {
    let samples = [0, 1, 2, 3, 8, 15, 100];
    for (bucket, count) in histogram(&samples, 8).iter().enumerate() {
        println!("[{}..): {count}", lower_bound(bucket));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn groups_values_by_bit_width() {
        assert_eq!(histogram(&[0, 1, 2, 3, 4], 4), vec![1, 1, 2, 1]);
    }
    #[test]
    fn zero_buckets_are_supported() { assert!(histogram(&[1, 2], 0).is_empty()); }
}
