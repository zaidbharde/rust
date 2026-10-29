use std::collections::VecDeque;

/// Returns the values of an array-indexed binary tree, grouped by breadth-first level.
///
/// `nodes[i]` has children at `2 * i + 1` and `2 * i + 2`. `None` marks an
/// absent node; its descendants are unreachable and are not included.
pub fn binary_tree_level_order<T>(nodes: &[Option<T>]) -> Vec<Vec<&T>> {
    let Some(Some(_)) = nodes.first() else {
        return Vec::new();
    };

    let mut levels = Vec::new();
    let mut queue = VecDeque::from([(0usize, 0usize)]);
    while let Some((index, depth)) = queue.pop_front() {
        let Some(Some(value)) = nodes.get(index) else {
            continue;
        };
        if levels.len() == depth {
            levels.push(Vec::new());
        }
        levels[depth].push(value);

        if let Some(left) = index.checked_mul(2).and_then(|i| i.checked_add(1)) {
            if nodes.get(left).is_some_and(Option::is_some) {
                queue.push_back((left, depth + 1));
            }
            if let Some(right) = left.checked_add(1) {
                if nodes.get(right).is_some_and(Option::is_some) {
                    queue.push_back((right, depth + 1));
                }
            }
        }
    }
    levels
}

#[cfg(test)]
mod tests {
    use super::binary_tree_level_order;

    #[test]
    fn groups_complete_tree_by_level() {
        let tree = [Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7)];
        assert_eq!(binary_tree_level_order(&tree), vec![vec![&1], vec![&2, &3], vec![&4, &6, &7]]);
    }

    #[test]
    fn handles_empty_root_and_unreachable_nodes() {
        let empty: [Option<i32>; 0] = [];
        assert!(binary_tree_level_order(&empty).is_empty());
        assert!(binary_tree_level_order(&[None, Some(9)]).is_empty());
        assert_eq!(binary_tree_level_order(&[Some(5), None, Some(8)]), vec![vec![&5], vec![&8]]);
    }

    #[test]
    fn handles_single_node() {
        assert_eq!(binary_tree_level_order(&[Some("root")]), vec![vec![&"root"]]);
    }
}
