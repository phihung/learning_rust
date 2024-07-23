/// https://leetcode.com/problems/number-of-digit-one

// 0..3123 = 0..1000 + 1000..2000 + 3000..3123
// f(3123) = f(999) + (1000 + f(999)) + f(123)
// f(999) => from 000 to 999 there is 3000 digits, same probability => number of 1s = 3000/ 10 = 300
//    f(10^n - 1) = n * 10^(n-1)
// 1000..2000 => 1000 number 1 + f(999)
//    f(2*10^n - 1) = 10^n + 2*n*10^(n-1)
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut num = n;
        let mut result = 0;

        let mut n_digits: u32 = 0;
        let mut pow = 1;
        while num > 0 {
            let digit = num % 10;
            n_digits += 1;
            if n_digits > 1 {
                result += digit * (n_digits as i32 - 1) * (pow / 10);
            }
            if digit > 1 {
                result += pow;
            } else if digit == 1 {
                result += (n % pow) + 1;
            }
            pow *= 10;
            num = num / 10;
        }
        result
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    assert_eq!(Solution::count_digit_one(0), 0);
    assert_eq!(Solution::count_digit_one(1), 1);
    assert_eq!(Solution::count_digit_one(3), 1);
    assert_eq!(Solution::count_digit_one(13), 6);
    assert_eq!(Solution::count_digit_one(99), 20);
    assert_eq!(Solution::count_digit_one(132), 67);
    assert_eq!(Solution::count_digit_one(2014), 1607);
    assert_eq!(Solution::count_digit_one(824883294), 767944060);
}
