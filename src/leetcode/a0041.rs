/// https://leetcode.com/problems/first-missing-positive/
/// Given an unsorted integer array nums. Return the smallest positive integer that is not present in nums.

// Approach: Iteratively swapping each value v to index (v - 1)
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        for i in 0..n {
            loop {
                let v = nums[i] as usize;
                if v == i + 1 {
                    break;
                }
                if v <= 0 || v > n || nums[v - 1] == v as i32 {
                    nums[i] = 0;
                    break;
                }
                nums.swap(v - 1, i);
            }
        }
        for (i, &v) in nums.iter().enumerate() {
            if v == 0 {
                return i as i32 + 1;
            }
        }
        return n as i32 + 1;
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    assert_eq!(Solution::first_missing_positive(vec![2, 2, 3]), 1);
    assert_eq!(Solution::first_missing_positive(vec![2, 3, 1]), 4);
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}
