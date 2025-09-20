mod code150;

use crate::code150::top_k_frequent::Solution;

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    println!("{:?}", result);
}