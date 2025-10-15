pub struct Solution;

impl Solution {
    pub fn encode(&self, strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in strs {
            result.push_str(&s.len().to_string());
            result.push_str("#");
            result.push_str(&s);
        }
        result
    }

    pub fn decode(&self, s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < s.len() {
            let mut j = i;
            while j < s.len() && s.chars().nth(j).unwrap() != '#' {
                j += 1;
            }
            let length = s[i..j].parse::<usize>().unwrap();
            result.push(s[j + 1..j + length + 1].to_string());
            i = j + length + 1;
        }
        result
    }
}
