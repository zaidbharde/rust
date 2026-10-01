use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub fn topological_order<T>(edges: &[(T, T)]) -> Result<Vec<T>, &'static str>
where T: Eq + Hash + Clone {
    let mut graph: HashMap<T, Vec<T>> = HashMap::new();
    let mut indegree: HashMap<T, usize> = HashMap::new();
    for (from, to) in edges {
        graph.entry(from.clone()).or_default().push(to.clone());
        indegree.entry(from.clone()).or_insert(0);
        *indegree.entry(to.clone()).or_insert(0) += 1;
    }
    let mut ready: VecDeque<T> = indegree.iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(node, _)| node.clone()).collect();
    let mut order = Vec::with_capacity(indegree.len());
    while let Some(node) = ready.pop_front() {
        order.push(node.clone());
        for next in graph.get(&node).into_iter().flatten() {
            let degree = indegree.get_mut(next).unwrap();
            *degree -= 1;
            if *degree == 0 { ready.push_back(next.clone()); }
        }
    }
    if order.len() == indegree.len() { Ok(order) } else { Err("graph contains a cycle") }
}

#[cfg(test)]
mod tests {
    use super::topological_order;
    #[test]
    fn detects_cycles() {
        assert_eq!(topological_order(&[(1, 2), (2, 1)]), Err("graph contains a cycle"));
    }
}
