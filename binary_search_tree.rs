#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn insert(&mut self, value: i32) {
        let child = if value < self.value { &mut self.left } else { &mut self.right };
        match child {
            Some(node) => node.insert(value),
            None => *child = Some(Box::new(Node { value, left: None, right: None })),
        }
    }

    fn contains(&self, value: i32) -> bool {
        if self.value == value { true }
        else if value < self.value { self.left.as_ref().is_some_and(|node| node.contains(value)) }
        else { self.right.as_ref().is_some_and(|node| node.contains(value)) }
    }

    fn inorder(&self, output: &mut Vec<i32>) {
        if let Some(left) = &self.left { left.inorder(output); }
        output.push(self.value);
        if let Some(right) = &self.right { right.inorder(output); }
    }
}

fn main() {
    let mut tree = Node { value: 8, left: None, right: None };
    for value in [3, 10, 1, 6, 14, 4, 7] { tree.insert(value); }
    let mut ordered = Vec::new();
    tree.inorder(&mut ordered);
    println!("{:?}; contains 6: {}", ordered, tree.contains(6));
}
