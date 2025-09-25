pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let res: String = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        res == res.chars().rev().collect::<String>()
    }
}