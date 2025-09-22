mod code150;

use crate::code150::product_except_self::Solution;

fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = Solution::product_except_self(nums);
    println!("{:?}", result);
}
