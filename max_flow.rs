//! Dinic's maximum-flow/minimum-cut algorithm for directed capacitated networks.
//!
//! Edges are added with nonnegative `u64` capacities. Every query builds its own
//! residual capacities from the original edges, so calling `max_flow` or
//! `minimum_cut` repeatedly does not mutate the network.

use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Edge {
    from: usize,
    to: usize,
    capacity: u64,
}

/// A directed network whose edge capacities are preserved across queries.
#[derive(Clone, Debug)]
pub struct FlowNetwork {
    vertex_count: usize,
    edges: Vec<Edge>,
    adjacency: Vec<Vec<usize>>,
}

/// A minimum source-side cut and its total capacity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinimumCut {
    /// Vertices reachable from the source in the final residual network.
    pub source_side: Vec<bool>,
    /// Sum of original capacities crossing from `source_side` to its complement.
    pub capacity: u64,
}

impl FlowNetwork {
    /// Creates a network with vertices numbered `0..vertex_count`.
    pub fn new(vertex_count: usize) -> Self {
        Self {
            vertex_count,
            edges: Vec::new(),
            adjacency: vec![Vec::new(); vertex_count],
        }
    }

    /// Adds a directed edge. Parallel edges and opposite-direction edges are allowed.
    pub fn add_edge(&mut self, from: usize, to: usize, capacity: u64) -> Result<(), String> {
        if from >= self.vertex_count || to >= self.vertex_count {
            return Err(format!(
                "edge endpoints ({from}, {to}) must be below vertex count {}",
                self.vertex_count
            ));
        }
        let edge_index = self.edges.len();
        self.edges.push(Edge { from, to, capacity });
        self.edges.push(Edge { from: to, to: from, capacity: 0 });
        self.adjacency[from].push(edge_index);
        self.adjacency[to].push(edge_index + 1);
        Ok(())
    }

    /// Returns the maximum flow from `source` to `sink` without changing this network.
    pub fn max_flow(&self, source: usize, sink: usize) -> Result<u64, String> {
        self.validate_terminals(source, sink)?;
        if source == sink {
            return Ok(0);
        }

        let mut residual: Vec<u64> = self.edges.iter().map(|edge| edge.capacity).collect();
        let mut total_flow = 0u64;
        loop {
            let levels = self.build_levels(source, sink, &residual);
            if levels[sink] == usize::MAX {
                return Ok(total_flow);
            }
            let mut next_edge = vec![0usize; self.vertex_count];
            loop {
                let pushed = self.send_flow(
                    source,
                    sink,
                    u64::MAX,
                    &levels,
                    &mut next_edge,
                    &mut residual,
                );
                if pushed == 0 {
                    break;
                }
                total_flow = total_flow
                    .checked_add(pushed)
                    .ok_or_else(|| "maximum flow exceeds u64 capacity".to_string())?;
            }
        }
    }

    /// Returns a minimum cut, found from the residual graph after maximum flow.
    pub fn minimum_cut(&self, source: usize, sink: usize) -> Result<MinimumCut, String> {
        self.validate_terminals(source, sink)?;
        if source == sink {
            return Err("source and sink must be different to define a minimum cut".to_string());
        }
        let mut residual: Vec<u64> = self.edges.iter().map(|edge| edge.capacity).collect();
        let mut total_flow = 0u64;
        loop {
            let levels = self.build_levels(source, sink, &residual);
            if levels[sink] == usize::MAX {
                break;
            }
            let mut next_edge = vec![0usize; self.vertex_count];
            loop {
                let pushed = self.send_flow(
                    source,
                    sink,
                    u64::MAX,
                    &levels,
                    &mut next_edge,
                    &mut residual,
                );
                if pushed == 0 {
                    break;
                }
                total_flow = total_flow
                    .checked_add(pushed)
                    .ok_or_else(|| "maximum flow exceeds u64 capacity".to_string())?;
            }
        }

        let source_side = self.residual_reachable(source, &residual);
        let capacity = self
            .edges
            .iter()
            .step_by(2)
            .filter(|edge| source_side[edge.from] && !source_side[edge.to])
            .try_fold(0u64, |sum, edge| sum.checked_add(edge.capacity))
            .ok_or_else(|| "minimum-cut capacity exceeds u64 capacity".to_string())?;
        if capacity != total_flow {
            return Err("internal error: maximum-flow/minimum-cut invariant failed".to_string());
        }
        Ok(MinimumCut { source_side, capacity })
    }

    fn validate_terminals(&self, source: usize, sink: usize) -> Result<(), String> {
        if source >= self.vertex_count || sink >= self.vertex_count {
            return Err(format!(
                "source and sink must be below vertex count {}",
                self.vertex_count
            ));
        }
        Ok(())
    }

    fn build_levels(&self, source: usize, sink: usize, residual: &[u64]) -> Vec<usize> {
        let mut levels = vec![usize::MAX; self.vertex_count];
        let mut queue = VecDeque::new();
        levels[source] = 0;
        queue.push_back(source);
        while let Some(vertex) = queue.pop_front() {
            for &edge_index in &self.adjacency[vertex] {
                let edge = &self.edges[edge_index];
                if residual[edge_index] > 0 && levels[edge.to] == usize::MAX {
                    levels[edge.to] = levels[vertex] + 1;
                    if edge.to != sink {
                        queue.push_back(edge.to);
                    }
                }
            }
        }
        levels
    }

    fn send_flow(
        &self,
        vertex: usize,
        sink: usize,
        limit: u64,
        levels: &[usize],
        next_edge: &mut [usize],
        residual: &mut [u64],
    ) -> u64 {
        if vertex == sink {
            return limit;
        }
        while next_edge[vertex] < self.adjacency[vertex].len() {
            let edge_index = self.adjacency[vertex][next_edge[vertex]];
            let to = self.edges[edge_index].to;
            if residual[edge_index] > 0 && levels[to] == levels[vertex] + 1 {
                let pushed = self.send_flow(
                    to,
                    sink,
                    limit.min(residual[edge_index]),
                    levels,
                    next_edge,
                    residual,
                );
                if pushed > 0 {
                    residual[edge_index] -= pushed;
                    residual[edge_index ^ 1] += pushed;
                    return pushed;
                }
            }
            next_edge[vertex] += 1;
        }
        0
    }

    fn residual_reachable(&self, source: usize, residual: &[u64]) -> Vec<bool> {
        let mut reached = vec![false; self.vertex_count];
        let mut queue = VecDeque::new();
        reached[source] = true;
        queue.push_back(source);
        while let Some(vertex) = queue.pop_front() {
            for &edge_index in &self.adjacency[vertex] {
                let edge = &self.edges[edge_index];
                if residual[edge_index] > 0 && !reached[edge.to] {
                    reached[edge.to] = true;
                    queue.push_back(edge.to);
                }
            }
        }
        reached
    }
}

#[cfg(test)]
mod tests {
    use super::FlowNetwork;

    #[test]
    fn zero_flow_and_zero_capacity_edges() {
        let mut network = FlowNetwork::new(3);
        network.add_edge(0, 1, 0).unwrap();
        network.add_edge(1, 2, 7).unwrap();
        assert_eq!(network.max_flow(0, 2).unwrap(), 0);
        assert_eq!(network.minimum_cut(0, 2).unwrap().capacity, 0);
    }

    #[test]
    fn disconnected_terminals_have_zero_flow() {
        let mut network = FlowNetwork::new(4);
        network.add_edge(0, 1, 9).unwrap();
        network.add_edge(2, 3, 4).unwrap();
        assert_eq!(network.max_flow(0, 3).unwrap(), 0);
        assert_eq!(network.minimum_cut(0, 3).unwrap().capacity, 0);
    }

    #[test]
    fn sums_parallel_edges_and_handles_opposite_edges() {
        let mut network = FlowNetwork::new(2);
        network.add_edge(0, 1, 2).unwrap();
        network.add_edge(0, 1, 3).unwrap();
        network.add_edge(1, 0, 11).unwrap();
        assert_eq!(network.max_flow(0, 1).unwrap(), 5);
        assert_eq!(network.minimum_cut(0, 1).unwrap().capacity, 5);
    }

    #[test]
    fn bottleneck_limits_two_routes() {
        let mut network = FlowNetwork::new(4);
        network.add_edge(0, 1, 8).unwrap();
        network.add_edge(0, 2, 7).unwrap();
        network.add_edge(1, 3, 4).unwrap();
        network.add_edge(2, 3, 6).unwrap();
        assert_eq!(network.max_flow(0, 3).unwrap(), 10);
    }

    #[test]
    fn known_minimum_cut_matches_maximum_flow() {
        let mut network = FlowNetwork::new(6);
        for (from, to, capacity) in [
            (0, 1, 16), (0, 2, 13), (1, 2, 10), (2, 1, 4),
            (1, 3, 12), (3, 2, 9), (2, 4, 14), (4, 3, 7),
            (3, 5, 20), (4, 5, 4),
        ] {
            network.add_edge(from, to, capacity).unwrap();
        }
        let flow = network.max_flow(0, 5).unwrap();
        let cut = network.minimum_cut(0, 5).unwrap();
        assert_eq!(flow, 23);
        assert_eq!(cut.capacity, flow);
        assert!(cut.source_side[0]);
        assert!(!cut.source_side[5]);
    }

    #[test]
    fn repeated_queries_leave_network_unchanged() {
        let mut network = FlowNetwork::new(3);
        network.add_edge(0, 1, 5).unwrap();
        network.add_edge(1, 2, 3).unwrap();
        assert_eq!(network.max_flow(0, 2).unwrap(), 3);
        assert_eq!(network.max_flow(0, 2).unwrap(), 3);
        assert_eq!(network.minimum_cut(0, 2).unwrap().capacity, 3);
    }

    #[test]
    fn validates_edges_and_terminals_and_same_vertex_case() {
        let mut network = FlowNetwork::new(2);
        assert!(network.add_edge(0, 2, 1).is_err());
        assert!(network.max_flow(0, 2).is_err());
        assert_eq!(network.max_flow(1, 1).unwrap(), 0);
        assert!(network.minimum_cut(1, 1).is_err());
        assert!(FlowNetwork::new(0).max_flow(0, 0).is_err());
    }

    #[test]
    fn reports_total_flow_overflow() {
        let mut network = FlowNetwork::new(4);
        network.add_edge(0, 1, u64::MAX).unwrap();
        network.add_edge(1, 3, u64::MAX).unwrap();
        network.add_edge(0, 2, u64::MAX).unwrap();
        network.add_edge(2, 3, u64::MAX).unwrap();
        assert!(network.max_flow(0, 3).unwrap_err().contains("exceeds u64"));
        assert!(network.minimum_cut(0, 3).unwrap_err().contains("exceeds u64"));
    }
}

fn main() {
    let mut network = FlowNetwork::new(4);
    for (from, to, capacity) in [(0, 1, 3), (0, 2, 2), (1, 2, 1), (1, 3, 2), (2, 3, 4)] {
        network.add_edge(from, to, capacity).expect("valid example edge");
    }
    let flow = network.max_flow(0, 3).expect("valid example terminals");
    let cut = network.minimum_cut(0, 3).expect("valid example terminals");
    println!("Maximum flow: {flow}; minimum-cut capacity: {}", cut.capacity);
}
