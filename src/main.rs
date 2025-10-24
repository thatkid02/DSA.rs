mod code150;

use code150::reorder_list::{ListNode, Solution};

fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = &node.next;
    }
    println!("None");
}

fn main() {
    // Create a linked list: 1 -> 2 -> 3 -> 4 -> 5
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(4)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(5)));

    println!("Original list:");
    print_list(&head);

    Solution::reorder_list(&mut head);

    println!("Reordered list:");
    print_list(&head);
}
