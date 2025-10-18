mod code150;

use crate::code150::rotated_search::Solution;

fn main() {
        let piles = vec![4,5,6,7,0,1,2];
        let target = 0;
        let res = Solution::rotated_search(piles, target);
        println!("res: {}", res);
}
