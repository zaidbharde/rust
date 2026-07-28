//! Smart Pointers — Box, Rc, RefCell, and Weak references.

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;

// ── Box — heap allocation ─────────────────────────────────────────────
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self { List::Nil }

    fn push(self, value: i32) -> Self {
        List::Cons(value, Box::new(self))
    }

    fn sum(&self) -> i32 {
        match self {
            List::Cons(val, rest) => val + rest.sum(),
            List::Nil             => 0,
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Cons(v, rest) => write!(f, "{} → {}", v, rest),
            List::Nil           => write!(f, "Nil"),
        }
    }
}

// ── Rc + RefCell — shared mutable state ──────────────────────────────
#[derive(Debug)]
struct Node {
    value:    i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent:   RefCell<Weak<Node>>,    // Weak to avoid reference cycles
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            children: RefCell::new(vec![]),
            parent:   RefCell::new(Weak::new()),
        })
    }

    fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }

    fn print_tree(&self, depth: usize) {
        println!("{}Node({})", "  ".repeat(depth), self.value);
        for child in self.children.borrow().iter() {
            child.print_tree(depth + 1);
        }
    }
}

fn main() {
    println!("=== Smart Pointers ===");

    // ── Box + recursive type ──────────────────────────────────────────
    println!("\n--- Box (recursive list) ---");
    let list = List::new().push(1).push(2).push(3).push(4).push(5);
    println!("  List : {}", list);
    println!("  Sum  : {}", list.sum());

    // ── Box<dyn Trait> ────────────────────────────────────────────────
    println!("\n--- Box<dyn Trait> ---");
    trait Drawable { fn draw(&self) -> String; }

    struct Circle  { radius: f64 }
    struct Square  { side:   f64 }

    impl Drawable for Circle { fn draw(&self) -> String { format!("Circle(r={})", self.radius) } }
    impl Drawable for Square { fn draw(&self) -> String { format!("Square(s={})", self.side)   } }

    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Square { side:   4.0 }),
        Box::new(Circle { radius: 1.5 }),
    ];
    for shape in &shapes { println!("  {}", shape.draw()); }

    // ── Rc — multiple ownership ───────────────────────────────────────
    println!("\n--- Rc (shared ownership) ---");
    let shared = Rc::new(String::from("shared data"));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);
    println!("  Value      : {}", shared);
    println!("  Ref count  : {}", Rc::strong_count(&shared));
    drop(clone1);
    println!("  After drop : {}", Rc::strong_count(&shared));
    drop(clone2);
    println!("  After drop : {}", Rc::strong_count(&shared));

    // ── Rc<Node> tree with Weak parent ────────────────────────────────
    println!("\n--- Rc + RefCell + Weak (tree) ---");
    let root  = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    let grandchild = Node::new(4);

    Node::add_child(&child1, grandchild);
    Node::add_child(&root, child1);
    Node::add_child(&root, child2);

    root.print_tree(0);

    println!("  Root strong count: {}", Rc::strong_count(&root));
}
