mod code150;

use crate::code150::rotate_search::Solution;

fn main() {
        let piles = vec![4,5,6,7,0,1,2];
        let ans = Solution::search(piles, 0);
        println!("{ans}");
}
