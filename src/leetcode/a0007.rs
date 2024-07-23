/// https://leetcode.com/problems/reverse-integer/description/
/// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // -2147483648 -> 2 147 483 647
        let last_digit = (x % 10).abs();
        if (x < 1_000_000_000 && x > -1_000_000_000) || last_digit == 1 {
            Self::simple_reverse(x)
        } else if last_digit > 2 {
            0
        } else {
            Self::checked_reverse(x)
        }
    }

    fn simple_reverse(x: i32) -> i32 {
        let mut x = x;
        let mut z: i32 = 0;
        while x != 0 {
            let last_digit = x % 10;
            x = x / 10;
            z = z * 10 + last_digit;
        }
        z
    }

    fn checked_reverse(x: i32) -> i32 {
        let mut x = x;
        let mut z: i32 = 0;
        while x != 0 {
            let last_digit = x % 10;
            x = x / 10;
            z = match z.checked_mul(10_i32) {
                Some(v) => v,
                None => return 0,
            };
            z = match z.checked_add(last_digit) {
                Some(v) => v,
                None => return 0,
            };
        }
        z
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    basic_test_cases(Solution::reverse);
}

#[test]
fn test_solution2() {
    basic_test_cases(Solution::checked_reverse);
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(i32) -> i32,
{
    assert_eq!(func(i32::MIN), 0);
    assert_eq!(func(121), 121);
    assert_eq!(func(-121), -121);
    assert_eq!(func(11), 11);
    assert_eq!(func(100), 1);
    assert_eq!(func(1534236469), 0);
    assert_eq!(func(i32::MAX), 0);
    assert_eq!(func(i32::MAX - 5), 0);
}
