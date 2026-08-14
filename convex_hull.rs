#[derive(Debug, Clone, Copy, PartialEq)]
struct Point { x: f64, y: f64 }

fn cross(o: &Point, a: &Point, b: &Point) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap().then(a.y.partial_cmp(&b.y).unwrap()));
    points.dedup_by(|a, b| a == b);
    let n = points.len();
    if n < 3 { return points; }

    let mut hull: Vec<Point> = Vec::new();

    for &p in &points {
        while hull.len() >= 2 && cross(&hull[hull.len()-2], &hull[hull.len()-1], &p) <= 0.0 {
            hull.pop();
        }
        hull.push(p);
    }

    let lower_size = hull.len() + 1;
    for &p in points.iter().rev() {
        while hull.len() >= lower_size && cross(&hull[hull.len()-2], &hull[hull.len()-1], &p) <= 0.0 {
            hull.pop();
        }
        hull.push(p);
    }

    hull.pop();
    hull
}

fn main() {
    let points = vec![
        Point{x:0.0,y:0.0}, Point{x:1.0,y:1.0}, Point{x:2.0,y:2.0},
        Point{x:0.0,y:2.0}, Point{x:2.0,y:0.0}, Point{x:1.0,y:0.5},
    ];
    let hull = convex_hull(points);
    println!("Convex hull points: {:?}", hull);
}
