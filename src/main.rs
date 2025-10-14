mod code150;

use crate::code150::search_matrix::Solution;

fn main() {
    let matrix = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60],
    ];
    let target = 3;
    let result = Solution::search_matrix(matrix, target);
    println!("result: {result}");
}
