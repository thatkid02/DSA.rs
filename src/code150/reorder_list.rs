pub struct Solution;

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Convert to vector
        let mut nodes = Vec::new();
        let mut current = head.take();

        while let Some(mut node) = current {
            current = node.next.take();
            nodes.push(node.val);
        }

        if nodes.len() <= 2 {
            // Reconstruct and return
            *head = Self::build_list_from_vec(&nodes);
            return;
        }

        let mut left = 0;
        let mut right = nodes.len() - 1;
        let mut reordered = Vec::new();

        while left <= right {
            reordered.push(nodes[left]);
            if left != right {
                reordered.push(nodes[right]);
            }
            left += 1;
            right -= 1;
        }

        // Reconstruct the list
        *head = Self::build_list_from_vec(&reordered);
    }

    fn build_list_from_vec(values: &[i32]) -> Option<Box<ListNode>> {
        if values.is_empty() {
            return None;
        }

        let mut head = Box::new(ListNode::new(values[0]));
        let mut current = &mut head;

        for &val in &values[1..] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }

        Some(head)
    }
}
