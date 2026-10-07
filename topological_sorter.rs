//! Kahn-style topological sorting with explicit cycle detection.

use std::collections::VecDeque;

pub fn sort(nodes: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, &'static str> {
    let mut graph = vec![Vec::new(); nodes];
    let mut indegree = vec![0usize; nodes];
    for &(from, to) in edges {
        if from >= nodes || to >= nodes { return Err("edge endpoint out of range"); }
        graph[from].push(to);
        indegree[to] += 1;
    }

    let mut ready = (0..nodes).filter(|&node| indegree[node] == 0).collect::<VecDeque<_>>();
    let mut ordering = Vec::with_capacity(nodes);
    while let Some(node) = ready.pop_front() {
        ordering.push(node);
        for &neighbor in &graph[node] {
            indegree[neighbor] -= 1;
            if indegree[neighbor] == 0 { ready.push_back(neighbor); }
        }
    }

    if ordering.len() == nodes { Ok(ordering) } else { Err("graph contains a cycle") }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn detects_cycles() {
        assert_eq!(sort(2, &[(0, 1), (1, 0)]), Err("graph contains a cycle"));
    }
}
