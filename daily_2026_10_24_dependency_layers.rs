//! Group dependency nodes into executable layers.

use std::collections::{HashMap, HashSet, VecDeque};

pub fn layers(nodes: &[&str], edges: &[(&str, &str)]) -> Result<Vec<Vec<String>>, &'static str> {
    let mut incoming: HashMap<&str, usize> = nodes.iter().map(|&node| (node, 0)).collect();
    let mut outgoing: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(from, to) in edges {
        incoming.entry(from).or_insert(0);
        let count = incoming.entry(to).or_insert(0);
        *count += 1;
        outgoing.entry(from).or_default().push(to);
    }
    let mut ready: VecDeque<&str> = incoming.iter().filter(|(_, degree)| **degree == 0)
        .map(|(&node, _)| node).collect();
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    while !ready.is_empty() {
        let mut current = Vec::new();
        for _ in 0..ready.len() {
            let node = ready.pop_front().unwrap();
            current.push(node.to_string());
            seen.insert(node);
        }
        current.sort();
        for node in &current {
            for &child in outgoing.get(node.as_str()).unwrap_or(&Vec::new()) {
                let degree = incoming.get_mut(child).unwrap();
                *degree -= 1;
                if *degree == 0 { ready.push_back(child); }
            }
        }
        result.push(current);
    }
    if seen.len() != incoming.len() { return Err("dependency graph contains a cycle"); }
    Ok(result)
}

fn main() {
    let graph = [("fetch", "parse"), ("parse", "store"), ("fetch", "audit")];
    assert_eq!(layers(&["store", "parse", "fetch", "audit"], &graph).unwrap(),
        vec![vec!["fetch"], vec!["audit", "parse"], vec!["store"]]);
    println!("dependency layers computed");
}
