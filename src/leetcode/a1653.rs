// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/description/

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        Self::method_1(s)
    }

    // 0ms
    pub fn method_1(s: String) -> i32 {
        let n_b: i32 = s.as_bytes().iter().map(|&c| (c - b'a') as i32).sum();
        let n_a = s.len() as i32 - n_b;
        let mut min = n_a.min(n_b);
        let mut left_b = 0;
        for (i, &c) in s.as_bytes().into_iter().enumerate() {
            left_b += (c - b'a') as i32;
            let right_a = n_a + left_b - i as i32 - 1;
            min = min.min(left_b + right_a);
            if right_a == 0 || left_b == n_b {
                break;
            }
        }
        min as i32
    }

    pub fn method_2(s: String) -> i32 {
        let n_b: i32 = s.as_bytes().iter().map(|&c| (c - b'a') as i32).sum();
        let n_a = s.len() as i32 - n_b;
        let mut min = n_a;
        let mut left_b_right_a = n_a;
        for &c in s.as_bytes().into_iter() {
            left_b_right_a += 2 * (c - b'a') as i32 - 1;
            min = min.min(left_b_right_a);
        }
        min as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(|s: &str| Solution::minimum_deletions(s.to_string()))
    }

    #[test]
    fn test_method_1() {
        run_test(|s: &str| Solution::method_1(s.to_string()))
    }

    #[test]
    fn test_method_2() {
        run_test(|s: &str| Solution::method_2(s.to_string()))
    }

    fn run_test(func: impl Fn(&str) -> i32) {
        assert_eq!(func("aaaaa"), 0);
        assert_eq!(func("bbbb"), 0);
        assert_eq!(func("aabbbb"), 0);
        assert_eq!(func("baaaba"), 2);
        assert_eq!(func("baaaaa"), 1);
        assert_eq!(func("aababbab"), 2);
        assert_eq!(func("bbaaaaabb"), 2);
        assert_eq!(func("bababababab"), 5);
        assert_eq!(func("bbbbbbbaabbbbbaaabbbabbbbaabbbbbbaabbaaabaabbbaaaabaaababbbabbabbaaaabbbabbbbbaabbababbbaaaaaababaaababaabbabbbaaaabbbbbabbabaaaabbbaba"), 60);
    }
}
