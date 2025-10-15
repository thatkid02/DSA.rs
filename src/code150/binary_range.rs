pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = *piles.iter().max().unwrap_or(&1) as i64;
        let mut res = r;

        while l <= r {
            let m = l + (r - l) / 2;
            let hours: i64 = piles.iter().map(|&value| ((value as i64 + m - 1) / m) as i64).sum();

            if hours <= h as i64 {
                res = res.min(m);
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        res as i32
    }
}