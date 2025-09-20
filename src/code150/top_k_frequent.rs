use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_map = HashMap::new();
        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        let mut freq_vec: Vec<_> = freq_map.into_iter().collect();
        freq_vec.sort_unstable_by_key(|&(_, count)| -count);

        freq_vec.into_iter().take(k as usize).map(|(num, _)| num).collect()
    }
}