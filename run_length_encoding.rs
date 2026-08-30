fn encode(text: &str) -> Vec<(char, usize)> {
    let mut encoded = Vec::new();
    let mut current = None;
    let mut count = 0;
    for character in text.chars() {
        if current == Some(character) {
            count += 1;
        } else {
            if let Some(previous) = current { encoded.push((previous, count)); }
            current = Some(character);
            count = 1;
        }
    }
    if let Some(last) = current { encoded.push((last, count)); }
    encoded
}

fn main() {
    for (character, count) in encode("aaabbccccd") {
        print!("{}{} ", character, count);
    }
    println!();
}
