mod code150;

use crate::code150::reverse_polish::Solution;

fn main() {
    let tokens = vec![
        "2".into(),
        "1".into(),
        "+".into(),
        "3".into(),
        "*".into(),
    ];
    let result = Solution::eval_rpn(tokens);
    println!("Result: {}", result);
}
