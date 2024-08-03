// https://leetcode.com/problems/increasing-triplet-subsequence

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut min, mut min2) = (i32::MAX, i32::MAX);
        for x in nums {
            if x <= min {
                min = x;
            } else if x <= min2 {
                min2 = x;
            } else {
                return true;
            }
        }
        false
    }

    // O(N) speed, O(1) memory
    pub fn increasing_triplet2(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut min = i32::MAX;
        for i in 0..n {
            if nums[i] <= min {
                min = nums[i];
                nums[i] = i32::MIN;
            }
        }

        let mut max = nums[n - 1];
        for x in nums.into_iter().rev().skip(1) {
            if x == i32::MIN {
                continue;
            }
            if x < max {
                return true;
            }
            max = x;
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        let func = |nums: &[i32]| Solution::increasing_triplet(nums.to_owned());
        run_test(func)
    }

    #[test]
    fn test_solution_2() {
        let func = |nums: &[i32]| Solution::increasing_triplet2(nums.to_owned());
        run_test(func)
    }

    fn run_test(func: impl Fn(&[i32]) -> bool) {
        assert_eq!(func(&[1, 1, 0, 1, 1, 0, -1, 1, 1, 0]), false);

        assert_eq!(func(&[20, 100, 10, 12, 5, 13]), true);
        assert_eq!(func(&[10, 9, 7, 6, 12, 10, 9, 7, 5, 3]), false);
        assert_eq!(func(&[1, 2, 3, 4, 5]), true);
        assert_eq!(func(&[5, 4, 3, 2, 1]), false);
        assert_eq!(func(&[2, 1, 5, 0, 4, 6]), true);
    }
}
