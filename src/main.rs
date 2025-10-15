mod code150;

use crate::code150::min_sort::Solution;

fn main() {
    let piles = vec![3, 6, 7, 11];
    let ans = Solution::find_min(piles);
    println!("{ans}");
}
