mod code150;

fn main() {
    use code150::invert_tree::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    type TreeRef = Rc<RefCell<TreeNode>>;

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));

    let inverted = Solution::invert_tree(root);

    if let Some(ref node) = inverted {
        println!("Inverted tree root: {}", node.borrow().val);
        if let Some(ref left) = node.borrow().left {
            println!("Left child: {}", left.borrow().val);
        }
        if let Some(ref right) = node.borrow().right {
            println!("Right child: {}", right.borrow().val);
        }
    }
}
