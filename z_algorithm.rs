fn z_function(s: &str) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);

    for i in 1..n {
        if i < r {
            z[i] = std::cmp::min(r - i, z[i - l]);
        }
        while i + z[i] < n && chars[z[i]] == chars[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

fn search(text: &str, pattern: &str) -> Vec<usize> {
    let combined = format!("{}\u{1}{}", pattern, text);
    let z = z_function(&combined);
    let plen = pattern.chars().count();

    z.iter().enumerate()
        .filter(|&(i, &val)| val == plen && i > plen)
        .map(|(i, _)| i - plen - 1)
        .collect()
}

fn main() {
    let text = "abxabcabcabyabc";
    let pattern = "abc";
    println!("Matches at: {:?}", search(text, pattern));
}
