/// https://leetcode.com/problems/scramble-string/description/
use std::collections::HashMap;

// Top 100%
// https://leetcode.com/problems/scramble-string/solutions/5459757/100-speed-100-memory-simple-memoization-recursive/
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let mut memory = HashMap::new();
        Self::is_scramble_recursive(s1, s2, true, true, true, &mut memory)
    }

    pub fn is_scramble_recursive<'a>(
        s1: &'a [u8],
        s2: &'a [u8],
        left: bool,
        right: bool,
        first_call: bool,
        memory: &mut HashMap<(&'a [u8], &'a [u8]), bool>,
    ) -> bool {
        let n = s1.len();
        if !first_call && n < 4 {
            return true;
        }
        if let Some(&o) = memory.get(&(s1, s2)) {
            return o;
        }
        if s1 == s2 {
            return true;
        }
        if left {
            let splits = Self::find_substring(&mut s1.iter(), &mut s2.iter());
            if splits.last() != Some(&n) {
                memory.insert((s1, s2), false);
                return false;
            }
            for i in splits {
                if i < n {
                    if Self::is_scramble_recursive(&s1[..i], &s2[..i], false, true, false, memory)
                        && Self::is_scramble_recursive(
                            &s1[i..],
                            &s2[i..],
                            true,
                            true,
                            false,
                            memory,
                        )
                    {
                        memory.insert((s1, s2), true);
                        return true;
                    }
                }
            }
        }
        if right {
            let splits = Self::find_substring(&mut s1.iter(), &mut s2.iter().rev());
            if splits.last() != Some(&n) {
                memory.insert((s1, s2), false);
                return false;
            }
            for i in splits {
                if i < n {
                    if Self::is_scramble_recursive(
                        &s1[..i],
                        &s2[(n - i)..n],
                        true,
                        false,
                        false,
                        memory,
                    ) && Self::is_scramble_recursive(
                        &s1[i..],
                        &s2[..(n - i)],
                        true,
                        true,
                        false,
                        memory,
                    ) {
                        memory.insert((s1, s2), true);
                        return true;
                    }
                }
            }
        }
        memory.insert((s1, s2), false);
        false
    }

    fn find_substring<'a, I1, I2>(s1: &'a mut I1, s2: &'a mut I2) -> Vec<usize>
    where
        I1: Iterator<Item = &'a u8> + 'a,
        I2: Iterator<Item = &'a u8> + 'a,
    {
        let mut counts = vec![0; u8::MAX as usize];
        let mut delta = 0;
        let mut out = vec![];
        for (i, (&c1, &c2)) in s1.zip(s2).enumerate() {
            let v2 = counts[c2 as usize];
            if v2 == 0 {
                delta += 1;
            } else if v2 == -1 {
                delta -= 1;
            }
            counts[c2 as usize] = v2 + 1;
            let v1 = counts[c1 as usize];
            if v1 == 0 {
                delta += 1;
            } else if v1 == 1 {
                delta -= 1;
            }
            counts[c1 as usize] = v1 - 1;
            if delta == 0 {
                out.push(i + 1);
            }
        }
        out
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |s1: &str, s2: &str| Solution::is_scramble(s1.to_string(), s2.to_string());
    // assert_eq!(func("abcdbdac", "bdacabcd"), true);
    assert_eq!(func("abcdbdacbdac", "bdacabcdbdac"), true);
    assert_eq!(func("great", "rgeat"), true);
    assert_eq!(func("abcde", "caebd"), false);
    assert_eq!(func("a", "a"), true);
    assert_eq!(func("ab", "ca"), false);
}
