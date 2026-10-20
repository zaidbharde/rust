//! Kahn-style topological ordering with explicit cycle detection.

use std::collections::{BTreeSet, HashMap};

pub fn order(nodes: &[&str], edges: &[(&str, &str)]) -> Result<Vec<String>, Vec<String>> {
    let mut outgoing: HashMap<&str, Vec<&str>> = nodes.iter().map(|node| (*node, Vec::new())).collect();
    let mut indegree: HashMap<&str, usize> = nodes.iter().map(|node| (*node, 0)).collect();
    for &(from, to) in edges {
        if !outgoing.contains_key(from) || !indegree.contains_key(to) { continue; }
        outgoing.get_mut(from).unwrap().push(to);
        *indegree.get_mut(to).unwrap() += 1;
    }
    let mut ready: BTreeSet<&str> = indegree.iter().filter_map(|(node, degree)| (*degree == 0).then_some(*node)).collect();
    let mut result = Vec::with_capacity(nodes.len());
    while let Some(node) = ready.pop_first() {
        result.push(node.to_owned());
        for next in outgoing[node].iter().copied() {
            let degree = indegree.get_mut(next).unwrap();
            *degree -= 1;
            if *degree == 0 { ready.insert(next); }
        }
    }
    if result.len() == nodes.len() { Ok(result) } else {
        Err(indegree.into_iter().filter_map(|(node, degree)| (degree > 0).then_some(node.to_owned())).collect())
    }
}
