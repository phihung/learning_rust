// https://leetcode.com/problems/fraction-addition-and-subtraction

impl Solution {
    pub fn fraction_addition(e: String) -> String {
        let e = e.into_bytes();

        let acc_den = 5 * 7 * 8 * 9;
        let mut acc_num = 0;
        let mut whole_part = 0;

        let consume_sign = |i: usize| match e[i] {
            b'+' => (i + 1, 1),
            b'-' => (i + 1, -1),
            _ => (i, 1),
        };
        let consume_num = |i: usize| {
            if i < e.len() - 1 && e[i + 1] == b'0' {
                (i + 2, 10)
            } else {
                (i + 1, (e[i] - b'0') as i32)
            }
        };
        let mut i = 0;
        while i < e.len() {
            let (sign, num, den);
            (i, sign) = consume_sign(i);
            (i, num) = consume_num(i);
            i += 1; // '/'
            (i, den) = consume_num(i);
            if num == 0 {
                continue;
            }
            let num = num * sign;
            whole_part += num / den;
            acc_num += acc_den / den * (num % den);
        }
        let d = Self::gcd(acc_den, acc_num.abs());
        let (acc_num, acc_den) = (acc_num / d + acc_den / d * whole_part, acc_den / d);
        acc_num.to_string() + "/" + &acc_den.to_string()
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |e: &str| Solution::fraction_addition(e.to_string());
        assert_eq!(func("-0/2+0/10+0/9"), "0/1");
        assert_eq!(func("-5/2+3/10+7/9"), "-64/45");
        assert_eq!(func("-5/2+10/3+7/9"), "29/18");
        assert_eq!(func("-1/2+1/2+1/3"), "1/3");
        assert_eq!(func("1/3-1/2"), "-1/6");
        assert_eq!(func("1/7"), "1/7");
        assert_eq!(func("-1/7"), "-1/7");
        assert_eq!(func("-1/7-3/9"), "-10/21");
        assert_eq!(func("-9/3-3/9"), "-10/3");
    }
}
