struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
        let mut i = self.data.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() { return None; }
        let len = self.data.len();
        self.data.swap(0, len - 1);
        let min = self.data.pop();
        let mut i = 0;
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;
            if left < self.data.len() && self.data[left] < self.data[smallest] { smallest = left; }
            if right < self.data.len() && self.data[right] < self.data[smallest] { smallest = right; }
            if smallest == i { break; }
            self.data.swap(i, smallest);
            i = smallest;
        }
        min
    }
}

fn main() {
    let mut heap = MinHeap::new();
    for v in [5, 3, 8, 1, 9, 2] {
        heap.push(v);
    }
    while let Some(min) = heap.pop() {
        print!("{} ", min);
    }
    println!();
}
