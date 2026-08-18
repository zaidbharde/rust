use rand::Rng;

struct Node {
    value: i32,
    priority: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn rotate_right(mut node: Box<Node>) -> Box<Node> {
    let mut left = node.left.take().unwrap();
    node.left = left.right.take();
    left.right = Some(node);
    left
}

fn rotate_left(mut node: Box<Node>) -> Box<Node> {
    let mut right = node.right.take().unwrap();
    node.right = right.left.take();
    right.left = Some(node);
    right
}

fn insert(root: Option<Box<Node>>, value: i32, priority: u32) -> Box<Node> {
    match root {
        None => Box::new(Node { value, priority, left: None, right: None }),
        Some(mut node) => {
            if value < node.value {
                node.left = Some(insert(node.left.take(), value, priority));
                if node.left.as_ref().unwrap().priority > node.priority {
                    node = rotate_right(node);
                }
            } else {
                node.right = Some(insert(node.right.take(), value, priority));
                if node.right.as_ref().unwrap().priority > node.priority {
                    node = rotate_left(node);
                }
            }
            node
        }
    }
}

fn inorder(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder(&n.left, result);
        result.push(n.value);
        inorder(&n.right, result);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut root: Option<Box<Node>> = None;

    for value in [5, 3, 8, 1, 4, 7, 9] {
        let priority: u32 = rng.gen();
        root = Some(insert(root, value, priority));
    }

    let mut result = Vec::new();
    inorder(&root, &mut result);
    println!("Inorder (sorted): {:?}", result);
}
