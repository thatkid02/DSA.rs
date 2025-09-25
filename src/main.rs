mod code150;

use crate::code150::valid_palindrome::Solution;

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let result = Solution::is_palindrome(s);
    println!("The answer to this {}", result);
}