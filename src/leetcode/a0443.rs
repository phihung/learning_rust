// https://leetcode.com/problems/string-compression/

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut w = 0;
        let mut cnt = 1;
        for i in 1..=n {
            if i == n || chars[i] != chars[i - 1] {
                chars[w] = chars[i - 1];
                w += 1;
                if cnt > 1 {
                    let n_digits = Self::get_n_digits(cnt);
                    for i in (0..n_digits).rev() {
                        chars[w + i] = ((cnt % 10) as u8 + b'0') as char;
                        cnt /= 10;
                    }
                    w += n_digits;
                }
            }
            cnt += 1;
        }
        w as i32
    }

    #[inline]
    fn get_n_digits(n: usize) -> usize {
        if n >= 1000 {
            4
        } else if n >= 100 {
            3
        } else if n >= 10 {
            2
        } else {
            1
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(|s: &str| Solution::compress(&mut s.chars().collect()));
    }

    fn run_test(func: impl Fn(&str) -> i32) {
        assert_eq!(func("aabbccc"), 6);
        assert_eq!(func("aaaaaaaaaab"), 4);
        assert_eq!(func("caaaaaaaaaaabb"), 6);
        assert_eq!(func("a"), 1);
    }
}
