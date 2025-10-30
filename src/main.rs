mod code150;

fn main() {
    use code150::reverse_k_group::{ListNode, Solution};

    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

    let k = 2;
    let reversed = Solution::reverse_k_group(head, k);

    let mut current = reversed;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
    println!();
}
