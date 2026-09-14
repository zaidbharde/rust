use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct LruCache<K, V> { capacity: usize, values: HashMap<K, V>, order: VecDeque<K> }

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self { Self { capacity, values: HashMap::new(), order: VecDeque::new() } }
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.values.contains_key(key) { self.touch(key); }
        self.values.get(key)
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.values.insert(key.clone(), value);
        self.touch(&key);
        while self.values.len() > self.capacity {
            if let Some(oldest) = self.order.pop_front() { self.values.remove(&oldest); }
        }
    }
    fn touch(&mut self, key: &K) {
        self.order.retain(|item| item != key);
        self.order.push_back(key.clone());
    }
    pub fn len(&self) -> usize { self.values.len() }
}

fn main() {
    let mut cache = LruCache::new(2);
    cache.insert("a", 1); cache.insert("b", 2); cache.get(&"a"); cache.insert("c", 3);
    println!("len={}, has_a={}, has_b={}", cache.len(), cache.get(&"a").is_some(), cache.get(&"b").is_some());
}
