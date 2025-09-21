mod code150;

use crate::code150::encode_decode::Solution;

fn main() {
    let solution = Solution;
    let encoded = solution.encode(vec!["Hello".into(), "World".into()]);
    println!("{}", encoded);
    let decoded = solution.decode(encoded);
    println!("{:?}", decoded);
}
