use std::ops::Range;

/// Returns pairs of indexes whose half-open time ranges overlap.
pub fn conflicting_pairs(ranges: &[Range<i32>]) -> Vec<(usize, usize)> {
    let mut conflicts = Vec::new();
    for left in 0..ranges.len() {
        for right in (left + 1)..ranges.len() {
            if ranges[left].start < ranges[right].end
                && ranges[right].start < ranges[left].end
            {
                conflicts.push((left, right));
            }
        }
    }
    conflicts
}

fn main() {
    let meetings = vec![0..30, 20..45, 50..70, 60..90];
    for (left, right) in conflicting_pairs(&meetings) {
        println!("meeting {left} conflicts with meeting {right}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_ranges_are_not_conflicts() {
        assert!(conflicting_pairs(&[0..5, 5..9]).is_empty());
    }

    #[test]
    fn nested_ranges_are_reported() {
        assert_eq!(conflicting_pairs(&[1..10, 3..4]), vec![(0, 1)]);
    }
}
