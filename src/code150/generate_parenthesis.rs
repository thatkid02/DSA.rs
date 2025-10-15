pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        Self::solver(
            n as usize,
            0,
            0,
            &mut String::with_capacity(n as usize * 2),
            &mut res,
        );
        res
    }

    fn solver(n: usize, open: usize, close: usize, current: &mut String, res: &mut Vec<String>) {
        if current.len() == n * 2 {
            res.push(current.clone());
            return;
        }

        if open < n {
            current.push('(');
            Self::solver(n, open + 1, close, current, res);
            current.pop();
        }

        if close < open {
            current.push(')');
            Self::solver(n, open, close + 1, current, res);
            current.pop();
        }
    }
}
