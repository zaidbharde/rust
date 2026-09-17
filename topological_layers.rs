use std::collections::VecDeque;

/// Groups a directed acyclic graph into dependency layers.
pub fn layers(node_count: usize, edges: &[(usize, usize)]) -> Option<Vec<Vec<usize>>> {
    let mut graph = vec![Vec::new(); node_count];
    let mut indegree = vec![0usize; node_count];
    for &(from, to) in edges {
        if from >= node_count || to >= node_count { return None; }
        graph[from].push(to);
        indegree[to] += 1;
    }
    let mut ready = VecDeque::new();
    for (node, &degree) in indegree.iter().enumerate() {
        if degree == 0 { ready.push_back(node); }
    }
    let mut result = Vec::new();
    let mut visited = 0;
    while !ready.is_empty() {
        let width = ready.len();
        let mut layer = Vec::with_capacity(width);
        for _ in 0..width {
            let node = ready.pop_front().unwrap();
            visited += 1;
            layer.push(node);
            for &next in &graph[node] {
                indegree[next] -= 1;
                if indegree[next] == 0 { ready.push_back(next); }
            }
        }
        result.push(layer);
    }
    (visited == node_count).then_some(result)
}

fn main() {
    let plan = layers(5, &[(0, 2), (1, 2), (2, 3), (2, 4)]).unwrap();
    println!("{plan:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_dependency_layers() {
        assert_eq!(layers(4, &[(0, 2), (1, 2), (2, 3)]), Some(vec![vec![0, 1], vec![2], vec![3]]));
    }
    #[test]
    fn detects_cycles() { assert_eq!(layers(2, &[(0, 1), (1, 0)]), None); }
}
