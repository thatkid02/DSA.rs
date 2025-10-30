#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut head = head;
            let mut prev = None;
            while let Some(mut node) = head {
                head = node.next.take();
                node.next = prev.take();
                prev = Some(node);
            }
            prev
        }

        let mut res = Some(Box::new(ListNode::new(0)));
        let mut prev = &mut res;
        let mut cur = head;
        while cur.is_some() {
            let mut q = &mut cur;
            for _ in 0..k - 1 {
                if q.is_none() {
                    break;
                }
                q = &mut q.as_mut().unwrap().next;
            }
            if q.is_none() {
                prev.as_mut().unwrap().next = cur;
                return res.unwrap().next;
            }

            let b = q.as_mut().unwrap().next.take();
            prev.as_mut().unwrap().next = reverse(cur);
            while prev.is_some() && prev.as_mut().unwrap().next.is_some() {
                prev = &mut prev.as_mut().unwrap().next;
            }
            cur = b;
        }
        res.unwrap().next
    }
}
