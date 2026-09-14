use std::collections::{HashMap, VecDeque};

pub fn sort(nodes: &[&str], edges: &[(&str, &str)]) -> Result<Vec<String>, Vec<String>> {
    let mut incoming = nodes.iter().map(|&node| (node.to_string(), 0usize)).collect::<HashMap<_, _>>();
    let mut outgoing: HashMap<String, Vec<String>> = HashMap::new();
    for &(from, to) in edges {
        if let Some(count) = incoming.get_mut(to) { *count += 1; }
        outgoing.entry(from.to_string()).or_default().push(to.to_string());
    }
    let mut ready = nodes.iter().filter(|&&node| incoming[node] == 0).map(|&n| n.to_string()).collect::<VecDeque<_>>();
    let mut ordered = Vec::with_capacity(nodes.len());
    while let Some(node) = ready.pop_front() {
        ordered.push(node.clone());
        for next in outgoing.get(&node).into_iter().flatten() {
            let count = incoming.get_mut(next).expect("edge target must be a node");
            *count -= 1;
            if *count == 0 { ready.push_back(next.clone()); }
        }
    }
    if ordered.len() == nodes.len() { Ok(ordered) } else {
        Err(incoming.into_iter().filter_map(|(n, count)| (count > 0).then_some(n)).collect())
    }
}

fn main() {
    let graph = [("fetch", "parse"), ("parse", "store")];
    println!("{:?}", sort(&["fetch", "parse", "store"], &graph));
}
