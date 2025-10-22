mod code150;

use crate::code150::merge_sorted_list::{ListNode, Solution};

fn main() {
    let list1 = Some(Box::new(ListNode::new(1)));
    let list2 = Some(Box::new(ListNode::new(2)));
    let merged = Solution::merge_two_lists(list1, list2);
    println!("Merged: {:?}", merged);
}
