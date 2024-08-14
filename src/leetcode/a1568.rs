// https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island

impl Solution {
    // Brut force
    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if !Self::check_connectivity(&grid) {
            return 0;
        }
        let count = grid.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>();
        if count <= 2 {
            return count;
        }
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    grid[r][c] = 0;
                    if !Self::check_connectivity(&grid) {
                        return 1;
                    }
                    grid[r][c] = 1;
                }
            }
        }
        2
    }

    fn check_connectivity(grid: &Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dsu = DSU::new(m * n);
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 0 {
                    continue;
                }
                dsu.make_set(r * n + c);
                if r > 0 && grid[r - 1][c] > 0 {
                    dsu.union_sets((r - 1) * n + c, r * n + c);
                }
                if c > 0 && grid[r][c - 1] > 0 {
                    dsu.union_sets(r * n + c - 1, r * n + c);
                }
            }
        }
        dsu.count() < 2
    }
}

#[derive(Clone, Debug)]
pub struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![usize::MAX; n],
        }
    }

    pub fn count(&self) -> usize {
        self.parent
            .iter()
            .enumerate()
            .map(|(i, &x)| if i == x { 1 } else { 0 })
            .sum::<usize>()
    }

    pub fn make_set(&mut self, v: usize) {
        self.parent[v] = v;
    }

    pub fn find_set(&mut self, v: usize) -> usize {
        if v == self.parent[v] {
            v
        } else {
            self.parent[v] = self.find_set(self.parent[v]);
            self.parent[v]
        }
    }

    pub fn union_sets(&mut self, a: usize, b: usize) -> bool {
        let a = self.find_set(a);
        let b = self.find_set(b);
        if a != b {
            self.parent[b] = a;
            true
        } else {
            false
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |grid: &[&[i32]]| Solution::min_days(grid.iter().map(|x| x.to_vec()).collect());
        assert_eq!(
            func(&[
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 0, 1, 1]
            ]),
            2
        );
        assert_eq!(
            func(&[
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 0, 1, 1],
                &[1, 1, 0, 1, 1]
            ]),
            1
        );

        assert_eq!(func(&[&[0, 1, 1], &[1, 1, 1], &[1, 1, 0]]), 1);
        assert_eq!(func(&[&[0, 1, 1, 0], &[0, 1, 1, 0], &[0, 0, 0, 0]]), 2);
        assert_eq!(func(&[&[1, 1]]), 2);
        assert_eq!(func(&[&[0, 1, 0]]), 1);
        assert_eq!(func(&[&[1, 1, 1]]), 1);
        assert_eq!(func(&[&[0, 1, 1, 1, 0]]), 1);
        assert_eq!(func(&[&[0, 1, 0, 1, 0]]), 0);
        assert_eq!(func(&[&[0, 1, 1, 0, 1, 0]]), 0);
    }
}
