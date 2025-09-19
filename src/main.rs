mod code150;
use crate::code150::group_anagrams::Solution;

fn main() {
    let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let groups = Solution::group_anagrams(strs);
    println!("{:?}", groups);
}