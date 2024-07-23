/// https://leetcode.com/problems/two-sum/description/
/// Leetcode 01: Find the indexes of two numbers in the vector that sum up to the given target number
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, usize> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            if let Some(&j) = m.get(&(target - v)) {
                return vec![j as i32, i as i32];
            }
            m.insert(v, i);
        }
        vec![]
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_two_sum() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
