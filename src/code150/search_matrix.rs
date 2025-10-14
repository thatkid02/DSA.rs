pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let m: i32 = matrix.len() as i32;
        let n: i32 = matrix[0].len() as i32;
        let mut low: i32 = 0;
        let mut high: i32 = m * n - 1;

        while low <= high{
            let mid: i32 = low + (high - low) / 2;
            let mid_row: usize = (mid / n) as usize;
            let mid_col: usize = (mid % n) as usize;
            
            if matrix[mid_row][mid_col] == target{
                return true;
            }else if matrix[mid_row][mid_col] > target {
                high = mid - 1;
            }else {
                low = mid + 1;
            }
        }

        false
    }
}