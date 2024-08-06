// https://leetcode.com/problems/patching-array

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut count = 0;
        let mut max_cover: u32 = 1; // right exclusive
        let mut nums = nums.into_iter().filter(|&x| x <= n);
        while max_cover <= n as u32 {
            if let Some(x) = nums.next() {
                while max_cover < x as u32 {
                    max_cover += max_cover;
                    count += 1;
                }
                max_cover += x as u32;
                continue;
            }

            max_cover += max_cover;
            count += 1;
        }
        count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let func = |nums: &[i32], n: i32| Solution::min_patches(nums.to_vec(), n);

        assert_eq!(func(&[1, 3], 6), 1);
        assert_eq!(func(&[1, 5, 10], 4), 2);
        assert_eq!(func(&[1, 5, 10], 5), 2);
        assert_eq!(func(&[1, 5, 10], 6), 2);
        assert_eq!(func(&[1, 5, 10], 20), 2);
        assert_eq!(func(&[1, 2, 2], 5), 0);
        assert_eq!(func(&[1, 2, 2], 30), 3);
        assert_eq!(func(&[1, 2, 31, 33], 2147483647), 28);

        assert_eq!(func(&[5, 10], 4), 3);
        let a = [
            1, 2, 2, 6, 34, 38, 41, 44, 47, 47, 56, 59, 62, 73, 77, 83, 87, 89, 94,
        ];
        assert_eq!(func(&a, 20), 1);
        assert_eq!(func(&a, 6), 0);
        assert_eq!(func(&a, 34), 2);
    }
}
