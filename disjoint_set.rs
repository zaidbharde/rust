//! Disjoint-set union structure for connectivity and component counting.

#[derive(Debug)]
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u8>,
    components: usize,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self { parent: (0..size).collect(), rank: vec![0; size], components: size }
    }

    pub fn find(&mut self, item: usize) -> usize {
        assert!(item < self.parent.len(), "item out of range");
        if self.parent[item] != item {
            let root = self.find(self.parent[item]);
            self.parent[item] = root;
        }
        self.parent[item]
    }

    pub fn union(&mut self, left: usize, right: usize) -> bool {
        let mut root_left = self.find(left);
        let mut root_right = self.find(right);
        if root_left == root_right { return false; }
        if self.rank[root_left] < self.rank[root_right] { std::mem::swap(&mut root_left, &mut root_right); }
        self.parent[root_right] = root_left;
        if self.rank[root_left] == self.rank[root_right] { self.rank[root_left] += 1; }
        self.components -= 1;
        true
    }

    pub fn component_count(&self) -> usize { self.components }
    pub fn connected(&mut self, left: usize, right: usize) -> bool { self.find(left) == self.find(right) }
}
