// https://leetcode.com/problems/regular-expression-matching/description/
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pattern<'a> {
    AnyStr(usize),
    AnyChar,
    Exact(&'a str),
    Repeat(char),
}
use Pattern::*;

// 0ms. 100%. NFA + memorization
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let patterns = Self::parse(&p);
        Self::run(&patterns, &s)
    }

    fn run(nodes: &Vec<Pattern>, s: &str) -> bool {
        let chars: &[u8] = s.as_bytes();
        let mut visited = HashSet::new();
        let mut stack = vec![(0, 0)];
        while let Some((i, current_node)) = stack.pop() {
            if visited.contains(&(i, current_node)) {
                continue;
            } else {
                visited.insert((i, current_node));
            }
            if i >= chars.len() {
                if Self::match_empty_string(&nodes[current_node..]) {
                    return true;
                }
                continue;
            }

            if current_node >= nodes.len() {
                continue;
            }
            let next_node = current_node + 1;
            match nodes[current_node] {
                AnyStr(min_len) => {
                    stack.push((i + 1, current_node));
                    stack.push((i + min_len, next_node));
                }
                AnyChar => stack.push((i + 1, next_node)),
                Exact(substr) => {
                    let k = substr.len();
                    if s.len() >= i + k && &s[i..(i + k)] == substr {
                        stack.push((i + k, next_node));
                    }
                }
                Repeat(c) => {
                    stack.push((i, next_node));
                    if c as u8 == chars[i] {
                        stack.push((i + 1, current_node));
                    }
                }
            }
        }
        false
    }

    fn match_empty_string(patterns: &[Pattern]) -> bool {
        for p in patterns {
            match p {
                Exact(_) | AnyChar => return false,
                &AnyStr(min_len) if min_len > 0 => return false,
                _ => (),
            }
        }
        true
    }

    fn parse(p: &str) -> Vec<Pattern> {
        let mut patterns = vec![];

        let mut index = 0;
        let chars = p.as_bytes();
        let peek_next = |i: usize| if i + 1 < chars.len() { chars[i + 1] } else { 0 };
        while index < chars.len() {
            let c = chars[index];
            match c {
                b'.' => {
                    if peek_next(index) != b'*' {
                        patterns.push(AnyChar)
                    }
                }
                b'*' => {
                    let prev = chars[index - 1];
                    if prev != b'*' {
                        patterns.push(if prev != b'.' {
                            Repeat(prev as char)
                        } else {
                            AnyStr(0)
                        })
                    }
                }
                _ => {
                    let start = index;
                    let mut next = peek_next(index);
                    while b'a' <= next && next <= b'z' {
                        index += 1;
                        next = peek_next(index);
                    }
                    if next == 0 || next == b'.' {
                        patterns.push(Exact(&p[start..=index]))
                    } else if index > start {
                        patterns.push(Exact(&p[start..index]))
                    }
                }
            }
            index += 1;
        }

        patterns
    }
}

#[test]
fn test_0() {
    assert!(!Solution::is_match("aaa".to_string(), "aaaa".to_string()));
}

#[test]
fn test_3() {
    let a = Solution::parse("a.*a");
    for s in ["aaa", "aba", "aea", "aa", "aaba", "abaa", "abbba"] {
        assert!(Solution::run(&a, s), "{s}");
    }
    for s in ["a", "aab", "aaab"] {
        assert!(!Solution::run(&a, s), "{s}");
    }
}

#[test]
fn test_2() {
    let a = Solution::parse("a.a");
    for s in ["aaa", "aba", "aea"] {
        assert!(Solution::run(&a, s), "{s}");
    }
    for s in ["a", "aa", "aab", "aaab", "aaba", "abaa", "abbba"] {
        assert!(!Solution::run(&a, s), "{s}");
    }
}

#[test]
fn test_1() {
    let a = Solution::parse("ab*a");
    for s in ["aa", "aba", "abbba"] {
        assert!(Solution::run(&a, s), "{s}");
    }
    for s in ["a", "aaba", "abaa"] {
        assert!(!Solution::run(&a, s), "{s}");
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |s: &str, p: &str| Solution::is_match(s.to_string(), p.to_string());
    assert!(func("a", "ab*"));
    assert!(func("abc", "a*.b*b*.*c*c*.*"));
    assert!(func("cbaacacaaccbaabcb", "c*b*b*.*ac*.*bc*a*"));
    assert!(!func("aab", "a.*a"));
    assert!(func("ab", ".*"));
    assert!(!func("mississippi", "mis*is*p*."));
    assert!(func("mississippi", "mis*is*ip*."));
    assert!(func("aaa", "a.a"));
    assert!(func("aab", "c*a*b"));
    assert!(!func("aa", "a"));
    assert!(func("aa", "a*"));
    assert!(func("b", "a*b"));
    assert!(func("ab", "a*b*"));
}

#[test]
fn test_all() {
    let data = get_test_data();

    data.iter().for_each(|(s, p, expect)| {
        assert_eq!(
            Solution::is_match(s.to_string(), p.to_string()),
            *expect,
            "{s}, {p}"
        )
    });
}

#[bench]
fn benchmark(b: &mut test::Bencher) {
    let data = get_test_data();

    b.iter(|| {
        data.iter().for_each(|(s, p, expect)| {
            assert_eq!(
                Solution::is_match(s.to_string(), p.to_string()),
                *expect,
                "{s}, {p}"
            )
        })
    });
}

#[cfg(test)]
pub fn get_test_data() -> Vec<(String, String, bool)> {
    [
        ("aab", "c*a*b", true),
        ("mississippi", "mis*is*p*.", false),
        ("aaa", "ab*ac*a", true),
        ("aaa", "ab*a*c*a", true),
        ("aaca", "ab*a*c*a", true),
        ("a", "ab*", true),
        ("bbbba", ".*a*a", true),
        ("ab", ".*..", true),
        ("ab", ".*..c*", true),
        ("a", ".*.", true),
        ("aasdfasdfasdfasdfas", "aasdf.*asdf.*asdf.*asdf.*s", true),
        ("abbbcd", "ab*bbbcd", true),
        ("bbab", "b*a*", false),
        ("a", "c*.", true),
        ("a", "c*a", true),
        ("b", "a*.", true),
        ("a", ".*a*", true),
        ("a", "..*", true),
        ("aabcbcbcaccbcaabc", ".*a*aa*.*b*.c*.*a*", true),
        ("abbabaaaaaaacaa", "a*.*b.a.*c*b*a*c*", true),
        ("bcaccbbacbcbcab", "b*.c*..*.b*b*.*c*", true),
        ("baabbbaccbccacacc", "c*..b*a*a.*a..*c", true),
        ("abcaaaaaaabaabcabac", ".*ab.a.*a*a*.*b*b*", true),
        ("cbaacacaaccbaabcb", "c*b*b*.*ac*.*bc*a*", true),
        ("cbaacacaaccbaabcb", "c*b*b*.*ac*.*bc*a*", true),
        ("cabbbbcbcacbabc", ".*b.*.ab*.*b*a*c", true),
        ("abbcacbbbbbabcbaca", "a*a*.*a*.*a*.b*a*", true),
        ("aababbabacaabacbbbc", ".b*ac*.*c*a*b*.*", true),
        ("aaabaaaababcbccbaab", "c*c*.*c*a*..*c*", true),
        ("cbccaababcbabac", "c*aab*.*b.b.*.*a*.", false),
        ("caccccaccbabbcb", "c*c*b*a*.*c*.a*a*a*", true),
        ("bbbaccbbbaababbac", ".b*b*.*...*.*c*.", true),
        ("ccbbcabcbbaabaccc", "c*a*.*a*a*.*c*b*b*.", true),
        ("abbaaaabaabbcba", "a*.*ba.*c*..a*.a*.", true),
        ("bbcacbabbcbaaccabc", "b*a*a*.c*bb*b*.*.*", true),
        ("aabccbcbacabaab", ".*c*a*b.*a*ba*bb*", true),
        ("cbbbaccbcacbcca", "b*.*b*a*.a*b*.a*", true),
        ("cbacbbabbcaabbb", "b*c*.*a*..a.*c*.*", true),
        ("abaabababbcbcabbcbc", "b*ab.*.*.*.b..*", true),
        ("caaacccbaababbb", "c*.*b*ba*ac*c*b*.*", true),
        ("abbbaabccbaabacab", "ab*b*b*bc*ac*.*bb*", true),
        ("abbbaabccbaabacab", "ab*b*b*bc*ac*.*bb*", true),
        ("cacbacccbbbccab", ".b*b*.*c*a*.*bb*", true),
        ("abcbccbcbaabbcbb", "c*a.*ab*.*ab*a*..b*", true),
        ("caabbabbbbccccbbbcc", ".b*c*.*.*bb*.*.*", true),
        ("caaccabbbabcacaac", "b*c*b*b*.b*.*c*a*c", true),
        ("cbcaabcbaabccbaa", "c*b*ab*.*b*c*a*", false),
        ("bccbcccbcbbbcbb", "c*c*c*c*c*.*.*b*b*", true),
        ("ccacbcbcccabbab", ".c*a*aa*b*.*b*.*", true),
        ("aabbcbcacbacaaccacc", "c*b*b*.*.*.*a*.*", true),
        ("bcbabcaacacbcabac", "a*c*a*b*.*aa*c*a*a*", true),
        ("acabbabacaccacccabc", "a*.*c*a*.b.*a*.*", true),
        ("babbcccbacaabcbac", "b.*.*c*b*b*.*c*c", true),
        ("cbbbbabaabbacbbc", "a*c*b*.*bb*a*.*a*", true),
        ("accbabbacbbbacb", ".*.*.*a*bba*ba*", false),
        ("ababbcaaabbaccb", "c*c*..*a*a*a*.*", true),
        ("bcabcbcaccabcbb", "a*a*c*a*.*a*c*bc*.", true),
        ("bcbbbacbabccbabbac", "c*.*b*a.*a*a*a*", true),
        ("ccbbbbbacacaaabcaa", ".*ba*.*.b*c*c*b*a.*", true),
        ("acaababbccbaacabcab", "..*bb*b*c*a*c*.*.b", true),
        ("cbabcabbbacbcaca", "a*c*.*a*a*b*c*a*.*", true),
        ("bacacaababbbcbc", ".*a*.*a*.aa*c*b*c", false),
        ("cbabcbbaabbcaca", ".a*b*.*.*b*c*.*b*a*", true),
        ("bbaaaacabccbcac", "b*b*a*c*c*a*c*.*", true),
        ("bcccccbaccccacaa", ".*bb*c*a*b*.*b*b*c*", true),
        ("bcbaccbbbccabaac", "c*.*a*b*ac*a*a*", true),
        ("bacacbacaaabccbcbaa", "a*.c*c*c*a*b*..*", true),
        ("baccbbcbcacacbbc", "c*.*b*c*ba*b*b*.a*", true),
    ]
    .into_iter()
    .map(|(a, b, c)| (a.to_string(), b.to_string(), c))
    .collect()
}
