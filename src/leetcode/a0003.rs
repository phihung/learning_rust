/// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
/// Given a string s, find the length of the longest substring without repeating characters.
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_to_index: HashMap<char, usize> = HashMap::new();
        let mut start = 0;
        let mut max_length = 0;

        for (i, c) in s.chars().enumerate() {
            if let Some(last_seen) = char_to_index.get(&c) {
                start = start.max(last_seen + 1);
            }
            max_length = max_length.max(i - start + 1);
            char_to_index.insert(c, i);
        }
        max_length as i32
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    basic_test_cases(|s: &str| Solution::length_of_longest_substring(s.to_string()));
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(&str) -> i32,
{
    assert_eq!(func("abcabc"), 3);
    assert_eq!(func("dddddd"), 1);
    assert_eq!(func("pwwkew"), 3);
    assert_eq!(func("abcad"), 4);
    assert_eq!(func("abcada"), 4);
    assert_eq!(func("abcadab"), 4);
}
