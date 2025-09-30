mod code150;

use crate::code150::time_to_buy::Solution;

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let max_profit = Solution::max_profit(prices);
    println!("Max profit: {}", max_profit);
}
