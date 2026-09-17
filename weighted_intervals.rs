#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Job { pub start: i32, pub finish: i32, pub reward: i32 }

/// Selects non-overlapping jobs with maximum total reward.
pub fn maximize(mut jobs: Vec<Job>) -> (i32, Vec<Job>) {
    jobs.sort_by_key(|job| job.finish);
    let mut best = vec![0; jobs.len() + 1];
    for index in 1..=jobs.len() {
        let current = &jobs[index - 1];
        let previous = jobs[..index - 1]
            .iter().rposition(|job| job.finish <= current.start)
            .map_or(0, |position| best[position + 1]);
        best[index] = best[index - 1].max(previous + current.reward);
    }
    let mut selected = Vec::new();
    let mut index = jobs.len();
    while index > 0 {
        let current = &jobs[index - 1];
        if best[index] == best[index - 1] { index -= 1; continue; }
        selected.push(current.clone());
        index = jobs[..index - 1]
            .iter().rposition(|job| job.finish <= current.start)
            .map_or(0, |position| position + 1);
    }
    selected.reverse();
    (best[jobs.len()], selected)
}

fn main() {
    let jobs = vec![Job { start: 1, finish: 3, reward: 5 }, Job { start: 3, finish: 6, reward: 7 }];
    println!("{:?}", maximize(jobs));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chooses_compatible_high_value_jobs() {
        let jobs = vec![Job { start: 1, finish: 3, reward: 5 }, Job { start: 2, finish: 5, reward: 8 }, Job { start: 5, finish: 7, reward: 6 }];
        assert_eq!(maximize(jobs).0, 14);
    }
}
