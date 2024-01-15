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

struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(100);
        Solution::traverse(&root, &mut ans);

        ans
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        Solution::traverse(&root.as_ref().unwrap().borrow().left, ans);
        Solution::traverse(&root.as_ref().unwrap().borrow().right, ans);
        ans.push(root.as_ref().unwrap().borrow().val);
    }
}
