//! Infer lightweight column types from delimited text without external crates.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType { Integer, Decimal, Boolean, Text, Empty }

#[derive(Debug, PartialEq, Eq)]
pub struct ColumnSchema { pub name: String, pub kind: ColumnType, pub non_empty: usize }

fn classify(value: &str) -> ColumnType {
    let value = value.trim();
    if value.is_empty() { return ColumnType::Empty; }
    if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
        return ColumnType::Boolean;
    }
    if value.parse::<i64>().is_ok() { return ColumnType::Integer; }
    if value.parse::<f64>().is_ok() { return ColumnType::Decimal; }
    ColumnType::Text
}

fn merge(left: ColumnType, right: ColumnType) -> ColumnType {
    use ColumnType::*;
    match (left, right) {
        (Empty, other) | (other, Empty) => other,
        (Integer, Integer) => Integer,
        (Integer, Decimal) | (Decimal, Integer) | (Decimal, Decimal) => Decimal,
        (Boolean, Boolean) => Boolean,
        _ => Text,
    }
}

pub fn infer_schema(header: &str, rows: &[&str]) -> Vec<ColumnSchema> {
    let names: Vec<_> = header.split(',').map(str::trim).collect();
    let mut kinds = vec![ColumnType::Empty; names.len()];
    let mut counts = vec![0usize; names.len()];
    for row in rows {
        for (index, value) in row.split(',').enumerate().take(names.len()) {
            let kind = classify(value);
            if kind != ColumnType::Empty { counts[index] += 1; }
            kinds[index] = merge(kinds[index], kind);
        }
    }
    names.into_iter().enumerate().map(|(i, name)| ColumnSchema {
        name: name.to_owned(), kind: kinds[i], non_empty: counts[i],
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn widens_mixed_numeric_values() {
        let schema = infer_schema("id,score,active", &["1,2,true", "2,2.5,false"]);
        assert_eq!(schema[1].kind, ColumnType::Decimal);
        assert_eq!(schema[2].kind, ColumnType::Boolean);
    }
}
