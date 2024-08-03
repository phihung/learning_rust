// https://leetcode.com/problems/merge-k-sorted-lists/description/

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        flatten_and_sort(lists)
    }
}

// Binary Heap
pub fn use_binary_heap(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
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
fn flatten_and_sort(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    let cnt = lists.iter().map(count).sum();
    let mut flatlist = Vec::with_capacity(cnt);
    for l in lists.iter_mut() {
        while let Some(x) = l {
            flatlist.push(x.val);
            *l = x.next.take();
        }
    }

    flatlist.sort_unstable();
    create_list(&flatlist)
}

fn count(mut l: &Option<Box<ListNode>>) -> usize {
    let mut cnt = 0;
    while let Some(n) = l {
        cnt += 1;
        l = &n.next;
    }
    cnt
}

fn create_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    for &v in arr {
        *tail = Some(Box::new(ListNode::new(v)));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

use super::utils::ListNode;

pub struct Solution {}

#[test]
fn test_solution() {
    let a = create_list(&[1, 4, 5]);
    let b = create_list(&[1, 3, 4]);
    let c = create_list(&[2, 6]);
    assert_eq!(
        flatten_and_sort(vec![a.clone(), b.clone(), c.clone()]),
        create_list(&[1, 1, 2, 3, 4, 4, 5, 6])
    );
    assert_eq!(
        use_binary_heap(vec![a, b, c]),
        create_list(&[1, 1, 2, 3, 4, 4, 5, 6])
    );
}
