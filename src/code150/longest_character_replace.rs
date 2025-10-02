pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
    let s: &[u8] = s.as_bytes();
    let mut count: [usize; 26] = [0; 26];

    let (mut m_count, mut res) = (0, 0);
    
    let  mut start = 0;
    for end in 0..s.len() {
        let end_pos = (s[end] - b'A') as usize;
        count[end_pos] += 1;
        m_count = m_count.max(count[end_pos]);  
        
        if end - start + 1 - m_count > k as usize {                                
            count[(s[start] - b'A') as usize] -= 1;
            start += 1;
        }     
        res = res.max(end - start + 1);
      }
      
    res as i32
  }          
}