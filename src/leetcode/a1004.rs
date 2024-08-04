// https://leetcode.com/problems/max-consecutive-ones-iii

use std::collections::VecDeque;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), k as usize);
        let mut pos0 = VecDeque::with_capacity(k + 2);
        pos0.push_back(-1);
        let mut result = 0;
        for (r, c) in nums.into_iter().enumerate() {
            if c == 0 {
                pos0.push_back(r as i32);
                if pos0.len() == k + 2 {
                    let l = pos0.pop_front().unwrap();
                    result = result.max(r as i32 - l - 1);
                }
            }
        }
        result = result.max(n as i32 - 1 - pos0.pop_front().unwrap());
        result
    }

    pub fn longest_ones2(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut result = 0;
        let mut l: i32 = -1;
        for (r, &x) in nums.iter().enumerate() {
            if x == 0 {
                k -= 1;
                if k < 0 {
                    result = result.max(r as i32 - l - 1);
                    l += 1;
                    while l < nums.len() as i32 && nums[l as usize] != 0 {
                        l += 1;
                    }
                }
            }
        }
        result = result.max(nums.len() as i32 - l - 1);
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(|nums: &[i32], k: i32| Solution::longest_ones(nums.to_vec(), k))
    }

    #[test]
    fn test_solution2() {
        run_test(|nums: &[i32], k: i32| Solution::longest_ones2(nums.to_vec(), k))
    }

    fn run_test(func: impl Fn(&[i32], i32) -> i32) {
        assert_eq!(func(&[1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
        assert_eq!(
            func(
                &[0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
