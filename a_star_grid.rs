use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathError {
    EmptyGrid,
    RaggedGrid,
    OutOfBounds(Point),
    BlockedEndpoint(Point),
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGrid => write!(f, "grid must contain at least one row and column"),
            Self::RaggedGrid => write!(f, "grid rows must all have the same length"),
            Self::OutOfBounds(p) => write!(f, "point ({}, {}) is outside the grid", p.row, p.col),
            Self::BlockedEndpoint(p) => write!(f, "endpoint ({}, {}) is blocked", p.row, p.col),
        }
    }
}

/// Find a shortest four-directional route. `true` cells are blocked.
/// Returns `Ok(None)` when valid endpoints cannot be connected.
pub fn a_star(
    blocked: &[Vec<bool>],
    start: Point,
    goal: Point,
) -> Result<Option<Vec<Point>>, PathError> {
    let rows = blocked.len();
    if rows == 0 || blocked[0].is_empty() {
        return Err(PathError::EmptyGrid);
    }
    let cols = blocked[0].len();
    if blocked.iter().any(|row| row.len() != cols) {
        return Err(PathError::RaggedGrid);
    }
    for point in [start, goal] {
        if point.row >= rows || point.col >= cols {
            return Err(PathError::OutOfBounds(point));
        }
        if blocked[point.row][point.col] {
            return Err(PathError::BlockedEndpoint(point));
        }
    }

    let index = |p: Point| p.row * cols + p.col;
    let point_at = |i: usize| Point { row: i / cols, col: i % cols };
    let heuristic = |p: Point| p.row.abs_diff(goal.row) + p.col.abs_diff(goal.col);
    let mut costs = vec![usize::MAX; rows * cols];
    let mut parents = vec![None; rows * cols];
    let start_i = index(start);
    let goal_i = index(goal);
    costs[start_i] = 0;
    // Reverse makes the min-heap; f, then h, then cell index provide stable ties.
    let mut open = BinaryHeap::new();
    open.push(Reverse((heuristic(start), heuristic(start), start_i, 0usize)));

    while let Some(Reverse((_f, _h, current, queued_cost))) = open.pop() {
        if queued_cost != costs[current] {
            continue;
        }
        if current == goal_i {
            let mut path = vec![goal];
            let mut cursor = goal_i;
            while cursor != start_i {
                cursor = parents[cursor].expect("reached cell must have a parent");
                path.push(point_at(cursor));
            }
            path.reverse();
            return Ok(Some(path));
        }

        let p = point_at(current);
        // Fixed neighbor order also makes equal-cost route selection reproducible.
        let neighbors = [
            (p.row.checked_sub(1), Some(p.col)),
            (Some(p.row + 1), Some(p.col)),
            (Some(p.row), p.col.checked_sub(1)),
            (Some(p.row), Some(p.col + 1)),
        ];
        for (row, col) in neighbors {
            let (Some(row), Some(col)) = (row, col) else { continue };
            if row >= rows || col >= cols || blocked[row][col] {
                continue;
            }
            let next = row * cols + col;
            let next_cost = queued_cost + 1;
            if next_cost < costs[next] {
                costs[next] = next_cost;
                parents[next] = Some(current);
                let h = heuristic(Point { row, col });
                open.push(Reverse((next_cost + h, h, next, next_cost)));
            }
        }
    }
    Ok(None)
}

fn main() {
    let map = vec![
        vec![false, false, false, false, false],
        vec![false, true, true, true, false],
        vec![false, false, false, false, false],
    ];
    let route = a_star(&map, Point { row: 0, col: 0 }, Point { row: 2, col: 4 })
        .expect("valid example grid")
        .expect("example goal is reachable");
    println!("Route ({} steps): {:?}", route.len() - 1, route);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn p(row: usize, col: usize) -> Point { Point { row, col } }

    fn check_route(grid: &[Vec<bool>], route: &[Point], start: Point, goal: Point) {
        assert_eq!(route.first(), Some(&start));
        assert_eq!(route.last(), Some(&goal));
        for point in route {
            assert!(!grid[point.row][point.col], "route crosses a blocked cell");
        }
        for pair in route.windows(2) {
            assert_eq!(pair[0].row.abs_diff(pair[1].row) + pair[0].col.abs_diff(pair[1].col), 1);
        }
    }

    fn bfs_distance(grid: &[Vec<bool>], start: Point, goal: Point) -> Option<usize> {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut distance = vec![None; rows * cols];
        let mut queue = VecDeque::from([start]);
        distance[start.row * cols + start.col] = Some(0);
        while let Some(cur) = queue.pop_front() {
            let d = distance[cur.row * cols + cur.col].unwrap();
            if cur == goal { return Some(d); }
            let adjacent = [cur.row.checked_sub(1).map(|r| p(r, cur.col)),
                (cur.row + 1 < rows).then(|| p(cur.row + 1, cur.col)),
                cur.col.checked_sub(1).map(|c| p(cur.row, c)),
                (cur.col + 1 < cols).then(|| p(cur.row, cur.col + 1))];
            for next in adjacent.into_iter().flatten() {
                let i = next.row * cols + next.col;
                if !grid[next.row][next.col] && distance[i].is_none() {
                    distance[i] = Some(d + 1);
                    queue.push_back(next);
                }
            }
        }
        None
    }

    #[test]
    fn finds_shortest_legal_route_against_bfs_oracle() {
        let grid = vec![vec![false; 6], vec![false, true, true, true, false, false],
                        vec![false, false, false, true, false, false], vec![true; 6]];
        let (start, goal) = (p(0, 0), p(2, 5));
        let route = a_star(&grid, start, goal).unwrap().unwrap();
        check_route(&grid, &route, start, goal);
        assert_eq!(route.len() - 1, bfs_distance(&grid, start, goal).unwrap());
    }

    #[test]
    fn blocked_start_or_goal_is_an_error() {
        let grid = vec![vec![true, false]];
        assert_eq!(a_star(&grid, p(0, 0), p(0, 1)), Err(PathError::BlockedEndpoint(p(0, 0))));
        let goal_blocked = vec![vec![false, true]];
        assert_eq!(a_star(&goal_blocked, p(0, 0), p(0, 1)),
                   Err(PathError::BlockedEndpoint(p(0, 1))));
    }

    #[test]
    fn unreachable_goal_returns_none() {
        let grid = vec![vec![false, true, false], vec![false, true, false], vec![false, true, false]];
        assert_eq!(a_star(&grid, p(0, 0), p(0, 2)), Ok(None));
    }

    #[test]
    fn identical_endpoints_return_single_cell_route() {
        assert_eq!(a_star(&[vec![false]], p(0, 0), p(0, 0)).unwrap(), Some(vec![p(0, 0)]));
    }

    #[test]
    fn rejects_empty_and_ragged_grids() {
        assert_eq!(a_star(&[], p(0, 0), p(0, 0)), Err(PathError::EmptyGrid));
        assert_eq!(a_star(&[vec![]], p(0, 0), p(0, 0)), Err(PathError::EmptyGrid));
        assert_eq!(a_star(&[vec![false], vec![false, false]], p(0, 0), p(0, 0)),
                   Err(PathError::RaggedGrid));
    }

    #[test]
    fn rejects_out_of_bounds_endpoint() {
        assert_eq!(a_star(&[vec![false]], p(1, 0), p(0, 0)), Err(PathError::OutOfBounds(p(1, 0))));
    }

    #[test]
    fn equal_cost_route_is_deterministic() {
        let grid = vec![vec![false; 3]; 3];
        let expected = a_star(&grid, p(1, 1), p(0, 0)).unwrap().unwrap();
        for _ in 0..10 { assert_eq!(a_star(&grid, p(1, 1), p(0, 0)).unwrap().unwrap(), expected); }
        assert_eq!(expected, vec![p(1, 1), p(0, 1), p(0, 0)]);
    }

    #[test]
    fn compares_many_small_grids_with_bfs() {
        for mask in 0u32..256 {
            let mut grid = vec![vec![false; 3]; 3];
            for bit in 0..9 {
                if mask & (1 << bit) != 0 { grid[bit / 3][bit % 3] = true; }
            }
            grid[0][0] = false;
            grid[2][2] = false;
            let expected = bfs_distance(&grid, p(0, 0), p(2, 2));
            let actual = a_star(&grid, p(0, 0), p(2, 2)).unwrap();
            assert_eq!(actual.map(|route| route.len() - 1), expected, "mask {mask:09b}");
        }
    }
}
