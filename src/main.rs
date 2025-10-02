mod code150;

use crate::code150::longest_character_replace::Solution;

fn main() {
    let s = String::from("AABABBA");
    let k = 1;
    let length = Solution::character_replacement(s, k);
    println!("Res: {}", length);
}
