use std::collections::{HashMap, VecDeque};

/// Groups a dependency graph into deterministic layers by node identifier.
pub fn dependency_layers(nodes: &[usize], edges: &[(usize, usize)]) -> Result<Vec<Vec<usize>>, &'static str> {
    let mut indegree = nodes.iter().map(|&node| (node, 0usize)).collect::<HashMap<_, _>>();
    let mut outgoing: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(from, to) in edges {
        if !indegree.contains_key(&from) || !indegree.contains_key(&to) { return Err("unknown node"); }
        outgoing.entry(from).or_default().push(to);
        *indegree.get_mut(&to).unwrap() += 1;
    }
    let mut ready = nodes.iter().copied().filter(|node| indegree[node] == 0).collect::<VecDeque<_>>();
    let mut layers = Vec::new();
    let mut visited = 0;
    while !ready.is_empty() {
        let width = ready.len();
        let mut layer = Vec::with_capacity(width);
        for _ in 0..width {
            let node = ready.pop_front().unwrap();
            visited += 1;
            layer.push(node);
            for next in outgoing.get(&node).cloned().unwrap_or_default() {
                let degree = indegree.get_mut(&next).unwrap();
                *degree -= 1;
                if *degree == 0 { ready.push_back(next); }
            }
        }
        layers.push(layer);
    }
    if visited == nodes.len() { Ok(layers) } else { Err("graph contains a cycle") }
}
