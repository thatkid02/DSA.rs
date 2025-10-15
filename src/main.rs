mod code150;

use crate::code150::binary_range::Solution;

fn main() {
    let piles = vec![3, 6, 7, 11];
    let h = 8;
    let result = Solution::min_eating_speed(piles, h);
    println!("result: {result}");
}
