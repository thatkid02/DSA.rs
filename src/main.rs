mod code150;

use crate::code150::daily_temperatures::Solution;

fn main() {
    let result = Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
    for r in result {
        println!("{}", r);
    }
}
