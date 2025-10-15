pub struct Solution;

impl Solution {
    pub fn search(arr: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = arr.len() as i32 - 1;

        while start <= end {
            let mid = start + (end - start) / 2;

            if target > arr[mid as usize] {
                start = mid + 1;
            } else if target < arr[mid as usize] {
                end = mid - 1;
            } else {
                return mid;
            }
        }
        -1
    }
}
