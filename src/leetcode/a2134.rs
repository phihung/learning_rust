// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let n1 = nums.iter().sum::<i32>();
        let mut cnt = nums[..(n1 as usize)].iter().sum::<i32>();
        let mut best = n1 - cnt;
        for i in 0..n {
            // Number of 1 in range(i+1)..(i+n1)
            cnt = cnt + nums[(i + n1 as usize) % n] - nums[i];
            // Cost of moving all 0 out
            best = best.min(n1 - cnt);
        }
        best
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(|nums: &[i32]| Solution::min_swaps(nums.to_owned()));
    }

    fn run_test(func: impl Fn(&[i32]) -> i32) {
        assert_eq!(func(&[0, 1, 1, 1, 0, 1, 1, 0, 1]), 1, "test 1");

        assert_eq!(func(&[0, 1, 1, 1, 0, 1, 1, 0, 1]), 1, "test 1");
        assert_eq!(func(&[0, 1, 1, 0, 1, 0, 1, 1, 0, 1]), 2, "test 2");
        assert_eq!(func(&[0, 1, 0, 1, 0, 1, 0, 1]), 2, "test 3");
        assert_eq!(func(&[0, 1, 0, 1, 1, 0, 0]), 1, "test 4");
        assert_eq!(func(&[0, 1, 1, 1, 0, 0, 1, 1, 0]), 2, "test 5");
        assert_eq!(func(&[1, 1, 0, 0, 1]), 0, "test 6");
    }
}
