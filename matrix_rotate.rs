/// Rotates a square matrix clockwise without allocating another matrix.
pub fn rotate_clockwise(matrix: &mut [Vec<i32>]) -> Result<(), &'static str> {
    let size = matrix.len();
    if matrix.iter().any(|row| row.len() != size) { return Err("matrix must be square"); }
    for layer in 0..size / 2 {
        let last = size - 1 - layer;
        for offset in 0..(last - layer) {
            let top = matrix[layer][layer + offset];
            matrix[layer][layer + offset] = matrix[last - offset][layer];
            matrix[last - offset][layer] = matrix[last][last - offset];
            matrix[last][last - offset] = matrix[layer + offset][last];
            matrix[layer + offset][last] = top;
        }
    }
    Ok(())
}

fn main() {
    let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate_clockwise(&mut grid).unwrap();
    println!("{grid:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotates_the_outer_ring() {
        let mut grid = vec![vec![1, 2], vec![3, 4]];
        rotate_clockwise(&mut grid).unwrap();
        assert_eq!(grid, vec![vec![3, 1], vec![4, 2]]);
    }
    #[test]
    fn rejects_non_square_input() {
        assert_eq!(rotate_clockwise(&mut [vec![1], vec![2, 3]]), Err("matrix must be square"));
    }
}
