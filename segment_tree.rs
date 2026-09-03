/// Iterative segment tree for mutable range-sum queries.
pub struct SegmentTree {
    size: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    pub fn new(values: &[i64]) -> Self {
        let size = values.len().next_power_of_two().max(1);
        let mut tree = vec![0; size * 2];
        tree[size..size + values.len()].copy_from_slice(values);
        for index in (1..size).rev() {
            tree[index] = tree[index * 2] + tree[index * 2 + 1];
        }
        Self { size, tree }
    }

    pub fn update(&mut self, index: usize, value: i64) {
        assert!(index < self.size, "index outside tree");
        let mut node = self.size + index;
        self.tree[node] = value;
        while node > 1 {
            node /= 2;
            self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
        }
    }

    pub fn sum(&self, mut left: usize, mut right: usize) -> i64 {
        assert!(left <= right && right <= self.size, "invalid range");
        left += self.size;
        right += self.size;
        let mut total = 0;
        while left < right {
            if left % 2 == 1 { total += self.tree[left]; left += 1; }
            if right % 2 == 1 { right -= 1; total += self.tree[right]; }
            left /= 2;
            right /= 2;
        }
        total
    }
}

fn main() {
    let mut tree = SegmentTree::new(&[2, 4, 6, 8]);
    tree.update(1, 10);
    println!("{}", tree.sum(1, 4));
}
