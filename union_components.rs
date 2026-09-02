//! Disjoint-set union structure with union-by-size and path compression.

pub struct UnionComponents {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionComponents {
    pub fn new(elements: usize) -> Self {
        Self {
            parent: (0..elements).collect(),
            size: vec![1; elements],
        }
    }

    pub fn find(&mut self, value: usize) -> Option<usize> {
        if value >= self.parent.len() {
            return None;
        }
        let mut root = value;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        let mut current = value;
        while self.parent[current] != current {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }
        Some(root)
    }

    pub fn union(&mut self, left: usize, right: usize) -> bool {
        let (Some(mut a), Some(mut b)) = (self.find(left), self.find(right)) else { return false };
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        true
    }

    pub fn component_size(&mut self, value: usize) -> Option<usize> {
        let root = self.find(value)?;
        Some(self.size[root])
    }

    pub fn components(&mut self) -> usize {
        (0..self.parent.len()).filter(|&index| self.find(index) == Some(index)).count()
    }
}
