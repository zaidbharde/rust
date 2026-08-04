#[derive(Debug, Clone, Copy)]
struct Point { x: f64, y: f64 }

struct Boundary { x: f64, y: f64, half_width: f64, half_height: f64 }

impl Boundary {
    fn contains(&self, p: &Point) -> bool {
        p.x >= self.x - self.half_width && p.x < self.x + self.half_width &&
        p.y >= self.y - self.half_height && p.y < self.y + self.half_height
    }
}

struct QuadTree {
    boundary: Boundary,
    capacity: usize,
    points: Vec<Point>,
    divided: bool,
    children: Vec<QuadTree>,
}

impl QuadTree {
    fn new(boundary: Boundary, capacity: usize) -> Self {
        QuadTree { boundary, capacity, points: Vec::new(), divided: false, children: Vec::new() }
    }

    fn subdivide(&mut self) {
        let (x, y, hw, hh) = (self.boundary.x, self.boundary.y, self.boundary.half_width / 2.0, self.boundary.half_height / 2.0);
        let offsets = [(-hw, -hh), (hw, -hh), (-hw, hh), (hw, hh)];
        for (dx, dy) in offsets {
            let b = Boundary { x: x + dx, y: y + dy, half_width: hw, half_height: hh };
            self.children.push(QuadTree::new(b, self.capacity));
        }
        self.divided = true;
    }

    fn insert(&mut self, p: Point) -> bool {
        if !self.boundary.contains(&p) { return false; }

        if self.points.len() < self.capacity {
            self.points.push(p);
            return true;
        }

        if !self.divided { self.subdivide(); }
        for child in &mut self.children {
            if child.insert(p) { return true; }
        }
        false
    }
}

fn main() {
    let boundary = Boundary { x: 50.0, y: 50.0, half_width: 50.0, half_height: 50.0 };
    let mut qt = QuadTree::new(boundary, 2);

    let points = [(10.0, 10.0), (20.0, 20.0), (80.0, 80.0), (15.0, 15.0), (90.0, 10.0)];
    for (x, y) in points {
        qt.insert(Point { x, y });
    }

    println!("Inserted {} points into quadtree", points.len());
    println!("Root node points: {:?}", qt.points);
}
