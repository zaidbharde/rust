//! Transpose a rectangular flat row-major matrix with checked dimensions.

/// Returns a row-major representation of the matrix with rows and columns swapped.
/// Empty dimensions are valid only when the input contains no elements.
pub fn transpose<T: Clone>(data: &[T], rows: usize, cols: usize) -> Result<Vec<T>, &'static str> {
    let expected = rows.checked_mul(cols).ok_or("matrix dimensions overflow")?;
    if data.len() != expected {
        return Err("data length does not match matrix dimensions");
    }

    let mut output = Vec::with_capacity(expected);
    for col in 0..cols {
        for row in 0..rows {
            output.push(data[row * cols + col].clone());
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::transpose;

    #[test]
    fn demo_transposes_rectangular_and_edge_case_matrices() {
        assert_eq!(transpose(&[1, 2, 3, 4, 5, 6], 2, 3), Ok(vec![1, 4, 2, 5, 3, 6]));
        assert_eq!(transpose::<i32>(&[], 0, 7), Ok(vec![]));
        assert_eq!(transpose(&[9], 1, 1), Ok(vec![9]));
        assert_eq!(transpose(&[1, 2], 1, 1), Err("data length does not match matrix dimensions"));
        assert_eq!(transpose::<i32>(&[], usize::MAX, 2), Err("matrix dimensions overflow"));
        assert_eq!(transpose::<i32>(&[1], 0, 3), Err("data length does not match matrix dimensions"));
    }
}
