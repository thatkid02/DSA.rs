pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .scan(VecDeque::with_capacity(nums.len()), |double_q, (i, &x)| {
                if i >= k as usize && double_q.front() == Some(&(i - k as usize)) {
                    double_q.pop_front();
                }
                while let Some(&b) = double_q.back() {
                    if nums[b] <= x {
                        double_q.pop_back();
                    } else {
                        break;
                    }
                }
                double_q.push_back(i);
                Some(nums[*double_q.front().unwrap()])
            })
            .skip(k as usize - 1)
            .collect()
    }
}
