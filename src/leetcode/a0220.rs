// https://leetcode.com/problems/contains-duplicate-iii/description/

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        _nums: Vec<i32>,
        _index_diff: i32,
        _value_diff: i32,
    ) -> bool {
        false
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let func = |nums: &[i32], idiff, vdiff| {
        Solution::contains_nearby_almost_duplicate(nums.to_vec(), idiff, vdiff)
    };
    assert_eq!(func(&[1, 2, 3, 1], 3, 0), true);
    assert_eq!(func(&[1, 5, 9, 1, 5, 9], 2, 3), false);
}
