/// Replaces the four-connected region containing `seed` with `replacement`.
///
/// The grid is a flat row-major slice. Dimensions must match its length; an
/// out-of-bounds seed is rejected. Returns the number of changed cells.
pub fn grid_flood_fill<T: Copy + Eq>(
    grid: &mut [T],
    rows: usize,
    cols: usize,
    seed: (usize, usize),
    replacement: T,
) -> Result<usize, &'static str> {
    let expected = rows.checked_mul(cols).ok_or("grid dimensions overflow")?;
    if grid.len() != expected {
        return Err("grid length does not match dimensions");
    }
    let (row, col) = seed;
    if row >= rows || col >= cols {
        return Err("seed is outside the grid");
    }
    let target = grid[row * cols + col];
    if target == replacement {
        return Ok(0);
    }

    let mut pending = vec![(row, col)];
    let mut changed = 0;
    while let Some((r, c)) = pending.pop() {
        let index = r * cols + c;
        if grid[index] != target {
            continue;
        }
        grid[index] = replacement;
        changed += 1;
        if r > 0 { pending.push((r - 1, c)); }
        if r + 1 < rows { pending.push((r + 1, c)); }
        if c > 0 { pending.push((r, c - 1)); }
        if c + 1 < cols { pending.push((r, c + 1)); }
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::grid_flood_fill;

    #[test]
    fn fills_only_the_four_connected_region() {
        let mut grid = [1, 1, 0, 1, 0, 1, 1, 0, 1];
        assert_eq!(grid_flood_fill(&mut grid, 3, 3, (0, 0), 2), Ok(4));
        assert_eq!(grid, [2, 2, 0, 2, 0, 1, 2, 0, 1]);
    }

    #[test]
    fn handles_noop_and_invalid_shapes_or_seed() {
        let mut grid = [4, 4];
        assert_eq!(grid_flood_fill(&mut grid, 1, 2, (0, 1), 4), Ok(0));
        assert_eq!(grid_flood_fill(&mut grid, 1, 3, (0, 0), 5), Err("grid length does not match dimensions"));
        assert_eq!(grid_flood_fill(&mut grid, 1, 2, (1, 0), 5), Err("seed is outside the grid"));
        assert_eq!(grid_flood_fill::<i32>(&mut [], usize::MAX, 2, (0, 0), 3), Err("grid dimensions overflow"));
    }
}
