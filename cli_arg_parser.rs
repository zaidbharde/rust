use std::collections::HashMap;
use std::env;

fn parse_args(args: &[String]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        if args[i].starts_with("--") {
            let key = args[i].trim_start_matches("--").to_string();
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                map.insert(key, args[i + 1].clone());
                i += 2;
            } else {
                map.insert(key, "true".to_string());
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    map
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed = parse_args(&args[1..]);

    // demo with hardcoded args since no CLI input given
    let demo_args = vec![
        "--name".to_string(), "Zaid".to_string(),
        "--verbose".to_string(),
        "--count".to_string(), "5".to_string(),
    ];
    let demo_parsed = parse_args(&demo_args);
    for (k, v) in &demo_parsed {
        println!("{} = {}", k, v);
    }
    println!("Real parsed (from actual args): {:?}", parsed);
}
