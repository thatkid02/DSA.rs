mod code150;

use crate::code150::histogram::Solution;

fn main() {
    let result = Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
    println!("{}", result);
}
