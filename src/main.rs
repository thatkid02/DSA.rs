
mod code150;
use crate::code150::contains_duplicate::Solution;

fn main() {
    let nums = vec![1, 2, 3, 1];
    let result = Solution::contains_duplicate(nums);
    println!("{}", result);
}
