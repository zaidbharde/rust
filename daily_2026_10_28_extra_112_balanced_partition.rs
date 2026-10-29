//! Split nonnegative integer weights into two groups with sums as close as possible.
/// Returns indices for the first group and the sums of both groups.
///
/// The exact subset-sum dynamic program is pseudo-polynomial in half the total.
/// Negative weights and arithmetic/allocation-size overflow are reported as errors.
pub fn balanced_partition(weights: &[u64]) -> Result<(Vec<usize>, Vec<usize>, u64, u64), &'static str> {
    let total = weights.iter().try_fold(0u64, |sum, &w| sum.checked_add(w))
        .ok_or("total weight overflow")?;
    let target = usize::try_from(total / 2).map_err(|_| "target is too large")?;
    let width = target.checked_add(1).ok_or("target is too large")?;
    let cells = weights.len().checked_add(1).and_then(|n| n.checked_mul(width))
        .ok_or("dynamic-programming table is too large")?;
    let mut reachable = Vec::new();
    reachable.try_reserve_exact(cells).map_err(|_| "dynamic-programming table allocation failed")?;
    reachable.resize(cells, false);
    reachable[0] = true;
    for (i, &weight) in weights.iter().enumerate() {
        let w = usize::try_from(weight).unwrap_or(usize::MAX);
        for sum in 0..=target {
            let prior = i * width + sum;
            if reachable[prior] {
                let next = sum.checked_add(w);
                if let Some(next) = next.filter(|&s| s <= target) {
                    reachable[(i + 1) * width + next] = true;
                }
            }
        }
        for sum in 0..=target {
            reachable[(i + 1) * width + sum] |= reachable[i * width + sum];
        }
    }
    let best = (0..=target).rev().find(|&s| reachable[weights.len() * width + s]).unwrap_or(0);
    let mut chosen = Vec::new();
    let mut sum = best;
    for i in (0..weights.len()).rev() {
        let row = (i + 1) * width;
        if !reachable[i * width + sum] {
            chosen.push(i);
            sum -= usize::try_from(weights[i]).map_err(|_| "weight is too large")?;
        }
        let _ = row;
    }
    chosen.reverse();
    let mut selected = vec![false; weights.len()];
    for &i in &chosen { selected[i] = true; }
    let other: Vec<usize> = (0..weights.len()).filter(|&i| !selected[i]).collect();
    let first_sum = chosen.iter().map(|&i| weights[i]).sum();
    let second_sum = total - first_sum;
    Ok((chosen, other, first_sum, second_sum))
}

#[cfg(test)]
mod tests {
    use super::balanced_partition;

    #[test]
    fn partitions_to_optimal_sums() {
        let weights = [8, 7, 6, 5];
        let (a, b, sa, sb) = balanced_partition(&weights).unwrap();
        assert_eq!(sa.abs_diff(sb), 0);
        assert_eq!(a.len() + b.len(), weights.len());
        assert_eq!(sa + sb, 26);
    }

    #[test]
    fn handles_empty_zero_and_oversized_weights() {
        assert_eq!(balanced_partition(&[]).unwrap(), (vec![], vec![], 0, 0));
        assert_eq!(balanced_partition(&[0, 0]).unwrap().2..=0, 0..=0);
        let result = balanced_partition(&[u64::MAX, 1]);
        assert!(result.is_err());
    }
}
