/// https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/description/
/// Find the number of leaf pairs of distance <= the given number
use std::cell::RefCell;
use std::rc::Rc;

// 5ms, 2.4 mb
impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        // Return for each length i, the number of leafs of distance i from the node
        fn process(node: Rc<RefCell<TreeNode>>, distance: usize, count: &mut i32) -> Vec<i32> {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            match (left, right) {
                // distance <= 10
                (None, None) => vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                (Some(node), None) | (None, Some(node)) => {
                    let mut lengths = process(node, distance, count);
                    for i in (1..distance).rev() {
                        lengths[i] = lengths[i - 1];
                    }
                    lengths[0] = 0;
                    lengths
                }
                (Some(left), Some(right)) => {
                    // length counts: distances from left/right node to its leafs
                    let l_lengths = process(left, distance, count);
                    let r_lengths = process(right, distance, count);

                    let mut out = vec![0; distance];
                    for (i, &i_cnt) in l_lengths[..(distance - 1)].iter().enumerate() {
                        for &j_cnt in &r_lengths[..(distance - 1 - i)] {
                            *count += i_cnt * j_cnt;
                        }
                        out[i + 1] = i_cnt + r_lengths[i];
                    }
                    out
                }
            }
        }
        let mut count = 0;
        process(root.unwrap(), distance as usize, &mut count);
        count
    }
}

use super::utils::TreeNode;

pub struct Solution {}

#[test]
fn test_solution() {
    use super::utils::array_to_tree;
    let func = |tree: &[i32], dist| Solution::count_pairs(array_to_tree(tree), dist);
    assert_eq!(func(&[1, 2, 3, 0, 4], 3), 1);
    assert_eq!(func(&[1, 2, 3, 4, 5, 6, 7], 1), 0);
    assert_eq!(func(&[1, 2, 3, 4, 5, 6, 7], 2), 2);
    assert_eq!(func(&[1, 2, 3, 4, 5, 6, 7], 3), 2);
    assert_eq!(func(&[1, 2, 3, 4, 5, 6, 7], 4), 6);
    assert_eq!(func(&[7, 1, 4, 6, 0, 5, 3, 0, 0, 0, 0, 0, 2], 3), 1);
}
