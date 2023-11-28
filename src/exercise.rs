#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[allow(unused)]
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut result = vec![];
    while !queue.is_empty() {
        let mut inner = vec![];
        for _ in 0..queue.len() {
            if let Some(Some(node)) = queue.pop_front() {
                let immut_node = node.borrow();
                inner.push(immut_node.val);
                queue.push_back(immut_node.left.clone());
                queue.push_back(immut_node.right.clone());
            }
        }
        if !inner.is_empty() {
            result.push(inner);
        }
    }
    result
}

#[allow(unused)]
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node);
        let mut result = vec![];
        while !queue.is_empty() {
            for index in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    let immut_node = node.borrow();
                    if index == 0 {
                        result.push(immut_node.val);
                    }
                    if let Some(node) = immut_node.right.clone() {
                        queue.push_back(node);
                    }
                    if let Some(node) = immut_node.left.clone() {
                        queue.push_back(node);
                    }
                }
            }
        }
        result
    } else {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::exercise::{right_side_view, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_insert() {
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        })));
        println!("{:?}", right_side_view(node));
    }
}
