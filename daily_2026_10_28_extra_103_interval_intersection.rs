//! Intersect two sorted, disjoint collections of closed integer intervals.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: i64,
    pub end: i64,
}

impl Interval {
    pub fn new(start: i64, end: i64) -> Result<Self, &'static str> {
        if start > end {
            return Err("interval start cannot exceed end");
        }
        Ok(Self { start, end })
    }
}

/// Return the pairwise overlap intervals in ascending order.
///
/// Inputs must each be sorted and disjoint as closed intervals. Empty inputs
/// are valid; touching intervals within one input are not disjoint.
pub fn intersect(a: &[Interval], b: &[Interval]) -> Result<Vec<Interval>, &'static str> {
    fn valid(items: &[Interval]) -> bool {
        items.iter().all(|x| x.start <= x.end)
            && items.windows(2).all(|w| w[0].end < w[1].start)
    }
    if !valid(a) || !valid(b) {
        return Err("intervals must be ordered, sorted, and disjoint");
    }

    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        let start = a[i].start.max(b[j].start);
        let end = a[i].end.min(b[j].end);
        if start <= end {
            result.push(Interval { start, end });
        }
        if a[i].end < b[j].end {
            i += 1;
        } else {
            j += 1;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{intersect, Interval};

    fn iv(start: i64, end: i64) -> Interval {
        Interval { start, end }
    }

    #[test]
    fn finds_overlaps_and_preserves_closed_endpoints() {
        let a = [iv(0, 2), iv(5, 9), iv(12, 15)];
        let b = [iv(2, 6), iv(8, 13)];
        assert_eq!(intersect(&a, &b).unwrap(), [iv(2, 2), iv(5, 6), iv(8, 9), iv(12, 13)]);
    }

    #[test]
    fn empty_and_non_overlapping_inputs() {
        assert_eq!(intersect(&[], &[iv(1, 3)]).unwrap(), Vec::<Interval>::new());
        assert_eq!(intersect(&[iv(1, 2)], &[iv(3, 4)]).unwrap(), Vec::<Interval>::new());
    }

    #[test]
    fn rejects_invalid_or_unsorted_collections() {
        assert!(Interval::new(4, 3).is_err());
        assert!(intersect(&[iv(4, 5), iv(1, 2)], &[]).is_err());
        assert!(intersect(&[iv(1, 3), iv(3, 4)], &[]).is_err());
    }
}
