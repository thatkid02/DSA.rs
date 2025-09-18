pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = num_map.get(&complement) {
                return vec![j as i32, i as i32];
            }
            num_map.insert(num, i);
        }
        vec![]
    }
}