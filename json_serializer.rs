enum JsonValue {
    Str(String),
    Num(f64),
    Bool(bool),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    fn to_json(&self) -> String {
        match self {
            JsonValue::Str(s) => format!("\"{}\"", s),
            JsonValue::Num(n) => n.to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_json()).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(obj) => {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v.to_json()))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
    }
}

fn main() {
    let data = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::Str("Zaid".to_string())),
        ("age".to_string(), JsonValue::Num(22.0)),
        ("active".to_string(), JsonValue::Bool(true)),
        ("skills".to_string(), JsonValue::Array(vec![
            JsonValue::Str("Rust".to_string()),
            JsonValue::Str("Java".to_string()),
        ])),
    ]);

    println!("{}", data.to_json());
}
