use std::collections::{HashMap, VecDeque};

/// Returns a stable ordering, or None when the directed graph contains a cycle.
pub fn stable_order(nodes: &[&str], edges: &[(&str, &str)]) -> Option<Vec<String>> {
    let mut indegree: HashMap<&str, usize> = nodes.iter().map(|&node| (node, 0)).collect();
    let mut outgoing: HashMap<&str, Vec<&str>> = nodes.iter().map(|&node| (node, Vec::new())).collect();
    for &(from, to) in edges {
        if !indegree.contains_key(from) || !indegree.contains_key(to) { return None; }
        outgoing.get_mut(from)?.push(to);
        *indegree.get_mut(to)? += 1;
    }
    let mut ready: VecDeque<&str> = nodes.iter().copied().filter(|node| indegree[node] == 0).collect();
    let mut result = Vec::with_capacity(nodes.len());
    while let Some(node) = ready.pop_front() {
        result.push(node.to_string());
        for next in &outgoing[node] {
            let degree = indegree.get_mut(next)?;
            *degree -= 1;
            if *degree == 0 { ready.push_back(next); }
        }
    }
    (result.len() == nodes.len()).then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_dependencies_before_dependents() {
        let order = stable_order(&["app", "db", "api"], &[("db", "api"), ("api", "app")]).unwrap();
        assert_eq!(order, vec!["db", "api", "app"]);
    }
    #[test]
    fn rejects_cycles() {
        assert!(stable_order(&["a", "b"], &[("a", "b"), ("b", "a")]).is_none());
    }
}
