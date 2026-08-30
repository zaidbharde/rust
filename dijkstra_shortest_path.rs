use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn shortest_path(graph: &[Vec<(usize, u32)>], start: usize) -> Vec<u32> {
    let mut distance = vec![u32::MAX; graph.len()];
    let mut heap = BinaryHeap::new();
    distance[start] = 0;
    heap.push(Reverse((0, start)));
    while let Some(Reverse((cost, node))) = heap.pop() {
        if cost > distance[node] { continue; }
        for &(next, weight) in &graph[node] {
            let candidate = cost + weight;
            if candidate < distance[next] {
                distance[next] = candidate;
                heap.push(Reverse((candidate, next)));
            }
        }
    }
    distance
}

fn main() {
    let graph = vec![vec![(1, 4), (2, 1)], vec![(3, 1)], vec![(1, 2), (3, 5)], vec![]];
    println!("{:?}", shortest_path(&graph, 0));
}
