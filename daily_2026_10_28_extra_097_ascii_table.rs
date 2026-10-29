//! Render rows of text as a consistently sized bordered ASCII table.

/// Renders rows into a bordered table, treating missing cells as empty strings.
///
/// Cell widths count Unicode scalar values (not terminal display columns), and
/// line breaks or other control characters inside cells are replaced by spaces.
/// An empty input produces an empty string.
pub fn render_ascii_table(rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::new();
    }

    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    if columns == 0 {
        let mut output = "+".to_owned();
        for _ in rows {
            output.push_str("\n|\n+");
        }
        return output;
    }

    let clean = |cell: &str| -> String {
        cell.chars()
            .map(|ch| if ch.is_control() { ' ' } else { ch })
            .collect()
    };
    let cleaned: Vec<Vec<String>> = rows
        .iter()
        .map(|row| (0..columns).map(|i| row.get(i).map_or_else(String::new, |s| clean(s))).collect())
        .collect();
    let widths: Vec<usize> = (0..columns)
        .map(|col| cleaned.iter().map(|row| row[col].chars().count()).max().unwrap_or(0))
        .collect();
    let border = format!("+{}", widths.iter().map(|w| format!("{}+", "-".repeat(w + 2))).collect::<String>());
    let mut output = String::new();
    output.push_str(&border);
    for row in cleaned {
        output.push('\n');
        output.push('|');
        for (cell, width) in row.iter().zip(&widths) {
            output.push(' ');
            output.push_str(cell);
            output.push_str(&" ".repeat(width - cell.chars().count() + 1));
            output.push('|');
        }
        output.push('\n');
        output.push_str(&border);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::render_ascii_table;

    #[test]
    fn aligns_cells_and_fills_ragged_rows() {
        let rows = vec![vec!["Name".into(), "Age".into()], vec!["Ada".into(), "37".into()], vec!["Linus".into()]];
        assert_eq!(render_ascii_table(&rows), "+-------+-----+\n| Name  | Age |\n+-------+-----+\n| Ada   | 37  |\n+-------+-----+\n| Linus |     |\n+-------+-----+");
    }

    #[test]
    fn handles_empty_input_and_control_characters() {
        assert_eq!(render_ascii_table(&[]), "");
        assert_eq!(render_ascii_table(&[vec!["a\nb".into()]]), "+-----+\n| a b |\n+-----+");
    }

    #[test]
    fn zero_cell_rows_are_valid() {
        assert_eq!(render_ascii_table(&[vec![], vec![]]), "+\n|\n+\n|\n+");
    }
}
