pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut R = vec![HashSet::new(); 9];
        let mut C: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                let ch = board[i][j];
                if ch == '.' {
                    continue;
                }

                if !('1'..='9').contains(&ch) {
                    return false;
                }

                let k = (i / 3) * 3 + (j / 3);

                if R[i].contains(&ch)
                    || C[j].contains(&ch)
                    || boxes[k].contains(&ch)
                {
                    return false;
                }

                R[i].insert(ch);
                C[j].insert(ch);
                boxes[k].insert(ch);
            }
        }

        true
    }
}