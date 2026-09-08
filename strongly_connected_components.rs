//! Find strongly connected components in a directed graph.

pub fn components(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut visited = vec![false; graph.len()];
    let mut order = Vec::with_capacity(graph.len());
    for start in 0..graph.len() {
        if !visited[start] { finish_order(graph, start, &mut visited, &mut order); }
    }
    let mut reverse = vec![Vec::new(); graph.len()];
    for (from, edges) in graph.iter().enumerate() {
        for &to in edges { if to < graph.len() { reverse[to].push(from); } }
    }
    visited.fill(false);
    let mut result = Vec::new();
    while let Some(start) = order.pop() {
        if visited[start] { continue; }
        let mut stack = vec![start];
        let mut group = Vec::new();
        visited[start] = true;
        while let Some(node) = stack.pop() {
            group.push(node);
            for &next in &reverse[node] {
                if !visited[next] { visited[next] = true; stack.push(next); }
            }
        }
        group.sort_unstable();
        result.push(group);
    }
    result.sort_by_key(|group| group[0]);
    result
}

fn finish_order(graph: &[Vec<usize>], start: usize, visited: &mut [bool], order: &mut Vec<usize>) {
    let mut stack = vec![(start, false)];
    while let Some((node, expanded)) = stack.pop() {
        if expanded { order.push(node); continue; }
        if visited[node] { continue; }
        visited[node] = true;
        stack.push((node, true));
        for &next in graph[node].iter().rev() {
            if next < graph.len() && !visited[next] { stack.push((next, false)); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::components;
    #[test]
    fn groups_mutually_reachable_nodes() {
        let graph = vec![vec![1], vec![0, 2], vec![3], vec![2]];
        assert_eq!(components(&graph), vec![vec![0, 1], vec![2, 3]]);
    }
}
