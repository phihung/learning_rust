// https://leetcode.com/problems/spiral-matrix-iii/

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut out = Vec::with_capacity((rows * cols) as usize);
        let n_round = 1 + r_start.max(rows - r_start).max(c_start).max(cols - c_start);

        for round in 0..n_round {
            // Go east
            let r = r_start - round;
            if r >= 0 {
                for c in (c_start - round).max(0)..(c_start + 1 + round).min(cols) {
                    out.push(vec![r, c]);
                }
            }

            // Go south
            let c = c_start + 1 + round;
            if c < cols {
                for r in (r_start - round).max(0)..(r_start + 1 + round).min(rows) {
                    out.push(vec![r, c]);
                }
            }

            // Go west
            let r = r_start + round + 1;
            if r < rows {
                for c in ((c_start - round).max(0)..(c_start + 2 + round).min(cols)).rev() {
                    out.push(vec![r, c]);
                }
            }

            // Go north
            let c = c_start - round - 1;
            if c >= 0 {
                for r in ((r_start - round).max(0)..(r_start + 2 + round).min(rows)).rev() {
                    out.push(vec![r, c]);
                }
            }
        }
        out
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_solution() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 1, 0, 0),
            [[0, 0]]
        );

        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            [[0, 0], [0, 1], [0, 2], [0, 3]]
        );

        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 1),
            [[0, 1], [0, 2], [0, 0], [0, 3]]
        );

        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 3),
            [[0, 3], [0, 2], [0, 1], [0, 0]]
        );

        assert_eq!(
            Solution::spiral_matrix_iii(4, 2, 0, 0),
            [[0, 0], [0, 1], [1, 1], [1, 0], [2, 1], [2, 0], [3, 1], [3, 0]]
        );
        
        assert_eq!(
            Solution::spiral_matrix_iii(4, 2, 0, 1),
            [[0, 1], [1, 1], [1, 0], [0, 0], [2, 1], [2, 0], [3, 1], [3, 0]]
        );

        assert_eq!(
            Solution::spiral_matrix_iii(4, 2, 3, 1),
            [[3, 1], [3, 0], [2, 0], [2, 1], [1, 0], [1, 1], [0, 0], [0, 1]]
        );

        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            [[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,1],[4,0],[3,0],[2,0],[1,0],[0,0]]
        );
    }
}
