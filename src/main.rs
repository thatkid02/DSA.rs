mod code150;

use crate::code150::three_sum::Solution;

fn main() {
   let numbers = vec![-1, 0, 1, 2, -1, -4];
   let result = Solution::three_sum(numbers);
   println!("The answer to this is {:?}", result);
}