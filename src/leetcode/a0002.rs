/// https://leetcode.com/problems/add-two-numbers/description/
/// Each number is represented by a linked-list, in reversed order
///     123 = 3 -> 2 -> 1
/// Find the sum of two linked-lists

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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::_add(&l1, &l2)
    }

    pub fn _add(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut result_head: Option<Box<ListNode>> = None;
        let mut result_tail = &mut result_head;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = match (l1, l2) {
                (Some(node1), Some(node2)) => {
                    l1 = &node1.next;
                    l2 = &node2.next;
                    node1.val + node2.val + carry
                }
                (None, Some(node)) | (Some(node), None) => {
                    l1 = &node.next;
                    l2 = &None;
                    node.val + carry
                }
                (None, None) => carry,
            };

            carry = sum / 10;
            let node = Box::new(ListNode::new(sum % 10));
            if let Some(b) = result_tail {
                b.next = Some(node);
                result_tail = &mut b.next;
            } else {
                *result_tail = Some(node);
            }
        }
        result_head
    }
}

impl Solution2 {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::_sum(l1, l2, 0)
    }

    fn _sum(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        let (v1, next1) = l1.map_or((0, None), |node| (node.val, node.next));
        let (v2, next2) = l2.map_or((0, None), |node| (node.val, node.next));

        let sum = v1 + v2 + carry;
        let new_carry = sum / 10;
        let next = if next1.is_none() && next2.is_none() {
            if new_carry == 0 {
                None
            } else {
                Some(Box::new(ListNode::new(new_carry)))
            }
        } else {
            Self::_sum(next1, next2, new_carry)
        };
        Some(Box::new(ListNode {
            val: sum % 10,
            next,
        }))
    }
}

pub struct Solution {}
pub struct Solution2 {}

// ------ TEST ------

#[cfg(test)]
type OpNode = Option<Box<ListNode>>;

#[cfg(test)]
fn node_from_number(num: i32) -> OpNode {
    let next = if num > 9 {
        node_from_number(num / 10)
    } else {
        None
    };
    Some(Box::new(ListNode {
        val: num % 10,
        next,
    }))
}

#[test]
fn test_solution() {
    basic_test_cases(Solution::add_two_numbers)
}

#[test]
fn test_solution2() {
    basic_test_cases(Solution2::add_two_numbers)
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(OpNode, OpNode) -> OpNode,
{
    assert_eq!(
        func(node_from_number(1), node_from_number(19)),
        node_from_number(1 + 19)
    );
    assert_eq!(
        func(node_from_number(342), node_from_number(465)),
        node_from_number(807)
    );
    assert_eq!(
        func(node_from_number(0), node_from_number(0)),
        node_from_number(0)
    );
    assert_eq!(
        func(node_from_number(9999999), node_from_number(9999)),
        node_from_number(9999999 + 9999)
    );
}
