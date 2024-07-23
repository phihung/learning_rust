/// https://leetcode.com/problems/trapping-rain-water/description/

// O(N). Top 100% solution
// Trap at i-th position = min(max(height[..i]), max(height[(i+1)..]))
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut highest_on_left = Vec::with_capacity(n);

        // Traverse from left to right and store the highest value on the left for each element
        let mut max_left = 0;
        for &h in height.iter() {
            highest_on_left.push(max_left);
            max_left = max_left.max(h);
        }

        let mut total = 0;

        // right to left
        let mut max_right = 0;
        for (&max_left, &h) in highest_on_left.iter().rev().zip(height.iter().rev()) {
            total += (max_left.min(max_right) - h).max(0);
            max_right = max_right.max(h);
        }

        total
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    basic_test_cases(Solution::trap);
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(Vec<i32>) -> i32,
{
    assert_eq!(func(vec![1, 0, 2]), 1);
    assert_eq!(func(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}
