// https://leetcode.com/problems/wildcard-matching/description/
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pattern<'a> {
    AnyStr(usize),
    AnyChar(usize),
    Exact(&'a str),
}
use Pattern::*;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let patterns = Self::parse(&p);
        Self::run(&patterns, &s)
    }

    pub fn is_match_ref(s: &str, p: &str) -> bool {
        let patterns = Self::parse(p);
        Self::run(&patterns, s)
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
                if i == chars.len() && Self::match_empty_string(&nodes[current_node..]) {
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
                AnyChar(len) => {
                    stack.push((i + len, next_node));
                }
                Exact(substr) => {
                    let k = substr.len();
                    if s.len() >= i + k && &s[i..(i + k)] == substr {
                        stack.push((i + k, next_node));
                    }
                }
            }
        }
        false
    }

    fn match_empty_string(patterns: &[Pattern]) -> bool {
        for p in patterns {
            if let &AnyStr(min) = p {
                if min == 0 {
                    continue;
                }
            }
            return false;
        }
        true
    }

    fn parse<'a>(p: &'a str) -> Vec<Pattern> {
        let add_wildcard = |patterns: &mut Vec<Pattern>, min, star| {
            if star {
                patterns.push(AnyStr(min));
            } else if min > 0 {
                patterns.push(AnyChar(min));
            }
        };
        let add_exact = |patterns: &mut Vec<Pattern<'a>>, index, char_count| {
            if char_count > 0 {
                let start = index - char_count;
                patterns.push(Exact(&p[start..index]));
            }
        };
        let mut patterns = vec![];
        let (mut min, mut star, mut char_count) = (0, false, 0);
        for (index, &c) in p.as_bytes().iter().enumerate() {
            (min, star, char_count) = if c == b'*' {
                add_exact(&mut patterns, index, char_count);
                (min, true, 0)
            } else if c == b'?' {
                add_exact(&mut patterns, index, char_count);
                (min + 1, star, 0)
            } else {
                add_wildcard(&mut patterns, min, star);
                (0, false, char_count + 1)
            };
        }
        add_wildcard(&mut patterns, min, star);
        add_exact(&mut patterns, p.len(), char_count);
        patterns
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let func = |s: &str, p: &str| Solution::is_match(s.to_string(), p.to_string());
        let test = |p: &str, yes: &[&str], no: &[&str]| {
            yes.into_iter()
                .for_each(|s| assert_eq!(func(s, p), true, "[{s}] vs {p}"));
            no.into_iter()
                .for_each(|s| assert_eq!(func(s, p), false, "[{s}] vs {p}"));
        };

        assert_eq!(func("a", "??"), false);

        test("?a", &["ba", "aa"], &["", "a", "b", "ab", "aaa"]);
        test("*", &["", "ba", "aa", "a", "b", "ab", "aaa"], &[]);
        test("?", &["a", "b"], &["", "ba", "aa", "abcd", "aaa"]);
        test("??", &["ba", "aa"], &["", "a", "abc"]);
        test(
            "*a",
            &["a", "ba", "aa", "aaa", "adfdsja"],
            &["", "b", "abcd", "ab"],
        );
        test(
            "?*a*",
            &["aa", "ba", "aa", "aaa", "bcda", "bcdaef"],
            &["a", "ab", "", "abcd"],
        );
    }

    #[bench]
    fn benchmark_1(b: &mut test::Bencher) {
        let n = 2000;
        let s = create_str(&[("ab", n / 2)]);
        let p = create_str(&[("a?", n / 2 - 1), ("bb", 1)]);
        b.iter(|| assert_eq!(Solution::is_match_ref(&s, &p), false, "{s}, {p}"));
    }

    #[bench]
    fn benchmark_2(b: &mut test::Bencher) {
        let n = 2000;
        let s = create_str(&[("ab", n / 2)]);
        let p = create_str(&[("a?", n / 2 - 1), ("bb", 1)]);
        b.iter(|| assert_eq!(Solution::is_match_ref(&s, &p), false, "{s}, {p}"));
    }

    fn create_str(ls: &[(&str, usize)]) -> String {
        let n = ls.iter().map(|(p, cnt)| p.len() * cnt).sum();
        let mut s = String::with_capacity(n);
        for &(c, cnt) in ls {
            for _ in 0..cnt {
                s.push_str(c);
            }
        }
        s
    }
}
