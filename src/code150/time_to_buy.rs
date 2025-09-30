pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut res = 0;
        for i in 1..prices.len() {
            if prices[i] < buy {
                buy = prices[i];
            } else if prices[i] - buy > res {
                res = prices[i] - buy;
            }
        }
        res
    }
}