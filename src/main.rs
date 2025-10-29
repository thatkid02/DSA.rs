mod code150;

fn main() {
    use code150::merge_k_list::{ListNode, Solution};

    let mut list1 = Box::new(ListNode::new(1));
    list1.next = Some(Box::new(ListNode::new(4)));
    list1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

    let mut list2 = Box::new(ListNode::new(1));
    list2.next = Some(Box::new(ListNode::new(3)));
    list2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut list3 = Box::new(ListNode::new(2));
    list3.next = Some(Box::new(ListNode::new(6)));

    let lists = vec![Some(list1), Some(list2), Some(list3)];

    let result = Solution::merge_k_lists(lists);

    let mut current = result;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
    println!();
}
