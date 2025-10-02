mod code150;

use crate::code150::long_subs::Solution;

fn main() {
    let s = String::from("abcabcbb");
    let length = Solution::length_of_longest_substring(s);
    println!("Res: {}", length);
}
