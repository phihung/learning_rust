use std::{cell::RefCell, rc::Rc};

pub fn array_to_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let n = arr.len();
    if n == 0 {
        return None;
    }
    let create_node = |val| Rc::new(RefCell::new(TreeNode::new(val)));
    let root = create_node(arr[0]);
    let mut nodes = vec![Some(root.clone())];
    let mut i = 0;
    while i < n {
        let mut new_nodes = vec![];
        for node in nodes {
            if let Some(node) = node {
                let mut node = node.borrow_mut();
                i += 1;
                if i >= n {
                    break;
                }
                if arr[i] != 0 {
                    node.left = Some(create_node(arr[i]));
                }
                new_nodes.push(node.left.clone());
                i += 1;
                if i >= n {
                    break;
                }
                if arr[i] != 0 {
                    node.right = Some(create_node(arr[i]));
                }
                new_nodes.push(node.right.clone());
            }
        }
        nodes = new_nodes;
        if nodes.is_empty() {
            break;
        }
    }
    Some(root)
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// [0..n].pow(k)
pub fn carterian<F>(n: i32, k: usize, func: &mut F)
where
    F: FnMut(&[i32]) -> bool,
{
    let mut value = vec![0; k];
    value[k - 1] = -1;
    for _ in 0..n.pow(k as u32) {
        let mut j = k - 1;
        loop {
            value[j] += 1;
            if value[j] < n {
                break;
            }
            value[j] = 0;
            j -= 1;
        }
        if !func(&value) {
            break;
        }
    }
}
