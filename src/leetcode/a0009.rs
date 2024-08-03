/// https://leetcode.com/problems/palindrome-number/
/// Given an integer x, return true if x is a palindrome, and false otherwise.

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x >= 0 && x == Self::reverse(x)
    }

    // Improve: Stop reverse at half the digits and compare
    // Bug: The function gives wrong result when the reversed overflow i32
    fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut z = 0;
        while x > 0 {
            let last_digit = x % 10;
            x = x / 10;
            z = z * 10 + last_digit
        }
        z
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = Solution::is_palindrome;
    assert!(func(121));
    assert!(func(11));
    assert!(func(11211));
    assert!(!func(1212));
    assert!(!func(1212));
}
