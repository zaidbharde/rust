use std::collections::HashMap;

#[derive(Debug)]
pub struct TtlCache<K, V> {
    entries: HashMap<K, (V, u64)>,
}

impl<K: Eq + std::hash::Hash, V> TtlCache<K, V> {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn insert(&mut self, key: K, value: V, expires_at: u64) {
        self.entries.insert(key, (value, expires_at));
    }

    pub fn get(&mut self, key: &K, now: u64) -> Option<&V> {
        let expired = self.entries.get(key).is_some_and(|(_, deadline)| *deadline <= now);
        if expired {
            self.entries.remove(key);
            return None;
        }
        self.entries.get(key).map(|(value, _)| value)
    }

    pub fn purge(&mut self, now: u64) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, (_, deadline)| *deadline > now);
        before - self.entries.len()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K: Eq + std::hash::Hash, V> Default for TtlCache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
