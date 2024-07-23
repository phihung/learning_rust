// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        merge_k_lists_2(lists)
    }
}

// Binary Heap
pub fn merge_k_lists_1(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    let mut mins = BinaryHeap::with_capacity(lists.len());

    for (li, l) in lists.iter_mut().enumerate() {
        if let Some(x) = l {
            mins.push(Reverse((x.val, li)));
            *l = x.next.take();
        }
    }

    let mut head = None;
    let mut tail = &mut head;
    while let Some(Reverse((v, li))) = mins.pop() {
        // println!("{}", v);
        *tail = Some(Box::new(ListNode::new(v)));
        tail = &mut tail.as_mut().unwrap().next;
        if let Some(x) = &mut lists[li] {
            mins.push(Reverse((x.val, li)));
            lists[li] = x.next.take();
        }
    }
    head
}

// Flatten and sort
fn merge_k_lists_2(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    let mut flatlist = Vec::with_capacity(lists.len());
    for l in lists.iter_mut() {
        while let Some(x) = l {
            flatlist.push(x.val);
            *l = x.next.take();
        }
    }

    flatlist.sort();
    create_list(&flatlist)
}

fn create_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    for &v in arr {
        let b = Box::new(ListNode::new(v));
        *tail = Some(b);
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

#[test]
fn test_solution() {
    let a = create_list(&[1, 4, 5]);
    let b = create_list(&[1, 3, 4]);
    let c = create_list(&[2, 6]);
    assert_eq!(
        merge_k_lists_2(vec![a.clone(), b.clone(), c.clone()]),
        create_list(&[1, 1, 2, 3, 4, 4, 5, 6])
    );
    assert_eq!(
        merge_k_lists_1(vec![a, b, c]),
        create_list(&[1, 1, 2, 3, 4, 4, 5, 6])
    );
}
