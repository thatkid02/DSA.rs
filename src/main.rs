mod code150;

use crate::code150::permutation_string::Solution;

fn main() {
    let s1 = String::from("ab");
    let s2 = String::from("eidbaooo");
    let result = Solution::check_inclusion(s1, s2);
    println!("Result: {}", result);
}
