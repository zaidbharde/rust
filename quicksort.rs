fn quicksort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 { return; }
    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: PartialOrd + Copy>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

fn main() {
    let mut nums = vec![9, 3, 7, 1, 8, 2, 5];
    quicksort(&mut nums);
    println!("{:?}", nums);
}
