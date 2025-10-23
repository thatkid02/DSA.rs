pub struct Solution;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut cache = HashSet::new();
        let mut next = head;

        while let Some(node) = next.clone() {
            if !cache.insert(Rc::as_ptr(&node)) {
                return true;
            }

            next = node.borrow().next.clone();
        }
        false
    }
}
