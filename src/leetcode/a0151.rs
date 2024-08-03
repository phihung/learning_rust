// https://leetcode.com/problems/reverse-words-in-a-string/description/

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut b = s.into_bytes();
        let n = b.len();
        for i in 0..(n / 2) {
            b.swap(i, n - 1 - i);
        }
        let (mut i, mut wlen, mut i_write) = (0, 0, 0);
        while i <= n {
            if i == n || b[i] == b' ' {
                if wlen > 0 {
                    if i_write > 0 {
                        b[i_write] = b' ';
                        i_write += 1;
                    }
                    for j in 0..wlen.min((i - i_write) / 2) {
                        b.swap(i_write + j, i - j - 1);
                    }
                    i_write += wlen;
                    wlen = 0;
                }
            } else {
                wlen += 1;
            }
            i += 1;
        }
        unsafe {
            b.set_len(i_write);
            String::from_utf8_unchecked(b)
        }
    }
}

// ab_cde
// cde_ba
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(|s: &str| Solution::reverse_words(s.to_string()))
    }

    fn run_test(func: impl Fn(&str) -> String) {
        assert_eq!(func("  hello world "), "world hello");
        assert_eq!(func("a"), "a");
        assert_eq!(func("  a "), "a");
        assert_eq!(func("huge"), "huge");
        assert_eq!(func("a b"), "b a");
        assert_eq!(func("ab bd"), "bd ab");
        assert_eq!(func(" a  bc  def "), "def bc a");
        assert_eq!(func("the sky is blue"), "blue is sky the");
    }
}

// "the sky is blue"
// "eulb        the"
