/// https://leetcode.com/problems/longest-valid-parentheses/description/
/// Given a string containing just the characters '(' and ')', return the length of the longest valid (well-formed) parentheses substring.

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let mut ranges: Vec<(i32, i32)> = vec![];
        let mut longest_length = 0;
        for (i, c) in s.chars().enumerate() {
            let i = i as i32;
            if c == '(' {
                stack.push(i);
            } else if let Some(mut j) = stack.pop() {
                while let Some(range) = ranges.last() {
                    if j <= range.1 {
                        // Concat
                        j = range.0.min(j);
                        ranges.pop();
                    } else {
                        break;
                    }
                }

                ranges.push((j, i + 1));
                longest_length = longest_length.max(i + 1 - j);
            } else {
                stack.clear();
            }
        }
        longest_length as i32
    }
}

// Approach 2: Start with simple `()` candidates and expand iteratively
impl Solution2 {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        let chars: Vec<bool> = s.chars().map(|c| c == '(').collect();
        let mut candidates = vec![];

        // Find initial valid pairs "()" and push their (start, end) into candidates
        let mut previous_open = false;
        for (i, &is_open) in chars.iter().enumerate() {
            if previous_open && !is_open {
                candidates.push((i - 1, i + 1));
            }
            previous_open = is_open;
        }

        let mut first_iteration = true;
        loop {
            let mut changed = false;
            let mut prev = (&mut 0, &mut 0);
            for (i, (start, end)) in candidates.iter_mut().enumerate() {
                if *end == 0 {
                    // Skip already merged/deleted entries
                    continue;
                }

                // Merge with previous
                let should_merge = i > 0 && *start > 0 && *start == *prev.1;
                if should_merge {
                    *start = *prev.0;

                    // Mark previous as deleted
                    *prev.0 = 0;
                    *prev.1 = 0;
                    changed = true;
                }

                // Expand the current segment if possible
                if should_merge || first_iteration {
                    while *start > 0 && *end < n && chars[*start - 1] && !chars[*end] {
                        *start -= 1;
                        *end += 1;
                        changed = true;
                    }
                }

                prev = (start, end);
            }
            if !changed {
                break;
            }
            first_iteration = false;
        }

        let max_len = candidates
            .iter()
            .map(|(start, end)| end - start)
            .max()
            .unwrap_or(0);

        max_len as i32
    }
}

// ---- test ----

pub struct Solution {}
pub struct Solution2 {}

#[test]
fn test_solution() {
    basic_test_cases(|s| Solution::longest_valid_parentheses(s.to_string()));
}

#[test]
fn test_solution2() {
    basic_test_cases(|s| Solution2::longest_valid_parentheses(s.to_string()));
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(&str) -> i32,
{
    assert_eq!(func(""), 0);
    assert_eq!(func("()"), 2);
    assert_eq!(func("()()"), 4);
    assert_eq!(func("(())"), 4);
    assert_eq!(func("(("), 0);
    assert_eq!(func("(()("), 2);
    assert_eq!(func("(()(())"), 6);
}
