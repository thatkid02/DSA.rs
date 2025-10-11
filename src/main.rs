mod code150;

use crate::code150::car_fleet::Solution;

fn main() {
    let result = Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]);
    println!("{}", result);
}
