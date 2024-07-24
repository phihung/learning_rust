/// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another
/// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/solutions/5485829/dfs-without-recursion-100-speed-100-memory-11mb
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let (mut start_path, mut dest_path) = (None, None);
        Self::dfs_walk(root.unwrap(), &mut |val, path| {
            if val == start_value {
                start_path = Some(path.to_vec());
            }
            if val == dest_value {
                dest_path = Some(path.to_vec());
            }
            start_path.is_none() || dest_path.is_none()
        });

        Self::prepare_output(&start_path.unwrap(), &dest_path.unwrap())
    }

    fn dfs_walk<F>(node: Rc<RefCell<TreeNode>>, process: &mut F)
    where
        F: FnMut(i32, &[bool]) -> bool,
    {
        let mut stack = vec![];
        let mut queue = vec![(0, true, node)];
        while let Some((index, is_right, n)) = queue.pop() {
            let mut n = n.borrow_mut();
            Self::set_or_push(&mut stack, index, is_right);
            if !process(n.val, &stack[..(index + 1)]) {
                break;
            }
            if let Some(n1) = n.left.take() {
                queue.push((index + 1, false, n1));
            }
            if let Some(n1) = n.right.take() {
                queue.push((index + 1, true, n1));
            }
        }
    }

    fn set_or_push<T>(stack: &mut Vec<T>, index: usize, value: T) {
        if index >= stack.len() {
            stack.push(value);
        } else {
            stack[index] = value;
        }
    }

    fn prepare_output(start_path: &[bool], dest_path: &[bool]) -> String {
        let mut i = 0;
        for (a, b) in start_path.iter().zip(dest_path.iter()) {
            if a != b {
                break;
            }
            i += 1;
        }
        let mut out = Vec::with_capacity(start_path.len() + dest_path.len() - 2 * i);
        for _ in 0..(start_path.len() - i) {
            out.push('U' as u8);
        }
        for &d in &dest_path[i..] {
            out.push(if d { 'R' as u8 } else { 'L' as u8 });
        }
        unsafe { String::from_utf8_unchecked(out) }
    }
}

// ---- test ----

use crate::leetcode::utils::TreeNode;
pub struct Solution {}

#[test]
fn test_solution() {
    use super::utils::array_to_tree;
    let func = |arr: &[i32], start, end| Solution::get_directions(array_to_tree(arr), start, end);
    assert_eq!(func(&[5, 1, 2, 3, 0, 6, 4], 3, 6), "UURL");
    assert_eq!(func(&[2, 1], 2, 1), "L");
    assert_eq!(
        func(
            &[12, 3, 10, 4, 9, 0, 0, 0, 5, 13, 2, 6, 11, 8, 0, 0, 7, 0, 0, 0, 0, 0, 0, 1],
            8,
            7
        ),
        "UURR"
    );
}
