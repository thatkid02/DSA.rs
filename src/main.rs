mod code150;

use crate::code150::min_window::Solution;

fn main() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    let result = Solution::min_window(s, t);
    println!("Result: {}", result);
}
