// https://leetcode.com/problems/magic-squares-in-grid/description

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        if rows < 3 || cols < 3 {
            return 0;
        }
        let dirs = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];
        let mut count = 0;
        for r in 0..(rows - 2) {
            'a: for c in 0..(cols - 2) {
                for dir in dirs {
                    let s: i32 = dir.iter().map(|&(i, j)| grid[r + i][c + j]).sum();
                    if s != 15 {
                        continue 'a;
                    }
                }
                for i in 0..2 {
                    for j in 0..2 {
                        if grid[r + i][c + j] > 9 || grid[r + i][c + j] == 0 {
                            continue 'a;
                        }
                    }
                }
                let mut v = vec![false; 9];
                for i in 0..2 {
                    for j in 0..2 {
                        if v[grid[r + i][c + j] as usize - 1] {
                            continue 'a;
                        }
                        v[grid[r + i][c + j] as usize - 1] = true;
                    }
                }
                count += 1;
            }
        }
        count
    }

    pub fn num_magic_squares_inside2(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        if rows < 3 || cols < 3 {
            return 0;
        }
        let mut masks = vec![vec![true; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] > 9 || grid[r][c] == 0 {
                    for i in 0..2 {
                        for j in 0..2 {
                            if r >= i && c >= j {
                                masks[r - i][c - j] = false;
                            }
                        }
                    }
                }
            }
        }
        for r in 0..rows {
            for c in 2..cols {
                if grid[r][c] + grid[r][c - 1] + grid[r][c - 2] != 15 {
                    for i in 0..2 {
                        if r >= i {
                            masks[r - i][c - 2] = false;
                        }
                    }
                }
            }
        }
        for c in 0..cols {
            for r in 2..rows {
                if grid[r - 2][c] + grid[r - 1][c] + grid[r][c] != 15 {
                    for j in 0..2 {
                        if c >= j {
                            masks[r - 2][c - j] = false;
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for r in 0..(rows - 2) {
            'a: for c in 0..(cols - 2) {
                if masks[r][c]
                    && grid[r][c] + grid[r + 1][c + 1] + grid[r + 2][c + 2] == 15
                    && grid[r + 2][c] + grid[r + 1][c + 1] + grid[r][c + 2] == 15
                {
                    let mut v = vec![false; 9];
                    for i in 0..2 {
                        for j in 0..2 {
                            if v[grid[r + i][c + j] as usize - 1] {
                                continue 'a;
                            }
                            v[grid[r + i][c + j] as usize - 1] = true;
                        }
                    }
                    count += 1;
                }
            }
        }
        count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |x: &[&[i32]]| {
            Solution::num_magic_squares_inside(x.iter().map(|x| x.to_vec()).collect())
        };
        run_test(func);
    }

    #[test]
    fn test_solution2() {
        let func = |x: &[&[i32]]| {
            Solution::num_magic_squares_inside2(x.iter().map(|x| x.to_vec()).collect())
        };
        run_test(func);
    }

    fn run_test(func: impl Fn(&[&[i32]]) -> i32) {
        assert_eq!(func(&[&[2, 7, 6], &[1, 5, 9], &[4, 3, 8]]), 0);
        assert_eq!(func(&[&[5, 5, 5], &[5, 5, 5], &[5, 5, 5]]), 0);
        assert_eq!(func(&[&[4, 3, 8, 4], &[9, 5, 1, 9], &[2, 7, 6, 2]]), 1);
        assert_eq!(func(&[&[8]]), 0);
    }
}
