#[derive(Debug, PartialEq, Eq)]
pub enum Change<'a> {
    Equal(&'a str),
    Added(&'a str),
    Removed(&'a str),
}

pub fn diff<'a>(before: &'a str, after: &'a str) -> Vec<Change<'a>> {
    let left: Vec<_> = before.lines().collect();
    let right: Vec<_> = after.lines().collect();
    let mut table = vec![vec![0usize; right.len() + 1]; left.len() + 1];
    for i in (0..left.len()).rev() {
        for j in (0..right.len()).rev() {
            table[i][j] = if left[i] == right[j] {
                table[i + 1][j + 1] + 1
            } else {
                table[i + 1][j].max(table[i][j + 1])
            };
        }
    }
    let mut i = 0;
    let mut j = 0;
    let mut changes = Vec::new();
    while i < left.len() && j < right.len() {
        if left[i] == right[j] {
            changes.push(Change::Equal(left[i])); i += 1; j += 1;
        } else if table[i + 1][j] >= table[i][j + 1] {
            changes.push(Change::Removed(left[i])); i += 1;
        } else {
            changes.push(Change::Added(right[j])); j += 1;
        }
    }
    while i < left.len() { changes.push(Change::Removed(left[i])); i += 1; }
    while j < right.len() { changes.push(Change::Added(right[j])); j += 1; }
    changes
}

fn main() {
    for change in diff("keep\nold", "keep\nnew") { println!("{:?}", change); }
}
