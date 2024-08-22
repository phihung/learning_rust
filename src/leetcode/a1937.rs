// https://leetcode.com/problems/maximum-number-of-points-with-cost/description/

impl Solution {
    // O(M x N)
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        // dp[i] = a_i + max_k (dp[k] - |k - i|)
        //       = a_i + max(max_{k<i} (dp[k] + k) - i, max_{k>=i} (dp[k] - k) + i)

        let n = points[0].len();
        let mut dp = vec![0_i64; n];
        let mut right_max = vec![0; n + 1];
        for row in points {
            right_max[n] = i64::MIN;
            for i in (0..n).rev() {
                right_max[i] = right_max[i + 1].max(dp[i] - i as i64);
            }
            let mut left_max = i64::MIN;
            for i in 0..n {
                left_max = left_max.max(dp[i] + i as i64);
                dp[i] = (left_max - i as i64).max(right_max[i] + i as i64) + row[i] as i64;
            }
        }
        *dp.iter().max().unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func =
            |nums: &[&[i32]]| Solution::max_points(nums.iter().map(|x| x.to_vec()).collect());
        assert_eq!(func(&[&[1, 2, 3], &[1, 5, 1], &[3, 1, 1]]), 9);
        assert_eq!(func(&[&[1, 5], &[2, 3], &[4, 2]]), 11);
        assert_eq!(func(&[&[1]]), 1);
        assert_eq!(
            func(&[
                &[100000, 90000, 80000, 70000, 60000, 50000],
                &[1, 2, 3, 4, 5, 6],
                &[100000, 90000, 80000, 70000, 60000, 50000],
                &[5, 10, 15, 20, 25, 30],
                &[100000, 90000, 80000, 70000, 60000, 50000]
            ]),
            300021
        );
        assert_eq!(
            func(&[
                &[0, 0, 4, 1, 4],
                &[2, 1, 2, 0, 1],
                &[2, 2, 1, 3, 4],
                &[5, 2, 4, 5, 4],
                &[0, 5, 4, 2, 5]
            ]),
            18
        );
    }
}
