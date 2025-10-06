mod code150;

use crate::code150::valid_parentheses::Solution;

fn main() {
    let test_str = "{[()]}".to_string();
    let result = Solution::is_valid(test_str);
    println!("Is valid parentheses: {}", result);
}
