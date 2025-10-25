use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

pub struct Solution;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut map: HashMap<usize, Rc<RefCell<Node>>> = HashMap::new();

        let mut current = head.clone();
        while let Some(node) = current.clone() {
            let node_ref = node.borrow();
            let new_node = Rc::new(RefCell::new(Node {
                val: node_ref.val,
                next: None,
                random: None,
            }));
            let node_key = node.as_ptr() as usize;
            map.insert(node_key, new_node);
            current = node_ref.next.clone();
        }

        let mut current = head.clone();
        while let Some(node) = current.clone() {
            let node_ref = node.borrow();
            let node_key = node.as_ptr() as usize;
            let new_node = map.get(&node_key).unwrap().clone();

            if let Some(next_node) = &node_ref.next {
                let next_key = next_node.as_ptr() as usize;
                if let Some(next_new) = map.get(&next_key) {
                    new_node.borrow_mut().next = Some(next_new.clone());
                }
            }

            if let Some(random_node) = &node_ref.random {
                let random_key = random_node.as_ptr() as usize;
                if let Some(random_new) = map.get(&random_key) {
                    new_node.borrow_mut().random = Some(random_new.clone());
                }
            }

            current = node_ref.next.clone();
        }

        head.as_ref().and_then(|node| {
            let node_key = node.as_ptr() as usize;
            map.get(&node_key).map(|n| n.clone())
        })
    }
}
