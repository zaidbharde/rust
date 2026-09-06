use std::collections::VecDeque;

/// Normalize slash-separated paths, resolving `.` and safe `..` segments.
pub fn normalize(path: &str) -> String {
    let absolute = path.starts_with('/');
    let mut parts = VecDeque::new();
    for component in path.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                if parts.back().is_some_and(|part| part != "..") {
                    parts.pop_back();
                } else if !absolute {
                    parts.push_back("..".to_string());
                }
            }
            value => parts.push_back(value.to_string()),
        }
    }
    let joined = parts.into_iter().collect::<Vec<_>>().join("/");
    if absolute {
        format!("/{joined}")
    } else if joined.is_empty() {
        ".".to_string()
    } else {
        joined
    }
}

fn main() {
    println!("{}", normalize("/var/log/../tmp/./app.log"));
}
