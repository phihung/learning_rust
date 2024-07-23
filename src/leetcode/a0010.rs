/// https://leetcode.com/problems/regular-expression-matching/
/// Regular Expression Matching

#[derive(Debug, PartialEq, Eq)]
enum Matcher {
    AnyStr,
    AnyChar,
    Exact(Vec<char>),
    Repeat(char),
}

use Matcher::*;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let matchers = Self::parse(&p);
        let chars: Vec<char> = s.chars().collect();
        return Self::run_match(&chars, &matchers);
    }

    /// Parse regex pattern. Only support for : characters, . and *
    fn parse(p: &str) -> Vec<Matcher> {
        let mut prev = None;
        let mut substr = vec![];
        let mut matchers = vec![];
        for c in p.chars() {
            if c == '.' {
                if !substr.is_empty() {
                    matchers.push(Exact(substr));
                    substr = vec![];
                }
                if prev == Some('.') {
                    matchers.push(AnyChar);
                }
            } else if c == '*' {
                if prev == Some('.') {
                    matchers.push(AnyStr);
                } else if let Some(x) = prev {
                    if substr.len() > 1 {
                        substr.pop();
                        matchers.push(Exact(substr));
                    }
                    matchers.push(Repeat(x));
                    substr = vec![];
                }
            } else {
                if prev == Some('.') {
                    matchers.push(AnyChar);
                }
                substr.push(c)
            }
            prev = if c != '*' { Some(c) } else { None };
        }
        if !substr.is_empty() {
            matchers.push(Exact(substr));
        }
        if prev == Some('.') {
            matchers.push(AnyChar);
        }
        matchers
    }

    // Find and execute exact match first
    fn run_match(s: &[char], matchers: &[Matcher]) -> bool {
        for (mi, m) in matchers.iter().enumerate() {
            if let Exact(substr) = m {
                let (m_before, m_after) = (&matchers[..mi], &matchers[(mi + 1)..]);
                for (i, window_s) in s.windows(substr.len()).enumerate() {
                    if window_s == substr {
                        let (s_before, s_after) = (&s[0..i], &s[(i + substr.len())..]);
                        let before_ok = Self::run_match2(s_before, m_before);
                        let after_ok = Self::run_match(s_after, m_after);
                        if before_ok && after_ok {
                            return true;
                        }
                    }
                }
                return false;
            }
        }
        Self::run_match2(s, matchers)
    }

    // Match left to right
    fn run_match2(s: &[char], matchers: &[Matcher]) -> bool {
        if matchers.len() == 0 {
            return s.len() == 0;
        }
        let next = &matchers[1..];
        match &matchers[0] {
            AnyStr => {
                for i in 0..(s.len() + 1) {
                    if Self::run_match(&s[i..], next) {
                        return true;
                    }
                }
                return false;
            }
            AnyChar => (s.len() >= 1) && Self::run_match(&s[1..], next),
            Exact(substr) => {
                for (i, c) in substr.iter().enumerate() {
                    if s.len() < i + 1 || c != &s[i] {
                        return false;
                    }
                }
                Self::run_match(s.get(substr.len()..).unwrap(), next)
            }
            Repeat(x) => {
                for i in 0..(s.len() + 1) {
                    if i > 0 && &s[i - 1] != x {
                        break;
                    };
                    if Self::run_match(&s[i..], next) {
                        return true;
                    }
                }
                return false;
            } // Composite(ops) => panic!("Not supported"),
        }
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    basic_test_cases(|s, p| Solution::is_match(s.to_string(), p.to_string()));
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(&str, &str) -> bool,
{
    assert!(func("aab", "c*a*b"));
    assert!(!func("aa", "a"));
    assert!(func("aa", "a*"));
    assert!(func("b", "a*b"));
    assert!(func("ab", ".*"));
    assert!(func("ab", "a*b*"));
}

#[test]
fn test_parse() {
    assert_eq!(Solution::parse("a*b"), vec![Repeat('a'), Exact(vec!['b'])]);
    assert_eq!(Solution::parse("a"), vec![Exact(vec!['a'])]);
    assert_eq!(Solution::parse("ab"), vec![Exact(vec!['a', 'b'])]);
    assert_eq!(Solution::parse("."), vec![AnyChar]);
    assert_eq!(Solution::parse(".."), vec![AnyChar, AnyChar]);
    assert_eq!(Solution::parse("...*"), vec![AnyChar, AnyChar, AnyStr]);
    assert_eq!(
        Solution::parse("a***abc"),
        vec![Repeat('a'), Exact(vec!['a', 'b', 'c'])]
    );
    assert_eq!(
        Solution::parse("c*a*b"),
        vec![Repeat('c'), Repeat('a'), Exact(vec!['b'])]
    );
}
