use std::collections::{HashMap, VecDeque};

fn bfs(graph: &HashMap<u32, Vec<u32>>, start: u32) -> HashMap<u32, u32> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let dist = visited[&node];
        if let Some(neighbors) = graph.get(&node) {
            for &next in neighbors {
                if !visited.contains_key(&next) {
                    visited.insert(next, dist + 1);
                    queue.push_back(next);
                }
            }
        }
    }
    visited
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![4]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![5]);

    let distances = bfs(&graph, 1);
    let mut keys: Vec<_> = distances.keys().collect();
    keys.sort();
    for k in keys {
        println!("Node {} -> distance {}", k, distances[k]);
    }
}
