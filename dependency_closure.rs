//! Compute a deterministic transitive dependency closure from adjacency lists.

use std::collections::{BTreeSet, HashMap};

pub fn closure(graph: &HashMap<String, Vec<String>>, roots: &[String]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut stack: Vec<String> = roots.iter().cloned().rev().collect();
    while let Some(node) = stack.pop() {
        if !seen.insert(node.clone()) { continue; }
        if let Some(children) = graph.get(&node) {
            for child in children.iter().rev() { stack.push(child.clone()); }
        }
    }
    seen.into_iter().collect()
}

pub fn missing_dependencies(
    graph: &HashMap<String, Vec<String>>,
    roots: &[String],
    available: &BTreeSet<String>,
) -> Vec<String> {
    closure(graph, roots).into_iter().filter(|item| !available.contains(item)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handles_cycles_and_sorts_results() {
        let mut graph = HashMap::new();
        graph.insert("app".into(), vec!["core".into(), "ui".into()]);
        graph.insert("core".into(), vec!["app".into(), "crypto".into()]);
        let result = closure(&graph, &["app".into()]);
        assert_eq!(result, vec!["app", "core", "crypto", "ui"]);
    }

    #[test]
    fn reports_only_unavailable_nodes() {
        let graph = HashMap::from([(String::from("a"), vec![String::from("b")])]);
        let available = BTreeSet::from([String::from("a")]);
        assert_eq!(missing_dependencies(&graph, &["a".into()], &available), vec!["b"]);
    }
}
