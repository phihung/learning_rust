/// https://leetcode.com/problems/container-with-most-water/

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_area = 0;

        while left < right {
            let (left_height, right_height) = (height[left], height[right]);
            let width = (right - left) as i32;
            let current_area = width * left_height.min(right_height);
            max_area = max_area.max(current_area);

            if left_height < right_height {
                loop {
                    left += 1;
                    if left >= right || height[left] > left_height {
                        break;
                    }
                }
            } else {
                loop {
                    right -= 1;
                    if left >= right || height[right] > right_height {
                        break;
                    }
                }
            }
        }

        max_area
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    basic_test_cases(Solution::max_area);
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(Vec<i32>) -> i32,
{
    assert_eq!(func(vec![1, 1]), 1);
    assert_eq!(func(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
