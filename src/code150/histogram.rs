use std::cmp::max;
pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut res = 0;
        let mut i = 0;

        while i < heights.len() {
            if stack.last().map_or(true, |&j| heights[j] <= heights[i]) {
                stack.push(i);
                i += 1;
            } else {
                let j = stack.pop().unwrap();
                let w = stack.last().map_or(i, |&k| i - k - 1);

                res = max(res, heights[j] * w as i32);
            }
        }
        while let Some(j) = stack.pop() {
            let w = stack.last().map_or(i, |&j| i - j - 1);

            res = max(res, heights[j] * w as i32);
        }
        res
    }
}
