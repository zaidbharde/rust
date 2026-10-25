#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

pub fn uncovered_gaps(mut intervals: Vec<Interval>, horizon: Interval) -> Vec<Interval> {
    intervals.retain(|item| item.end > horizon.start && item.start < horizon.end);
    intervals.sort_by_key(|item| item.start);
    let mut cursor = horizon.start;
    let mut gaps = Vec::new();
    for interval in intervals {
        let start = interval.start.max(horizon.start);
        let end = interval.end.min(horizon.end);
        if start > cursor {
            gaps.push(Interval { start: cursor, end: start });
        }
        cursor = cursor.max(end);
        if cursor >= horizon.end {
            break;
        }
    }
    if cursor < horizon.end {
        gaps.push(Interval { start: cursor, end: horizon.end });
    }
    gaps
}

#[cfg(test)]
mod tests {
    use super::{uncovered_gaps, Interval};

    #[test]
    fn clips_to_horizon_and_merges_overlap() {
        let gaps = uncovered_gaps(vec![Interval { start: 2, end: 5 }, Interval { start: 4, end: 9 }], Interval { start: 0, end: 10 });
        assert_eq!(gaps, vec![Interval { start: 0, end: 2 }, Interval { start: 9, end: 10 }]);
    }
}
