/// https://leetcode.com/problems/longest-palindromic-substring/description/
/// Given a string s, return the longest palindromic substring in s.

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut candidats = Self::find_candidates(&chars);
        let mut answer = (0, 1);

        while let Some((start, end)) = candidats.pop() {
            if start > 0 && end < chars.len() && chars[start - 1] == chars[end] {
                candidats.push((start - 1, end + 1));
            } else if end - start > answer.1 - answer.0 {
                answer = (start, end);
            }
        }
        return s.get(answer.0..answer.1).unwrap().to_string();
    }

    // Find candidates. Return [(start_index, end)]
    // - Repeat chars: aaaa
    // - Palindromic of length 3: cdc
    fn find_candidates(chars: &Vec<char>) -> Vec<(usize, usize)> {
        let mut prev = ' ';
        let mut cnt = 0;
        let mut candidats = vec![];

        for (i, &c) in chars.iter().enumerate() {
            if c == prev {
                cnt += 1;
            } else {
                if cnt > 1 {
                    candidats.push((i - cnt, i));
                }
                if i > 1 && chars[i] == chars[i - 2] {
                    candidats.push((i - 2, i + 1));
                }
                prev = c;
                cnt = 1;
            }
        }
        if cnt > 1 {
            candidats.push((chars.len() - cnt, chars.len()));
        }
        candidats
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    basic_test_cases(|s| Solution::longest_palindrome(s.to_string()));
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(&str) -> String,
{
    assert_eq!(func("baaaacdcaa"), "aacdcaa");
    assert_eq!(func("aaa"), "aaa");
    assert_is_in(&func("abab"), &["aba", "bab"]);
}

#[cfg(test)]
fn assert_is_in(answer: &str, solutions: &[&str]) {
    assert!(solutions.contains(&answer))
}
