use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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

type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            true
        } else {
            return Solution::check_left_right(&root, &root);
        }
    }

    fn check_left_right(left: &Node, right: &Node) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }

        if left.is_none() || right.is_none() {
            return false;
        }

        left.clone().unwrap().as_ref().borrow().val == right.clone().unwrap().as_ref().borrow().val
            && Solution::check_left_right(
                &left.clone().unwrap().as_ref().borrow().left,
                &right.clone().unwrap().as_ref().borrow().right,
            )
            && Solution::check_left_right(
                &right.clone().unwrap().as_ref().borrow().right,
                &left.clone().unwrap().as_ref().borrow().left,
            )
    }
}
