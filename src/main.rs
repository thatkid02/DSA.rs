mod code150;

use crate::code150::median_sort_array::Solution;

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let median = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("Median: {}", median);
}
