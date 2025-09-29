mod code150;

use crate::code150::trap_water::Solution;

fn main() {
    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let result = Solution::trap(height);
    println!("The answer to this is {:?}", result);
}