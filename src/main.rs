mod code150;

use code150::copy_random_list::{Node, Solution};
use std::cell::RefCell;
use std::rc::Rc;

fn print_list(head: &Option<Rc<RefCell<Node>>>) {
    let mut current = head.clone();
    while let Some(node) = current {
        let node_ref = node.borrow();
        print!("{} -> ", node_ref.val);
        current = node_ref.next.clone();
    }
    println!("None");
}

fn main() {
    // Create a linked list: [[7,null],[13,0],[11,4],[10,2],[1,0]]
    // node: 7 -> 13 -> 11 -> 10 -> 1
    //  * here: 7->null, 13->7, 11->1, 10->11, 1->7

    let node1 = Rc::new(RefCell::new(Node {
        val: 7,
        next: None,
        random: None,
    }));

    let node2 = Rc::new(RefCell::new(Node {
        val: 13,
        next: None,
        random: None,
    }));

    let node3 = Rc::new(RefCell::new(Node {
        val: 11,
        next: None,
        random: None,
    }));

    let node4 = Rc::new(RefCell::new(Node {
        val: 10,
        next: None,
        random: None,
    }));

    let node5 = Rc::new(RefCell::new(Node {
        val: 1,
        next: None,
        random: None,
    }));

    // Build the next pointers: 7 -> 13 -> 11 -> 10 -> 1
    node1.borrow_mut().next = Some(node2.clone());
    node2.borrow_mut().next = Some(node3.clone());
    node3.borrow_mut().next = Some(node4.clone());
    node4.borrow_mut().next = Some(node5.clone());

    // Set random pointers: 7->null, 13->7, 11->1, 10->11, 1->7
    node1.borrow_mut().random = None;
    node2.borrow_mut().random = Some(node1.clone());
    node3.borrow_mut().random = Some(node5.clone());
    node4.borrow_mut().random = Some(node3.clone());
    node5.borrow_mut().random = Some(node1.clone());

    let head = Some(node1.clone());

    println!("existi list:");
    print_list(&head);

    let copied_head = Solution::copy_random_list(head);

    println!("\n\nCopied list:");
    print_list(&copied_head);
}
