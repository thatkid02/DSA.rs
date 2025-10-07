mod code150;

use crate::code150::min_stack::MinStack;

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(5);
    min_stack.push(2);
    min_stack.push(8);
    println!("Top element: {}", min_stack.top());
    println!("min element: {}", min_stack.get_min());
    min_stack.pop();
    println!("Top element after pop: {}", min_stack.top());
    println!("min element after pop: {}", min_stack.get_min());
}
