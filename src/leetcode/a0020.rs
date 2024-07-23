/// https://leetcode.com/problems/valid-parentheses/
/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(&cc) = map.get(&c) {
                if stack.pop() != Some(cc) {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    basic_test_cases(|s| Solution::is_valid(s.to_string()));
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(&str) -> bool,
{
    assert!(func("{}"));
    assert!(func("{}[]"));
    assert!(func("{[]}"));
    assert!(!func("{[}]"));
}
