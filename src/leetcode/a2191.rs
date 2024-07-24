// https://leetcode.com/problems/sort-the-jumbled-numbers

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        let mapping: &[i32; 10] = mapping.as_slice().try_into().unwrap();
        nums.sort_by_cached_key(|&x| Self::map(&mapping, x));
        nums
    }

    fn map(mapping: &[i32; 10], num: i32) -> i32 {
        let mut num = num;
        let mut out = 0;
        let mut unit = 1;
        loop {
            out += unit * mapping[(num % 10) as usize];
            num /= 10;
            unit *= 10;
            if num == 0 {
                break;
            }
        }
        out
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let func = |m: &[i32], nums: &[i32]| Solution::sort_jumbled(m.to_vec(), nums.to_vec());
    assert_eq!(
        func(&[8, 9, 4, 0, 2, 1, 3, 5, 7, 6], &[991, 338, 38]),
        &[338, 38, 991]
    );
    assert_eq!(
        func(&[8, 9, 4, 0, 2, 1, 3, 5, 7, 6], &[0, 991, 338, 38]),
        &[338, 38, 0, 991]
    );
    assert_eq!(
        func(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &[123, 456, 789]),
        &[123, 456, 789]
    );
}
