//! Find articulation points and bridges in an undirected multigraph in O(V + E).
//!
//! Vertices are numbered `0..vertex_count`. Edge endpoints are checked; parallel
//! edges and self-loops are supported. Results are sorted by vertex/edge index.

/// Connectivity-critical vertices and edges in the input edge list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectivityAnalysis {
    pub articulation_points: Vec<usize>,
    /// Indices into the supplied edge list (each parallel edge is distinct).
    pub bridges: Vec<usize>,
}

/// Analyze an undirected graph using DFS discovery times and low-link values.
///
/// Returns an error if any endpoint is outside `0..vertex_count`.
pub fn analyze_connectivity(
    vertex_count: usize,
    edges: &[(usize, usize)],
) -> Result<ConnectivityAnalysis, String> {
    let mut adjacency = vec![Vec::<(usize, usize)>::new(); vertex_count];
    for (edge_index, &(left, right)) in edges.iter().enumerate() {
        if left >= vertex_count || right >= vertex_count {
            return Err(format!("edge {edge_index} has endpoint outside 0..{vertex_count}"));
        }
        adjacency[left].push((right, edge_index));
        adjacency[right].push((left, edge_index));
    }

    let mut discovery = vec![usize::MAX; vertex_count];
    let mut low = vec![0; vertex_count];
    let mut articulation = vec![false; vertex_count];
    let mut bridges = Vec::new();
    let mut clock = 0;

    fn visit(
        vertex: usize,
        parent_edge: Option<usize>,
        adjacency: &[Vec<(usize, usize)>],
        discovery: &mut [usize],
        low: &mut [usize],
        articulation: &mut [bool],
        bridges: &mut Vec<usize>,
        clock: &mut usize,
    ) {
        discovery[vertex] = *clock;
        low[vertex] = *clock;
        *clock += 1;
        let mut child_count = 0;

        for &(neighbor, edge_index) in &adjacency[vertex] {
            if Some(edge_index) == parent_edge {
                continue;
            }
            if discovery[neighbor] == usize::MAX {
                child_count += 1;
                visit(neighbor, Some(edge_index), adjacency, discovery, low, articulation, bridges, clock);
                low[vertex] = low[vertex].min(low[neighbor]);
                if parent_edge.is_some() && low[neighbor] >= discovery[vertex] {
                    articulation[vertex] = true;
                }
                if low[neighbor] > discovery[vertex] {
                    bridges.push(edge_index);
                }
            } else {
                low[vertex] = low[vertex].min(discovery[neighbor]);
            }
        }

        if parent_edge.is_none() && child_count > 1 {
            articulation[vertex] = true;
        }
    }

    for vertex in 0..vertex_count {
        if discovery[vertex] == usize::MAX {
            visit(vertex, None, &adjacency, &mut discovery, &mut low, &mut articulation, &mut bridges, &mut clock);
        }
    }

    let articulation_points = articulation
        .iter()
        .enumerate()
        .filter_map(|(vertex, &is_critical)| is_critical.then_some(vertex))
        .collect();
    bridges.sort_unstable();
    Ok(ConnectivityAnalysis { articulation_points, bridges })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn analyze(vertices: usize, edges: &[(usize, usize)]) -> ConnectivityAnalysis {
        analyze_connectivity(vertices, edges).unwrap()
    }

    fn component_count(vertices: usize, edges: &[(usize, usize)], removed_vertex: Option<usize>, removed_edge: Option<usize>) -> usize {
        let mut adjacency = vec![Vec::new(); vertices];
        for (index, &(a, b)) in edges.iter().enumerate() {
            if Some(index) == removed_edge || Some(a) == removed_vertex || Some(b) == removed_vertex {
                continue;
            }
            adjacency[a].push(b);
            adjacency[b].push(a);
        }
        let mut seen = vec![false; vertices];
        let mut count = 0;
        for start in 0..vertices {
            if Some(start) == removed_vertex || seen[start] { continue; }
            count += 1;
            let mut stack = vec![start];
            seen[start] = true;
            while let Some(v) = stack.pop() {
                for &next in &adjacency[v] {
                    if !seen[next] { seen[next] = true; stack.push(next); }
                }
            }
        }
        count
    }

    fn brute_force(vertices: usize, edges: &[(usize, usize)]) -> ConnectivityAnalysis {
        let baseline = component_count(vertices, edges, None, None);
        let articulation_points = (0..vertices)
            .filter(|&v| component_count(vertices, edges, Some(v), None) > baseline)
            .collect();
        let bridges = (0..edges.len())
            .filter(|&e| component_count(vertices, edges, None, Some(e)) > baseline)
            .collect();
        ConnectivityAnalysis { articulation_points, bridges }
    }

    #[test]
    fn handles_disconnected_graphs_and_isolated_vertices() {
        let edges = [(0, 1), (1, 2), (3, 4)];
        assert_eq!(analyze(6, &edges), ConnectivityAnalysis { articulation_points: vec![1], bridges: vec![0, 1, 2] });
        assert_eq!(analyze(0, &[]), ConnectivityAnalysis { articulation_points: vec![], bridges: vec![] });
    }

    #[test]
    fn cycles_have_no_critical_elements() {
        assert_eq!(analyze(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]), ConnectivityAnalysis { articulation_points: vec![], bridges: vec![] });
    }

    #[test]
    fn paths_and_root_with_multiple_children() {
        assert_eq!(analyze(4, &[(0, 1), (1, 2), (2, 3)]), ConnectivityAnalysis { articulation_points: vec![1, 2], bridges: vec![0, 1, 2] });
        assert_eq!(analyze(4, &[(0, 1), (0, 2), (0, 3)]), ConnectivityAnalysis { articulation_points: vec![0], bridges: vec![0, 1, 2] });
        assert_eq!(analyze(2, &[(0, 1)]).articulation_points, vec![]);
    }

    #[test]
    fn parallel_edges_are_not_bridges() {
        let analysis = analyze(2, &[(0, 1), (0, 1)]);
        assert_eq!(analysis, ConnectivityAnalysis { articulation_points: vec![], bridges: vec![] });
    }

    #[test]
    fn self_loops_do_not_change_connectivity() {
        assert_eq!(analyze(2, &[(0, 0), (0, 1)]).bridges, vec![1]);
    }

    #[test]
    fn rejects_out_of_range_endpoints() {
        assert!(analyze_connectivity(2, &[(0, 2)]).unwrap_err().contains("edge 0"));
        assert!(analyze_connectivity(0, &[(0, 0)]).is_err());
    }

    #[test]
    fn matches_brute_force_removals_on_small_graphs() {
        for vertices in 0..=4 {
            let possible: Vec<(usize, usize)> = (0..vertices).flat_map(|a| (a + 1..vertices).map(move |b| (a, b))).collect();
            for mask in 0usize..(1usize << possible.len()) {
                let edges: Vec<_> = possible.iter().enumerate().filter_map(|(i, &edge)| ((mask >> i) & 1 == 1).then_some(edge)).collect();
                assert_eq!(analyze(vertices, &edges), brute_force(vertices, &edges), "vertices={vertices}, edges={edges:?}");
            }
        }
        let multigraphs = [vec![(0, 1), (0, 1)], vec![(0, 0), (0, 1), (1, 1)], vec![(0, 1), (1, 2), (2, 0), (1, 2)]];
        for edges in multigraphs {
            assert_eq!(analyze(3, &edges), brute_force(3, &edges), "edges={edges:?}");
        }
    }
}
