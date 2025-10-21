mod code150;

use crate::code150::reverse_linked_list::{ListNode, Solution};

fn main() {
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    let reversed = Solution::reverse_list(head);
    println!("Reversed: {:?}", reversed);
}
