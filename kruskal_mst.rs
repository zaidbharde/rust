//! Kruskal's minimum spanning forest for weighted undirected graphs.
//!
//! Vertices are numbered `0..vertex_count`. Edges may have negative weights;
//! invalid endpoints and a total-weight overflow are reported as errors.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub weight: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinimumSpanningForest {
    pub edges: Vec<Edge>,
    pub total_weight: i64,
    pub component_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KruskalError {
    InvalidEndpoint { edge_index: usize, vertex: usize, vertex_count: usize },
    TotalWeightOverflow,
}

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self { parent: (0..size).collect(), rank: vec![0; size] }
    }

    fn find(&mut self, vertex: usize) -> usize {
        if self.parent[vertex] != vertex {
            self.parent[vertex] = self.find(self.parent[vertex]);
        }
        self.parent[vertex]
    }

    fn union(&mut self, left: usize, right: usize) -> bool {
        let mut a = self.find(left);
        let mut b = self.find(right);
        if a == b { return false; }
        if self.rank[a] < self.rank[b] { std::mem::swap(&mut a, &mut b); }
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] { self.rank[a] += 1; }
        true
    }
}

/// Return a minimum spanning forest. Equal weights are ordered by endpoints,
/// then original input position, so repeated runs produce the same edge list.
pub fn kruskal_minimum_spanning_forest(
    vertex_count: usize,
    edges: &[Edge],
) -> Result<MinimumSpanningForest, KruskalError> {
    for (edge_index, edge) in edges.iter().enumerate() {
        for vertex in [edge.u, edge.v] {
            if vertex >= vertex_count {
                return Err(KruskalError::InvalidEndpoint { edge_index, vertex, vertex_count });
            }
        }
    }

    let mut order: Vec<usize> = (0..edges.len()).collect();
    order.sort_by_key(|&i| (edges[i].weight, edges[i].u.min(edges[i].v), edges[i].u.max(edges[i].v), i));
    let mut sets = DisjointSet::new(vertex_count);
    let mut chosen = Vec::with_capacity(vertex_count.saturating_sub(1));
    let mut total_weight = 0i64;
    let mut component_count = vertex_count;

    for i in order {
        let edge = edges[i];
        if sets.union(edge.u, edge.v) {
            total_weight = total_weight.checked_add(edge.weight).ok_or(KruskalError::TotalWeightOverflow)?;
            chosen.push(edge);
            component_count -= 1;
        }
    }
    Ok(MinimumSpanningForest { edges: chosen, total_weight, component_count })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(u: usize, v: usize, weight: i64) -> Edge { Edge { u, v, weight } }

    #[test]
    fn known_graph_and_forest_edge_count() {
        let graph = [e(0, 1, 4), e(0, 2, 3), e(1, 2, 1), e(1, 3, 2), e(2, 3, 5)];
        let forest = kruskal_minimum_spanning_forest(4, &graph).unwrap();
        assert_eq!(forest.total_weight, 6);
        assert_eq!(forest.edges, vec![e(1, 2, 1), e(1, 3, 2), e(0, 2, 3)]);
        assert_eq!(forest.edges.len(), 3);
        assert_eq!(forest.component_count, 1);
    }

    #[test]
    fn negative_weights_are_supported() {
        let forest = kruskal_minimum_spanning_forest(3, &[e(0, 1, -5), e(1, 2, -2), e(0, 2, 1)]).unwrap();
        assert_eq!(forest.total_weight, -7);
        assert_eq!(forest.edges.len(), 2);
    }

    #[test]
    fn disconnected_graph_returns_forest() {
        let forest = kruskal_minimum_spanning_forest(5, &[e(0, 1, 7), e(1, 2, 4), e(3, 4, 1)]).unwrap();
        assert_eq!(forest.total_weight, 12);
        assert_eq!(forest.edges.len(), 5 - forest.component_count);
        assert_eq!(forest.component_count, 2);
    }

    #[test]
    fn self_loops_are_ignored_and_ties_are_deterministic() {
        let input = [e(0, 0, -100), e(2, 1, 1), e(0, 2, 1), e(0, 1, 1)];
        let first = kruskal_minimum_spanning_forest(3, &input).unwrap();
        let second = kruskal_minimum_spanning_forest(3, &input).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.edges, vec![e(0, 1, 1), e(0, 2, 1)]);
        assert_eq!(first.total_weight, 2);
    }

    #[test]
    fn empty_singleton_and_isolated_vertices() {
        assert_eq!(kruskal_minimum_spanning_forest(0, &[]).unwrap(), MinimumSpanningForest { edges: vec![], total_weight: 0, component_count: 0 });
        assert_eq!(kruskal_minimum_spanning_forest(1, &[]).unwrap(), MinimumSpanningForest { edges: vec![], total_weight: 0, component_count: 1 });
        assert_eq!(kruskal_minimum_spanning_forest(4, &[]).unwrap().component_count, 4);
    }

    #[test]
    fn invalid_endpoint_is_reported() {
        assert_eq!(kruskal_minimum_spanning_forest(2, &[e(0, 2, 1)]), Err(KruskalError::InvalidEndpoint { edge_index: 0, vertex: 2, vertex_count: 2 }));
    }

    #[test]
    fn total_weight_overflow_is_reported() {
        assert_eq!(kruskal_minimum_spanning_forest(3, &[e(0, 1, i64::MAX), e(1, 2, 1)]), Err(KruskalError::TotalWeightOverflow));
    }

    // Independent tiny-graph oracle: enumerate every acyclic subset with the
    // required forest rank (vertices minus input graph's connected components).
    fn exhaustive_weight(vertex_count: usize, edges: &[Edge]) -> (i64, usize) {
        let mut graph_sets = DisjointSet::new(vertex_count);
        for edge in edges { graph_sets.union(edge.u, edge.v); }
        let graph_components = (0..vertex_count).map(|v| graph_sets.find(v)).collect::<std::collections::HashSet<_>>().len();
        let needed = vertex_count - graph_components;
        let mut best = i64::MAX;
        for mask in 0usize..(1usize << edges.len()) {
            if mask.count_ones() as usize != needed { continue; }
            let mut sets = DisjointSet::new(vertex_count);
            let mut sum = 0i64;
            let mut acyclic = true;
            for (i, edge) in edges.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    if !sets.union(edge.u, edge.v) { acyclic = false; break; }
                    sum += edge.weight;
                }
            }
            if acyclic { best = best.min(sum); }
        }
        (best, graph_components)
    }

    #[test]
    fn matches_exhaustive_spanning_forest_enumeration() {
        let samples = [
            (4, vec![e(0, 1, 3), e(1, 2, -1), e(2, 3, 2), e(0, 3, 8), e(0, 2, 4)]),
            (5, vec![e(0, 1, 5), e(1, 2, 2), e(0, 2, 7), e(3, 4, -3)]),
            (3, vec![e(0, 0, -9), e(0, 1, 2), e(1, 2, 2), e(0, 2, 2)]),
        ];
        for (n, graph) in samples {
            let result = kruskal_minimum_spanning_forest(n, &graph).unwrap();
            let (expected_weight, expected_components) = exhaustive_weight(n, &graph);
            assert_eq!(result.total_weight, expected_weight);
            assert_eq!(result.component_count, expected_components);
            assert_eq!(result.edges.len(), n - expected_components);
        }
    }
}
