// https://leetcode.com/problems/find-the-closest-palindrome/

impl Solution {
    pub fn nearest_palindromic(s: String) -> String {
        let n = i64::from_str_radix(&s, 10).unwrap();
        let (add, remove) = Self::get_add_and_remove(s.as_bytes());
        if add != 0 {
            return (if add < remove { n + add } else { n - remove }).to_string();
        }

        // n is already palindrom
        let (_, remove) = Self::get_add_and_remove((n - 1).to_string().as_bytes());
        let (add, _) = Self::get_add_and_remove((n + 1).to_string().as_bytes());
        return (if add < remove {
            n + add + 1
        } else {
            n - remove - 1
        })
        .to_string();
    }

    // Find next and prev palindrom
    fn get_add_and_remove(s: &[u8]) -> (i64, i64) {
        let ndigit = s.len();
        let half = ndigit / 2;

        let (mut l, mut r) = (0, 0);
        let mut cnt_mid_zero = 0;
        let mut cnt_mid_nine = 0;
        let mut all_zero = ndigit % 2 == 0 || s[half] == b'0';
        let mut all_nine = ndigit % 2 == 0 || s[half] == b'9';
        for i in 0..half {
            let (dl, dr) = (s[half - 1 - i] - b'0', s[ndigit - half + i] - b'0');
            l = l * 10 + dl as i64;
            r = r * 10 + dr as i64;
            all_zero &= dl == 0 && dr == 0;
            all_nine &= dl == 9 && dr == 9;
            cnt_mid_zero += all_zero as u32;
            cnt_mid_nine += all_nine as u32;
        }
        if r == 0 && l == 1 && (ndigit % 2 == 0 || s[half] == b'0') {
            // 100..00
            return (1, 1);
        }
        if r == l {
            return (0, 0);
        }

        let (mut remove, mut add) = if r > l {
            (r - l, 10_i64.pow(half as u32 - cnt_mid_nine) - (r - l))
        } else {
            (10_i64.pow(half as u32 - cnt_mid_zero) - (l - r), l - r)
        };
        if (ndigit % 2 == 0 || s[half] == b'9') && r > l {
            add += 10_i64.pow((half - cnt_mid_nine as usize - 1) as u32);
        }
        if (ndigit % 2 == 0 || s[half] == b'0') && r < l {
            remove += 10_i64.pow((half - cnt_mid_zero as usize - 1) as u32);
        }
        return (add, remove);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |e: &str| Solution::nearest_palindromic(e.to_string());
        assert_eq!(func("1"), "0");
        assert_eq!(func("2"), "1");
        assert_eq!(func("22"), "11");
        assert_eq!(func("202"), "212");
        assert_eq!(func("1221"), "1111");
        assert_eq!(func("11"), "9");
        assert_eq!(func("99"), "101");
        assert_eq!(func("999"), "1001");
        assert_eq!(func("101"), "99");
        assert_eq!(func("1991"), "2002"); //+11
        assert_eq!(func("191"), "181");
        assert_eq!(func("219912"), "220022");

        assert_eq!(func("12"), "11");
        assert_eq!(func("19"), "22");
        assert_eq!(func("123"), "121");
        assert_eq!(func("129"), "131");
        assert_eq!(func("199"), "202");
        assert_eq!(func("1200"), "1221");
        assert_eq!(func("1201"), "1221");
        assert_eq!(func("1299"), "1331");
        assert_eq!(func("9199"), "9229");
        assert_eq!(func("113494321"), "113494311");
        assert_eq!(func("10000"), "9999");
        assert_eq!(func("20000"), "20002");
        assert_eq!(func("50000"), "50005");
        assert_eq!(func("60000"), "59995");
        assert_eq!(func("80000"), "79997");
        assert_eq!(func("90000"), "89998");

        assert_eq!(func("990000"), "989989");
        assert_eq!(func("990900"), "991199");
    }
}
