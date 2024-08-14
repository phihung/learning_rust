// https://leetcode.com/problems/combination-sum-ii

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut counts = [0; 31];
        for x in candidates {
            if x <= target {
                counts[x as usize] += 1;
            }
        }

        fn backtracking(
            counts: &[i32],
            mut target: i32,
            idx: usize,
            current: &mut Vec<i32>,
            solutions: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 {
                solutions.push(current.clone());
                return;
            }
            if idx >= counts.len() {
                return;
            }
            let v = idx as i32;
            if target >= v + 1 {
                backtracking(counts, target, idx + 1, current, solutions);
            }
            let old_len = current.len();
            for _ in 0..counts[idx] {
                if target < v {
                    break;
                }
                current.push(v);
                backtracking(counts, target - v, idx + 1, current, solutions);
                target -= v;
            }
            current.truncate(old_len);
        }

        let mut solutions = vec![];
        backtracking(&counts, target, 1, &mut vec![], &mut solutions);
        solutions
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |c: &[i32], t| HashSet::from_iter(Solution::combination_sum(c.to_vec(), t));
        assert_eq!(
            func(&[2, 5, 2, 1, 2], 5),
            HashSet::from([vec![1, 2, 2], vec![5]])
        );
        assert_eq!(
            func(&[10, 1, 2, 7, 6, 1, 5], 8),
            HashSet::from([vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]])
        );
    }
}
