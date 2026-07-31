fn greedy_coloring(graph: &Vec<Vec<usize>>) -> Vec<i32> {
    let n = graph.len();
    let mut colors = vec![-1; n];
    colors[0] = 0;

    for u in 1..n {
        let mut used = vec![false; n];
        for &v in &graph[u] {
            if colors[v] != -1 {
                used[colors[v] as usize] = true;
            }
        }
        for c in 0..n {
            if !used[c] {
                colors[u] = c as i32;
                break;
            }
        }
    }
    colors
}

fn main() {
    let graph = vec![
        vec![1, 2],
        vec![0, 2, 3],
        vec![0, 1, 3],
        vec![1, 2],
    ];
    let colors = greedy_coloring(&graph);
    for (i, c) in colors.iter().enumerate() {
        println!("Node {} -> Color {}", i, c);
    }
}
