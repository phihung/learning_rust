/// https://leetcode.com/problems/lucky-numbers-in-a-matrix/description/

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        matrix
            .iter()
            .map(|row| (row.iter().enumerate().map(|(i, v)| (*v, i)).min().unwrap()))
            .max()
            .map(|(v, col)| {
                if matrix.iter().any(|row| row[col] > v) {
                    vec![]
                } else {
                    vec![v]
                }
            })
            .unwrap()
    }
}

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |m: &[&[i32]]| Solution::lucky_numbers(m.iter().map(|x| x.to_vec()).collect());
    assert_eq!(func(&[&[3, 7, 8], &[9, 11, 13], &[15, 16, 17]]), &[15]);
    assert_eq!(func(&[&[7, 8], &[1, 2]]), &[7]);
    assert_eq!(
        func(&[&[1, 10, 4, 2], &[9, 3, 8, 7], &[15, 16, 17, 12]]),
        &[12]
    );
}
