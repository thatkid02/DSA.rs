mod code150;

use crate::code150::contain_water::Solution;

fn main() {
   let heights = vec![1,8,6,2,5,4,8,3,7];
   let result = Solution::max_area(heights);
   println!("The answer to this is {:?}", result);
}