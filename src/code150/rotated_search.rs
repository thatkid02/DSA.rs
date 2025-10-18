pub struct Solution;

impl Solution {
    pub fn rotated_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left=0;
        let mut right=nums.len() as i32 -1;
        while left<=right{
            let mid =left+(right-left)/2;
            let mid_=nums[mid as usize];
            if mid_==target{
                return mid as i32;
            }
            if nums[left as usize] <= mid_ {
                if nums[left as usize] <= target && target < mid_ {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if mid_ < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}