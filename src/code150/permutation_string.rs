pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (mut freq, mut count, mut j, s2) = ([0; 26], 0, 0, s2.as_bytes());
        for b in s1.bytes() {
            count += (freq[(b - b'a') as usize] == 0) as i32;
            freq[(b - b'a') as usize] += 1
        }
        (0..s2.len()).any(|i| {
            let f = freq[(s2[i] - b'a') as usize];
            freq[(s2[i] - b'a') as usize] -= 1;
            if f == 1 {
                count -= 1
            } else if f == 0 {
                count += 1
            }
            while freq[(s2[i] - b'a') as usize] < 0 {
                let f = freq[(s2[j] - b'a') as usize];
                freq[(s2[j] - b'a') as usize] += 1;
                if f == -1 {
                    count -= 1
                } else if f == 0 {
                    count += 1
                }
                j += 1
            }
            count == 0
        })
    }
}
