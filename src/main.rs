
mod code150;
use crate::code150::valid_anagram::Solution;

fn main() { 
   let s = String::from("anagram");
   let t = String::from("nagaram");

   if Solution::is_anagram(s.clone(), t.clone()) {
       println!("{} and {} are anagrams.", s, t);
   } else {
       println!("{} and {} are not anagrams.", s, t);
   }
}
