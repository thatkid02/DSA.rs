use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<TreeRef>,
    pub right: Option<TreeRef>,
}

pub struct Solution;
type TreeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn invert_tree(root: Option<TreeRef>) -> Option<TreeRef> {
        if let Some(node) = root {
            {
                let mut node_ref = node.borrow_mut();
                let left = node_ref.left.take();
                let right = node_ref.right.take();
                node_ref.left = Self::invert_tree(right);
                node_ref.right = Self::invert_tree(left);
            }
            Some(node)
        } else {
            None
        }
    }
}
