#[derive(Debug, Clone, Copy)]
struct Vec3 { x: f64, y: f64, z: f64 }

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self { Vec3 { x, y, z } }

    fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();
        Vec3::new(self.x / mag, self.y / mag, self.z / mag)
    }
}

fn main() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);

    println!("Dot: {}", a.dot(&b));
    println!("Cross: {:?}", a.cross(&b));
    println!("Magnitude of a: {}", a.magnitude());
    println!("Normalized: {:?}", Vec3::new(3.0, 4.0, 0.0).normalize());
}
