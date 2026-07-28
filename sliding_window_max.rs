use std::collections::VecDeque;

fn sliding_window_max(nums: &[i32], k: usize) -> Vec<i32> {
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut result = Vec::new();

    for i in 0..nums.len() {
        while let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }
        while let Some(&back) = deque.back() {
            if nums[back] < nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    result
}

fn main() {
    let nums = [1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    println!("{:?}", sliding_window_max(&nums, k));
}
