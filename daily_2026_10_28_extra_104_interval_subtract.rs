use std::ops::Range;

/// Returns the portions of `target` not covered by `coverage`.
///
/// Ranges are half-open (`start..end`). `coverage` must be sorted by start;
/// overlapping and out-of-target ranges are allowed. Empty or reversed
/// ranges are ignored, and an empty or reversed target produces no result.
pub fn subtract_sorted(target: Range<i64>, coverage: &[Range<i64>]) -> Vec<Range<i64>> {
    if target.start >= target.end {
        return Vec::new();
    }

    let mut gaps = Vec::new();
    let mut cursor = target.start;
    for range in coverage {
        if range.start >= range.end || range.end <= cursor {
            continue;
        }
        if range.start >= target.end {
            break;
        }
        let covered_start = range.start.max(target.start);
        if covered_start > cursor {
            gaps.push(cursor..covered_start);
        }
        cursor = cursor.max(range.end.min(target.end));
        if cursor >= target.end {
            break;
        }
    }
    if cursor < target.end {
        gaps.push(cursor..target.end);
    }
    gaps
}

#[cfg(test)]
mod tests {
    use super::subtract_sorted;

    #[test]
    fn subtracts_clipped_overlapping_coverage() {
        assert_eq!(subtract_sorted(0..10, &[-2..2, 4..7, 6..12]), vec![2..4]);
    }

    #[test]
    fn handles_empty_coverage_and_empty_target() {
        assert_eq!(subtract_sorted(3..8, &[]), vec![3..8]);
        assert!(subtract_sorted(5..5, &[0..9]).is_empty());
        assert!(subtract_sorted(7..2, &[]).is_empty());
    }

    #[test]
    fn ignores_empty_ranges_and_keeps_touching_boundaries() {
        assert_eq!(subtract_sorted(0..8, &[0..0, 2..4, 4..4, 6..8]), vec![0..2, 4..6]);
    }
}
