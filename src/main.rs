mod code150;

use crate::code150::longest_consecutive_sequence::Solution;

fn main() {
    let nums = vec![1, 2, 3, 1];
    let result = Solution::longest_consecutive(nums);
    println!("The answer to this {}", result);
}