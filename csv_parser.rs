fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            fields.push(current.clone());
            current.clear();
        } else {
            current.push(c);
        }
        i += 1;
    }
    fields.push(current);
    fields
}

fn parse_csv(data: &str) -> Vec<Vec<String>> {
    data.lines().map(parse_csv_line).collect()
}

fn main() {
    let csv_data = "name,age,city\nZaid,22,\"Mumbai, MH\"\nJohn,25,Delhi";
    let rows = parse_csv(csv_data);
    for row in rows {
        println!("{:?}", row);
    }
}
