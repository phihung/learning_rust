// https://leetcode.com/problems/strange-printer/
impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        // Next position of the same character
        let next: Vec<_> = {
            let mut next = Vec::with_capacity(n);
            let mut prev = [n; 26];
            for (i, &c) in s.into_iter().enumerate() {
                if i < s.len() - 1 && s[i] == s[i + 1] {
                    continue;
                }
                let c = (c - b'a') as usize;
                next.push(n);
                if prev[c] != n {
                    next[prev[c]] = next.len() - 1;
                }
                prev[c] = next.len() - 1;
            }
            next
        };

        let n = next.len();
        let mut dp = vec![vec![0; n]; n];
        for to in 0..n {
            dp[to][to] = 1;
            for from in (0..to).rev() {
                let mut v = dp[from + 1][to] + 1;
                let mut i = next[from];
                while i <= to {
                    v = v.min(dp[from + 1][i - 1] + dp[i][to]);
                    i = next[i];
                }
                dp[from][to] = v;
            }
        }
        dp[0][n - 1]
    }
    // pub fn strange_printer(s: String) -> i32 {
    //     let s = s.as_bytes();

    //     // Next position of the same character
    //     let next: Vec<_> = {
    //         let mut next = Vec::with_capacity(s.len());
    //         let mut prev = [1000; 26];
    //         for (i, &c) in s.iter().enumerate() {
    //             if i < s.len() - 1 && s[i] == s[i + 1] {
    //                 continue;
    //             }
    //             let c = (c - b'a') as usize;
    //             next.push(1000);
    //             if prev[c] != 1000 {
    //                 next[prev[c]] = next.len() - 1;
    //             }
    //             prev[c] = next.len() - 1;
    //         }
    //         next
    //     };

    //     fn recursive(
    //         next: &[usize],
    //         from: usize,
    //         to: usize,
    //         memory: &mut Vec<Vec<usize>>,
    //     ) -> usize {
    //         if from >= to {
    //             return 0;
    //         }
    //         if from + 1 == to {
    //             return 1;
    //         }
    //         if memory[from][to - 1] > 0 {
    //             return memory[from][to - 1];
    //         }
    //         let mut ret = 1 + recursive(next, from + 1, to, memory);
    //         let mut end = next[from];
    //         while end != 1000 {
    //             ret = ret
    //                 .min(recursive(next, from + 1, end, memory) + recursive(next, end, to, memory));
    //             end = next[end];
    //         }
    //         memory[from][to - 1] = ret;
    //         ret
    //     }
    //     recursive(
    //         &next,
    //         0,
    //         next.len(),
    //         &mut vec![vec![0; next.len()]; next.len()],
    //     ) as i32
    // }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |s: &str| Solution::strange_printer(s.to_string());
        assert_eq!(func("abcacba"), 4);
        assert_eq!(func("dddccbdbababaddcbcaabdbdddcccddbbaabddb"), 15);
        assert_eq!(func("abaca"), 3);
        assert_eq!(func("abacada"), 4);
        assert_eq!(func("aabbb"), 2);
        assert_eq!(func("dabcacbd"), 5);
        assert_eq!(func("abcacb"), 4);
        assert_eq!(func("bcacba"), 4);
        assert_eq!(func("abcba"), 3);
        assert_eq!(func("abcab"), 4);

        assert_eq!(func("aba"), 2);
        assert_eq!(func("abca"), 3);
        assert_eq!(func("a"), 1);
        assert_eq!(func("aa"), 1);
        assert_eq!(func("aaab"), 2);
        assert_eq!(func("abab"), 3);
        assert_eq!(func("abcabc"), 5);
    }
}
