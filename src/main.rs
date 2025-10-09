mod code150;

use crate::code150::generate_parenthesis::Solution;

fn main() {
    let result = Solution::generate_parenthesis(3);
    for s in result {
        println!("{}", s);
    }
}
