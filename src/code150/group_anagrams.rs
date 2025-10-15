use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars = s.chars().collect::<Vec<_>>();
            chars.sort();
            let sorted = chars.into_iter().collect::<String>();
            res.entry(sorted).or_default().push(s);
        }

        res.into_values().collect()
    }
}
