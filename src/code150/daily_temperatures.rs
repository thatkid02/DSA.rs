pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack = Vec::new();

        for (i, &temp) in temperatures.iter().enumerate() {
            while let Some(&j) = stack.last() {
                if temp > temperatures[j] {
                    result[j] = (i - j) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        result
    }
}
