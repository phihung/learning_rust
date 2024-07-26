// https://leetcode.com/problems/n-queens/description/

// 92%, no recursive backtracking
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut cond_r = vec![true; n];
        let mut cond_c = vec![true; n];
        let mut cond_d_pos = vec![true; 2 * n - 1];
        let mut cond_d_neg = vec![true; 2 * n - 1];

        enum Action {
            Put(usize, usize),
            Remove(usize, usize),
        }

        let mut solutions = vec![];
        let mut current = vec![n; n];
        let mut stack: Vec<Action> = vec![];
        for c in 0..((n as f32 / 2.).ceil() as usize) {
            // Use symmetric
            stack.push(Action::Put(0, c));
        }
        while let Some(action) = stack.pop() {
            match action {
                Action::Put(r, c) => {
                    if !cond_r[r] || !cond_c[c] || !cond_d_pos[r + n - c - 1] || !cond_d_neg[r + c]
                    {
                        continue;
                    }
                    current[r] = c;
                    cond_r[r] = false;
                    cond_c[c] = false;
                    cond_d_pos[r + n - c - 1] = false;
                    cond_d_neg[r + c] = false;

                    stack.push(Action::Remove(r, c));
                    if r < n - 1 {
                        let next_r = r + 1;
                        let n1 = if r == 0 && n % 2 == 1 && current[0] == n / 2 {
                            n / 2
                        } else {
                            n
                        };
                        for next_c in 0..n1 {
                            stack.push(Action::Put(next_r, next_c));
                        }
                    } else {
                        solutions.push(Self::format_output(n, current.iter().copied()));
                        if n > 1 {
                            let symmetric = current.iter().map(|x| (n - 1 - x));
                            solutions.push(Self::format_output(n, symmetric));
                        }
                    }
                }
                Action::Remove(r, c) => {
                    current[r] = n;
                    cond_r[r] = true;
                    cond_c[c] = true;
                    cond_d_pos[r + n - c - 1] = true;
                    cond_d_neg[r + c] = true;
                }
            };
        }

        solutions
    }

    fn format_output(n: usize, solution: impl Iterator<Item = usize>) -> Vec<String> {
        solution
            .map(|col| unsafe {
                String::from_utf8_unchecked(
                    (0..n).map(|j| if j == col { b'Q' } else { b'.' }).collect(),
                )
            })
            .collect()
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    use std::collections::HashSet;
    let test = |n, expect: &[&[&str]]| {
        let actual = Solution::solve_n_queens(n);
        let expect: HashSet<Vec<String>> = expect
            .into_iter()
            .map(|&a| a.into_iter().map(|b| b.to_string()).collect())
            .collect();
        assert_eq!(HashSet::from_iter(actual), expect);
    };
    test(
        4,
        &[
            &[".Q..", "...Q", "Q...", "..Q."],
            &["..Q.", "Q...", "...Q", ".Q.."],
        ],
    );
    assert_eq!(Solution::solve_n_queens(5).len(), 10);
}
