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

#[macro_export]
macro_rules! to_owned {
    ($x:expr) => {
        $crate::leetcode::utils::DeepToOwned::deep_to_owned(&$x)
    };
}

pub trait DeepToOwned {
    type Owned;
    fn deep_to_owned(&self) -> Self::Owned;
}

macro_rules! impl_vec_deep_to_owned {
    () => {
        type Owned = Vec<T::Owned>;

        fn deep_to_owned(&self) -> Self::Owned {
            self.iter().map(|y| y.deep_to_owned()).collect()
        }
    };
}

impl DeepToOwned for String {
    type Owned = String;

    fn deep_to_owned(&self) -> Self::Owned {
        self.clone()
    }
}

impl DeepToOwned for &str {
    type Owned = String;

    fn deep_to_owned(&self) -> Self::Owned {
        self.to_string()
    }
}

pub trait Numeric {}
impl Numeric for f64 {}
impl Numeric for f32 {}
impl Numeric for i64 {}
impl Numeric for i32 {}
impl Numeric for i16 {}
impl Numeric for i8 {}
impl Numeric for isize {}
impl Numeric for u64 {}
impl Numeric for u32 {}
impl Numeric for u16 {}
impl Numeric for u8 {}
impl Numeric for usize {}

impl<T: Numeric + Copy> DeepToOwned for T {
    type Owned = T;

    fn deep_to_owned(&self) -> Self::Owned {
        *self
    }
}

impl<T: DeepToOwned> DeepToOwned for Vec<T> {
    impl_vec_deep_to_owned!();
}

impl<T: DeepToOwned> DeepToOwned for &[T] {
    impl_vec_deep_to_owned!();
}

impl<T: DeepToOwned, const N: usize> DeepToOwned for &[T; N] {
    impl_vec_deep_to_owned!();
}

impl<T: DeepToOwned, const N: usize> DeepToOwned for [T; N] {
    impl_vec_deep_to_owned!();
}
