mod code150;

use code150::linked_list_cycle::ListNode;
use std::{cell::RefCell, rc::Rc};

fn main() {
    let a = Rc::new(RefCell::new(ListNode::new(3)));
    let b = Rc::new(RefCell::new(ListNode::new(2)));
    let c = Rc::new(RefCell::new(ListNode::new(0)));
    let d = Rc::new(RefCell::new(ListNode::new(4)));
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    c.borrow_mut().next = Some(d.clone());
    d.borrow_mut().next = Some(a.clone());
    println!(
        "Has cycle: {}",
        code150::linked_list_cycle::Solution::has_cycle(Some(a))
    );
}
