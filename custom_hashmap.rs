struct CustomHashMap {
    buckets: Vec<Option<(String, i32)>>,
    size: usize,
}

impl CustomHashMap {
    fn new(capacity: usize) -> Self {
        CustomHashMap { buckets: vec![None; capacity], size: 0 }
    }

    fn hash(&self, key: &str) -> usize {
        let mut hash: usize = 0;
        for b in key.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(b as usize);
        }
        hash % self.buckets.len()
    }

    fn insert(&mut self, key: &str, value: i32) {
        let mut idx = self.hash(key);
        loop {
            match &self.buckets[idx] {
                None => {
                    self.buckets[idx] = Some((key.to_string(), value));
                    self.size += 1;
                    return;
                }
                Some((k, _)) if k == key => {
                    self.buckets[idx] = Some((key.to_string(), value));
                    return;
                }
                _ => idx = (idx + 1) % self.buckets.len(),
            }
        }
    }

    fn get(&self, key: &str) -> Option<i32> {
        let mut idx = self.hash(key);
        for _ in 0..self.buckets.len() {
            match &self.buckets[idx] {
                Some((k, v)) if k == key => return Some(*v),
                None => return None,
                _ => idx = (idx + 1) % self.buckets.len(),
            }
        }
        None
    }
}

fn main() {
    let mut map = CustomHashMap::new(16);
    map.insert("age", 22);
    map.insert("year", 2026);
    println!("age = {:?}", map.get("age"));
    println!("year = {:?}", map.get("year"));
    println!("missing = {:?}", map.get("missing"));
}
