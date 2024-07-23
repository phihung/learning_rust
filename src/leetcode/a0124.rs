/// https://leetcode.com/problems/binary-tree-maximum-path-sum/description/
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn process(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match node {
                None => (0, i32::MIN),
                Some(node) => {
                    let (l_value, l_max) = process(node.borrow_mut().left.take());
                    let (r_value, r_max) = process(node.borrow_mut().right.take());
                    let val = node.as_ref().borrow().val;
                    let node_max = val + l_value.max(0) + r_value.max(0);
                    let node_value = val + l_value.max(r_value).max(0);
                    (node_value, node_max.max(l_max).max(r_max))
                }
            }
        }
        process(root).1
    }
}

use super::utils::TreeNode;

pub struct Solution {}

#[test]
fn test_solution() {
    use super::utils::array_to_tree;
    let func = |tree: &[i32]| Solution::max_path_sum(array_to_tree(tree));
    assert_eq!(func(&[-3]), -3);
    assert_eq!(func(&[2, -1]), 2);
    assert_eq!(func(&[1, 2, 3]), 6);
    assert_eq!(func(&[-10, 9, 20, 0, 0, 15, 7]), 42);
}
