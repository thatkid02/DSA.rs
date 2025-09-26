mod code150;

use crate::code150::two_sum_sorted::Solution;

fn main() {
   let numbers = vec![2, 7, 11, 15];
   let target = 9;
   let result = Solution::two_sum(numbers, target);
   println!("The answer to this is {:?}", result);
}