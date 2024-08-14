// https://leetcode.com/problems/find-k-th-smallest-pair-distance

impl Solution {
    // Binary search: O(nlogM + nlogn)
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let (mut lo, mut hi) = (0, nums[nums.len() - 1] - nums[0] + 1);
        while lo < hi {
            let v = (lo + hi) / 2;
            if Self::count_pair(&nums, v) < k {
                lo = v + 1;
            } else {
                hi = v;
            }
        }
        lo
    }

    // Count A = {(i, j) | i < j, nums[j] - nums[i] <= upper }
    fn count_pair(nums: &[i32], upper: i32) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut cnt = 0;
        while r < nums.len() {
            let delta = nums[r] - nums[l];
            if delta <= upper {
                cnt += (r - l) as i32;
                r += 1;
            } else {
                l += 1;
                if l == r {
                    r += 1;
                }
            }
        }
        cnt
    }

    // Bucket sort: O(N^2)
    pub fn smallest_distance_pair2(nums: Vec<i32>, k: i32) -> i32 {
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        let mut count = vec![0; (1 + max - min) as usize];
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                count[(nums[i] - nums[j]).abs() as usize] += 1;
            }
        }
        let mut i = 0;
        for (v, cnt) in count.into_iter().enumerate() {
            i += cnt;
            if i >= k {
                return v as i32;
            }
        }
        unreachable!()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |x: &[i32], k: i32| Solution::smallest_distance_pair(x.to_vec(), k);
        run_test(func);
    }
    #[test]
    fn test_solution2() {
        let func = |x: &[i32], k: i32| Solution::smallest_distance_pair2(x.to_vec(), k);
        run_test(func);
    }

    #[test]
    fn test_count_pair() {
        assert_eq!(Solution::count_pair(&[1, 3, 4, 5], 1), 2);
        assert_eq!(Solution::count_pair(&[1, 3, 4, 5], 2), 4);
        assert_eq!(Solution::count_pair(&[1, 2, 3, 5], 2), 4);
        assert_eq!(Solution::count_pair(&[1, 2, 3, 4, 6], 3), 8);
        assert_eq!(Solution::count_pair(&[1, 2, 3, 8], 3), 3);
    }

    fn run_test(func: impl Fn(&[i32], i32) -> i32) {
        assert_eq!(func(&[1, 3, 1], 1), 0);
        assert_eq!(func(&[1, 3, 1], 2), 2);
        assert_eq!(func(&[1, 3, 1], 3), 2);
        assert_eq!(func(&[1, 1, 1], 2), 0);
        assert_eq!(func(&[1, 2], 1), 1);
    }
}
