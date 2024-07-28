// https://leetcode.com/problems/wildcard-matching/description/
#[derive(Debug, Clone, PartialEq, Eq)]
enum Pattern<'a> {
    AnyStr(usize),
    AnyChar(usize),
    Exact(&'a str),
}
use Pattern::*;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        is_match_dynamic_programming(&s, &p)
        // NfaSolution::is_match(s, p)
    }
}

fn is_match_dynamic_programming(s: &str, p: &str) -> bool {
    let patterns = NfaSolution::parse(p);
    let n = s.len();

    #[derive(Debug, PartialEq)]
    enum MatchState {
        Once(usize),           // at index
        All(usize),            // from index
        Few(Vec<bool>, usize), // mask + offset
        Fail,                  // KO
    }
    let mut current = MatchState::Once(0);
    let next_pattern_is_anystr = |j: usize| {
        if let Some(AnyStr(_)) = patterns.get(j + 1) {
            true
        } else {
            false
        }
    };
    for (j, x) in patterns.iter().enumerate() {
        current = match (x, current) {
            (AnyStr(min_len), MatchState::Once(pos)) => MatchState::All(pos + min_len),
            (AnyChar(len), MatchState::Once(pos)) => MatchState::Once(pos + len),
            (AnyChar(len), MatchState::Few(mask, offset)) => MatchState::Few(mask, offset + len),
            (Exact(substr), MatchState::Once(pos)) => {
                if pos <= s.len() && s[pos..].starts_with(substr) {
                    MatchState::Once(pos + substr.len())
                } else {
                    MatchState::Fail
                }
            }
            (Exact(ref substr), MatchState::All(pos)) => {
                if pos > s.len() {
                    MatchState::Fail
                } else if j == patterns.len() - 1 {
                    return pos + substr.len() <= s.len() && s.ends_with(substr);
                } else if next_pattern_is_anystr(j) {
                    if let Some(i) = s[pos..].find(substr) {
                        MatchState::Once(i + pos + substr.len())
                    } else {
                        MatchState::Fail
                    }
                } else {
                    let mut mask = vec![false; n];
                    let mut start = pos;
                    while let Some(i) = s[start..].find(substr) {
                        mask[start + i] = true;
                        start += i + 1;
                    }
                    MatchState::Few(mask, substr.len())
                }
            }
            (Exact(substr), MatchState::Few(mut mask, offset)) => {
                if next_pattern_is_anystr(j) {
                    let found = (0..(n - offset - substr.len() + 1))
                        .position(|i| mask[i] && s[i + offset..].starts_with(substr));
                    if let Some(i) = found {
                        MatchState::Once(i + offset + substr.len())
                    } else {
                        MatchState::Fail
                    }
                } else {
                    for i in 0..(n - offset - substr.len() + 1) {
                        if mask[i] && !s[i + offset..].starts_with(substr) {
                            mask[i] = false;
                        }
                    }
                    MatchState::Few(mask, offset + substr.len())
                }
            }
            // (AnyStr(_), MatchState::All(_)) => unreachable!(),
            // (AnyStr(_), MatchState::Few(_, _)) => unreachable!(),
            // (AnyChar(_), MatchState::All(_)) => unreachable!(),
            _ => unreachable!(),
        };
        if current == MatchState::Fail {
            return false;
        }
    }
    match current {
        MatchState::Once(pos) => pos == s.len(),
        MatchState::All(pos) => pos <= s.len(),
        MatchState::Few(mask, offset) => offset <= s.len() && mask[s.len() - offset],
        MatchState::Fail => false,
    }
}

impl NfaSolution {
    pub fn is_match(s: &str, p: &str) -> bool {
        let patterns = Self::parse(p);
        Self::run(&patterns, s)
    }

    fn run(nodes: &Vec<Pattern>, s: &str) -> bool {
        let chars: &[u8] = s.as_bytes();
        let mut visited = vec![vec![false; nodes.len()]; chars.len()];
        let mut stack = vec![(0, 0)];
        while let Some((i, current_node)) = stack.pop() {
            if i >= chars.len() {
                if i == chars.len() && Self::match_empty_string(&nodes[current_node..]) {
                    return true;
                }
                continue;
            }
            if current_node >= nodes.len() {
                continue;
            }
            if visited[i][current_node] {
                continue;
            }
            visited[i][current_node] = true;

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
pub struct NfaSolution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let func = |s: &str, p: &str| Solution::is_match(s.to_string(), p.to_string());
        run_test(func);
    }

    #[test]
    fn test_dp() {
        run_test(is_match_dynamic_programming);
    }

    #[test]
    fn test_nfa() {
        run_test(NfaSolution::is_match);
    }

    fn run_test(func: impl Fn(&str, &str) -> bool) {
        let test = |p: &str, yes: &[&str], no: &[&str]| {
            yes.into_iter()
                .for_each(|s| assert_eq!(func(s, p), true, "[{s}] vs {p}"));
            no.into_iter()
                .for_each(|s| assert_eq!(func(s, p), false, "[{s}] vs {p}"));
        };

        assert!(!func("aa", "???*a?"));
        assert!(!func("mississippi", "m??*ss*?i*pi"));
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
    fn benchmark_nfa_1(b: &mut test::Bencher) {
        let n = 2000;
        let s = create_str(&[("ab", n / 2)]);
        let p = create_str(&[("a?", n / 2 - 1), ("bb", 1)]);
        b.iter(|| assert_eq!(NfaSolution::is_match(&s, &p), false, "{s}, {p}"));
    }

    #[bench]
    fn benchmark_nfa_2(b: &mut test::Bencher) {
        let n = 2000;
        let s = create_str(&[("ab", n / 2)]);
        let p = create_str(&[("a?", n / 2 - 1), ("bb", 1)]);
        b.iter(|| assert_eq!(NfaSolution::is_match(&s, &p), false, "{s}, {p}"));
    }

    #[bench]
    fn benchmark_dp_1(b: &mut test::Bencher) {
        let n = 2000;
        let s = create_str(&[("ab", n / 2)]);
        let p = create_str(&[("a?", n / 2 - 1), ("bb", 1)]);
        b.iter(|| assert_eq!(is_match_dynamic_programming(&s, &p), false, "{s}, {p}"));
    }

    #[bench]
    fn benchmark_dp_2(b: &mut test::Bencher) {
        let n = 2000;
        let s = create_str(&[("ab", n / 2)]);
        let p = create_str(&[("a?", n / 2 - 1), ("bb", 1)]);
        b.iter(|| assert_eq!(is_match_dynamic_programming(&s, &p), false, "{s}, {p}"));
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
