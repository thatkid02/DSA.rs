mod code150;

use crate::code150::max_sliding_window::Solution;

fn main() {
    let nums = vec![1,3,-1,-3,5,3,6,7];
    let k = 3;
    let result = Solution::max_sliding_window(nums, k);
    println!("Result: {:?}", result);
}
