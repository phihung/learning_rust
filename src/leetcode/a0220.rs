// https://leetcode.com/problems/contains-duplicate-iii/description/

use std::collections::BTreeSet;

impl Solution {
    // O(N)
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let (min, max) = nums.iter().fold((i32::MAX, i32::MIN), |(min, max), &num| {
            (min.min(num), max.max(num))
        });

        let (arr, diff0, diff1, max1) =
            if (max - min) / (value_diff + 1) < nums.len() as i32 / (index_diff + 1) {
                let arr = nums
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| (i as i32, x - min))
                    .collect();
                (arr, index_diff, value_diff, max - min)
            } else {
                // It is more efficient to solve the dual problem: Invert the role of index and value
                let mut arr = nums
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| (x, i as i32))
                    .collect::<Vec<_>>();
                arr.sort_unstable();
                let max1 = arr.len() as i32;
                (arr, value_diff, index_diff, max1)
            };
        let bucket_size = diff1 + 1;
        let n_buckets = (max1 / bucket_size) as usize + 3;
        let mut buckets = vec![-diff1 - 1; n_buckets];
        let mut i_prev = 0;
        for &(v0, v1) in &arr {
            while v0 - arr[i_prev].0 > diff0 {
                buckets[(arr[i_prev].1 / bucket_size + 1) as usize] = -diff1 - 1;
                i_prev += 1;
            }

            let idx = 1 + (v1 / bucket_size) as usize;
            if (v1 - buckets[idx]).abs() <= diff1
                || (v1 - buckets[idx - 1]).abs() <= diff1
                || (v1 - buckets[idx + 1]).abs() <= diff1
            {
                return true;
            }
            buckets[idx] = v1;
        }
        false
    }

    // O(NlogN)
    pub fn contains_nearby_almost_duplicate2(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let index_diff = index_diff as usize;
        let mut tree = BTreeSet::new();
        for i in 0..nums.len() {
            let mut range = tree.range((nums[i] - value_diff)..);
            if let Some(&v) = range.next() {
                if v <= nums[i] + value_diff {
                    return true;
                }
            }
            if i >= index_diff {
                tree.remove(&nums[i - index_diff]);
            }
            tree.insert(nums[i]);
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        run_test(|nums, d1, d2| Solution::contains_nearby_almost_duplicate(nums.to_vec(), d1, d2))
    }

    #[test]
    fn test_solution2() {
        run_test(|nums, d1, d2| Solution::contains_nearby_almost_duplicate2(nums.to_vec(), d1, d2))
    }

    fn run_test(func: impl Fn(&[i32], i32, i32) -> bool) {
        assert_eq!(func(&[10, 100, 11, 9], 1, 2), true);
        assert_eq!(func(&[1, 2, 1, 1], 1, 0), true);
        assert_eq!(func(&[1, 2, 3, 4], 4, 0), false);
        assert_eq!(func(&[1, 2, 3, 1], 3, 0), true);
        assert_eq!(func(&[1, 5, 9, 1, 5, 9], 2, 3), false);
    }
}
