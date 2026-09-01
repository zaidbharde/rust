#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    pub name: String,
    pub start: u32,
    pub end: u32,
}

impl Job {
    pub fn new(name: impl Into<String>, start: u32, end: u32) -> Self {
        assert!(end >= start);
        Self { name: name.into(), start, end }
    }
}

/// Select compatible jobs using the classic earliest-finish-time strategy.
pub fn schedule(mut jobs: Vec<Job>) -> Vec<Job> {
    jobs.sort_by_key(|job| (job.end, job.start));
    let mut chosen = Vec::new();
    let mut available_at = 0;
    for job in jobs {
        if job.start >= available_at {
            available_at = job.end;
            chosen.push(job);
        }
    }
    chosen
}

fn main() {
    let jobs = vec![
        Job::new("design", 1, 4), Job::new("build", 4, 7),
        Job::new("review", 2, 5), Job::new("deploy", 7, 8),
    ];
    for job in schedule(jobs) { println!("{} {}-{}", job.name, job.start, job.end); }
}
