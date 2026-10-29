//! Strict parser for one CSV record, with byte offsets for malformed input.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsvParseError {
    NewlineInRecord { byte: usize },
    QuoteInUnquotedField { byte: usize },
    CharactersAfterClosingQuote { byte: usize },
    UnterminatedQuotedField,
}

/// Parses one comma-separated record. Quoted fields use doubled quotes (`""`).
/// Newlines are rejected because this function accepts exactly one record.
pub fn parse_csv_record(input: &str) -> Result<Vec<String>, CsvParseError> {
    let mut fields = Vec::new();
    let mut value = String::new();
    let mut quoted = false;
    let mut closed = false;
    let mut field_started = false;
    let mut chars = input.char_indices().peekable();

    while let Some((byte, ch)) = chars.next() {
        if ch == '\n' || ch == '\r' {
            return Err(CsvParseError::NewlineInRecord { byte });
        }
        if quoted {
            match ch {
                '"' if chars.peek().is_some_and(|(_, next)| *next == '"') => {
                    chars.next();
                    value.push('"');
                }
                '"' => { quoted = false; closed = true; }
                _ => value.push(ch),
            }
        } else {
            match ch {
                ',' => {
                    fields.push(std::mem::take(&mut value));
                    closed = false;
                    field_started = false;
                }
                '"' if !field_started => { quoted = true; field_started = true; }
                '"' => return Err(CsvParseError::QuoteInUnquotedField { byte }),
                _ if closed => return Err(CsvParseError::CharactersAfterClosingQuote { byte }),
                _ => { value.push(ch); field_started = true; }
            }
        }
    }
    if quoted { return Err(CsvParseError::UnterminatedQuotedField); }
    fields.push(value);
    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_commas_and_doubled_quotes() {
        assert_eq!(parse_csv_record("a,\"b,c\",\"say \"\"hi\"\"\"").unwrap(),
                   vec!["a", "b,c", "say \"hi\""]);
    }

    #[test]
    fn preserves_empty_and_trailing_fields() {
        assert_eq!(parse_csv_record(",\"\",").unwrap(), vec!["", "", ""]);
        assert_eq!(parse_csv_record("").unwrap(), vec![""]);
    }

    #[test]
    fn rejects_malformed_records_with_locations() {
        assert_eq!(parse_csv_record("é,ab\"c"), Err(CsvParseError::QuoteInUnquotedField { byte: 5 }));
        assert_eq!(parse_csv_record("\"x\"z"), Err(CsvParseError::CharactersAfterClosingQuote { byte: 3 }));
        assert_eq!(parse_csv_record("a\nb"), Err(CsvParseError::NewlineInRecord { byte: 1 }));
        assert_eq!(parse_csv_record("\"unfinished"), Err(CsvParseError::UnterminatedQuotedField));
    }
}
