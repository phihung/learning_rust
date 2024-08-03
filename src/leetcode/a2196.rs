// https://leetcode.com/problems/create-binary-tree-from-descriptions/description/

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(mut descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut val_to_node = HashMap::new();
        let mut get_or_create = |val, has_parent| {
            let e = val_to_node.entry(val).or_insert_with(|| {
                let node = Rc::new(RefCell::new(TreeNode::new(val)));
                (node, has_parent)
            });
            e.1 |= has_parent;
            e.0.clone()
        };

        while let Some(&[v_parent, v_child, is_left]) = descriptions.pop().as_deref() {
            let n_parent = get_or_create(v_parent, false);
            let n_child = get_or_create(v_child, true);
            match is_left {
                1 => n_parent.borrow_mut().left = Some(n_child),
                _ => n_parent.borrow_mut().right = Some(n_child),
            }
        }

        // faster than val_to_node.values().into_iter()....
        val_to_node
            .into_iter()
            .find(|(_, (_, has_parent))| !*has_parent)
            .map(|(_, (node, _))| node)
    }
}

// ---- test ----

#[test]
fn test_solution() {
    let func = |descriptions: &[[i32; 3]]| {
        Solution::create_binary_tree(descriptions.iter().map(|v| v.to_vec()).collect())
    };
    assert_eq!(
        func(&[
            [20, 15, 1],
            [20, 17, 0],
            [50, 20, 1],
            [50, 80, 0],
            [80, 19, 1]
        ]),
        // [50, 20, 80, 15, 17, 19]
        Some(Rc::new(RefCell::new(TreeNode {
            val: 50,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 80,
                left: Some(Rc::new(RefCell::new(TreeNode::new(19)))),
                right: None,
            }))),
        })))
    );
}

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

pub struct Solution {}
