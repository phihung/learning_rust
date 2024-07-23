// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums

// 100%
impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let (n_rows, n_cols) = (row_sum.len(), col_sum.len());
        let mut m = vec![vec![0; n_cols]; n_rows];
        let (mut i, mut j) = (0, 0);
        let (mut used_row, mut used_col) = (0, 0);
        while i < n_rows && j < n_cols {
            let (a, b) = (row_sum[i] - used_row, col_sum[j] - used_col);
            if a < b {
                m[i][j] = a;
                i += 1;
                // If col_num is mutable, we would do: col_num[j] -= a;
                // Instead, we need to keep in mind that col_num[j] has been partially used
                (used_row, used_col) = (0, used_col + a);
            } else {
                m[i][j] = b;
                j += 1;
                (used_row, used_col) = (used_row + b, 0);
            }
        }
        m
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let test = |rs: &[i32], cs: &[i32]| {
        let m = Solution::restore_matrix(rs.to_vec(), cs.to_vec());
        println!("{:?}", m);
        assert_eq!(m.len(), rs.len());
        for (row, &sum) in m.iter().zip(rs) {
            assert!(row.iter().all(|&x| x >= 0));
            assert_eq!(row.iter().sum::<i32>(), sum);
            assert_eq!(row.len(), cs.len());
        }
        cs.iter().enumerate().for_each(|(j, &sum)| {
            assert_eq!(m.iter().map(|r| r[j]).sum::<i32>(), sum);
        })
    };
    // test(&[3, 8], &[4, 7]);
    // test(&[3], &[3]);
    // test(&[6], &[3, 1, 2]);
    // test(&[3, 1, 2], &[6]);
    // test(&[0, 0, 0], &[0]);
    test(&[4, 12, 10, 1, 0], &[1, 0, 3, 16, 7]);
}
