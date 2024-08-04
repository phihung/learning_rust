// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/description

impl Solution {
    // 0ms, O(n^2)
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let total = nums.iter().sum::<i32>() as usize;

        // Sum counts
        let mut counts: Vec<i32> = vec![0; 1 + total];

        // Compute subarray sums
        let mut a = vec![0; n as usize];
        for (i, x) in nums.into_iter().enumerate() {
            for j in 0..i {
                a[j] += x;
                counts[a[j] as usize] += 1;
            }
            a[i] = x;
            counts[x as usize] += 1;
        }

        let mut sum = 0;
        let mut cnt = 0;
        let mut left_found = false;
        let modulo = 1_000_000_007;
        for (i, x) in counts.into_iter().enumerate() {
            if x == 0 {
                continue;
            }
            cnt += x;
            if !left_found && cnt >= left {
                sum = (cnt - left + 1) * i as i32;
                left_found = true;
            } else if left_found {
                sum = (sum + x * i as i32) % modulo;
            }
            if cnt >= right {
                sum = sum - (cnt - right) * i as i32;
                break;
            }
        }
        if sum >= 0 {
            sum
        } else {
            modulo - sum
        }
    }

    pub fn range_sum2(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let (n, left, right) = (n as usize, left as usize, right as usize);
        let mut sums = Vec::with_capacity(n * (n + 1) / 2);
        for (i, x) in nums.into_iter().enumerate() {
            sums.push(x);
            for j in (sums.len() - i - 1)..(sums.len() - 1) {
                sums.push(x + sums[j]);
            }
        }
        sums.sort_unstable();
        let modulo = 1_000_000_007;
        sums[(left - 1)..right]
            .into_iter()
            .copied()
            .reduce(|acc, x| (acc + x) % modulo)
            .unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(|nums: &[i32], n: i32, left: i32, right: i32| {
            Solution::range_sum(nums.to_vec(), n, left, right)
        })
    }

    #[test]
    fn test_solution2() {
        run_test(|nums: &[i32], n: i32, left: i32, right: i32| {
            Solution::range_sum2(nums.to_vec(), n, left, right)
        })
    }

    fn run_test(func: impl Fn(&[i32], i32, i32, i32) -> i32) {
        assert_eq!(func(&[1, 2, 3, 4], 4, 1, 5), 13);
        assert_eq!(func(&[1, 2, 3, 4], 4, 3, 4), 6);
        assert_eq!(func(&[1, 2, 3, 4], 4, 1, 10), 50);
        assert_eq!(func(&[100; 1000], 1000, 1, 500500), 716699888);
    }
}
