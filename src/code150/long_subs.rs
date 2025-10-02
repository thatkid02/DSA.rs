pub struct Solution;   

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut i = [0; 128];
        let mut start = 0;
        
        for (end, ch) in s.chars().enumerate() {
            start = start.max(i[ch as usize]);
            res = res.max(end - start + 1);
            i[ch as usize] = end + 1;
        }
        
        res as i32
    }
}