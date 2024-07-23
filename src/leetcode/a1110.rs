/// https://leetcode.com/problems/delete-nodes-and-return-forest
use std::cell::RefCell;
use std::rc::Rc;

// Given the root of a binary tree, each node in the tree has a distinct value.
// After deleting all nodes with a value in to_delete, we are left with a forest (a disjoint union of trees).
// Return the roots of the trees in the remaining forest.
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut to_delete_set = [false; 1001];
        for x in to_delete {
            to_delete_set[x as usize] = true;
        }

        let mut forest = vec![];
        let mut queue = vec![(None, false, root.unwrap())];
        while let Some((parent, is_left, node)) = queue.pop() {
            let node_ref = node.as_ref().borrow();
            let keep = !to_delete_set[node_ref.val as usize];
            let new_parent = if keep { Some(node.clone()) } else { None };
            if let Some(left) = node_ref.left.clone() {
                queue.push((new_parent.clone(), true, left));
            }
            if let Some(right) = node_ref.right.clone() {
                queue.push((new_parent, false, right));
            }
            if keep {
                if parent.is_none() {
                    forest.push(Some(node.clone()));
                }
            } else {
                if let Some(parent) = parent.clone() {
                    if is_left {
                        parent.borrow_mut().left = None;
                    } else {
                        parent.borrow_mut().right = None;
                    }
                }
            }
        }
        forest
    }
}

use super::utils::TreeNode;

pub struct Solution {}

#[test]
fn test_solution() {
    use super::utils::array_to_tree;
    let test = |root: &[i32], to_delete: &[i32], expected: &[&[i32]]| {
        let root = array_to_tree(root);
        let out = Solution::del_nodes(root, to_delete.to_vec());
        assert_eq!(out.len(), expected.len());
        for &exp in expected {
            let exp = array_to_tree(exp).unwrap();
            let mut found = false;
            for t in out.iter() {
                let t = t.clone().unwrap();
                if t == exp {
                    found = true;
                    break;
                }
            }
            if !found {
                println!("{:#?}", out);
                assert!(false);
            }
        }
    };
    assert_eq!(array_to_tree(&[2, 4, 5]), array_to_tree(&[2, 4, 5]));
    test(&[6], &[6], &[]);
    test(&[6], &[1, 6, 10], &[]);
    test(&[1, 2, 0, 4, 3], &[2, 3], &[&[1], &[4]]);
    test(
        &[1, 2, 3, 4, 5, 6, 7],
        &[9, 3, 5],
        &[&[1, 2, 0, 4], &[6], &[7]],
    );
    test(&[1, 2, 3, 4, 5, 6, 7], &[3, 1, 7], &[&[2, 4, 5], &[6]]);
    test(&[1, 2, 4, 0, 3], &[3], &[&[1, 2, 4]]);
}
