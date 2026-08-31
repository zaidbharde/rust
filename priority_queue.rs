use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Task {
    priority: u32,
    sequence: usize,
    name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
            .then_with(|| other.sequence.cmp(&self.sequence))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

fn schedule(tasks: Vec<(u32, &str)>) -> Vec<String> {
    let mut queue = BinaryHeap::new();
    for (sequence, (priority, name)) in tasks.into_iter().enumerate() {
        queue.push(Task { priority, sequence, name: name.to_string() });
    }
    std::iter::from_fn(|| queue.pop().map(|task| task.name)).collect()
}

fn main() {
    let tasks = vec![(2, "index"), (5, "backup"), (5, "deploy"), (1, "cleanup")];
    println!("{:?}", schedule(tasks));
}
