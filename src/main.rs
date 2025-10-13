mod code150;

use crate::code150::binary_search::Solution;

fn main() {
    let result = Solution::search(vec![-1,0,3,5,9,12], 9);
    println!("result: {result}");
}
