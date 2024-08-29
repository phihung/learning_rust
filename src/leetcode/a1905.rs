impl Solution {
    // DFS
    pub fn count_sub_islands(g1: Vec<Vec<i32>>, mut g2: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (g1.len(), g1[0].len());
        let mut cnt = 0;
        for i in 0..m {
            for j in 0..n {
                if g2[i][j] == 1 && g1[i][j] != 0 {
                    let mut is_sub_island = true;
                    Self::dfs(&mut g2, i, j, &mut |i1: usize, j1: usize| {
                        is_sub_island &= g1[i1][j1] != 0;
                    });
                    cnt += is_sub_island as i32
                }
            }
        }
        cnt
    }

    fn dfs<F>(g: &mut Vec<Vec<i32>>, i: usize, j: usize, func: &mut F)
    where
        F: FnMut(usize, usize),
    {
        if g[i][j] != 1 {
            return;
        }
        g[i][j] = 2;
        func(i, j);
        let (m, n) = (g.len(), g[0].len());
        if i > 0 {
            Self::dfs(g, i - 1, j, func);
        }
        if i < m - 1 {
            Self::dfs(g, i + 1, j, func);
        }
        if j > 0 {
            Self::dfs(g, i, j - 1, func);
        }
        if j < n - 1 {
            Self::dfs(g, i, j + 1, func);
        }
    }

    pub fn count_sub_islands2(g1: Vec<Vec<i32>>, g2: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (g1.len(), g1[0].len());
        let mut dsu2 = Self::compute_dsu(&g2);

        let mut mapping = vec![0_u8; m * n];
        for i in 0..m {
            for j in 0..n {
                if g2[i][j] == 0 {
                    continue;
                }
                let c2 = dsu2.find_set(i * n + j);

                mapping[c2] = 3 - mapping[c2] - g1[i][j] as u8;
                if g1[i][j] == 0 {
                    mapping[c2] = 1;
                } else if mapping[c2] == 0 {
                    // new g2 component
                    mapping[c2] = 2;
                }
            }
        }
        mapping.into_iter().map(|x| (x == 2) as i32).sum()
    }

    fn compute_dsu(g: &Vec<Vec<i32>>) -> DSU {
        let (m, n) = (g.len(), g[0].len());
        let mut dsu = DSU::new(m * n);
        for i in 0..m {
            for j in 0..n {
                if g[i][j] == 0 {
                    continue;
                }
                dsu.make_set(i * n + j);
                if i > 0 && g[i - 1][j] == 1 {
                    dsu.union_sets((i - 1) * n + j, i * n + j);
                }
                if j > 0 && g[i][j - 1] == 1 {
                    dsu.union_sets(i * n + (j - 1), i * n + j);
                }
            }
        }
        dsu
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
    use crate::to_owned;

    use super::*;

    #[test]
    fn test_solution() {
        let func =
            |a: &[&[i32]], b: &[&[i32]]| Solution::count_sub_islands(to_owned!(a), to_owned!(b));
        run_test(func);
    }

    #[test]
    fn test_solution2() {
        let func =
            |a: &[&[i32]], b: &[&[i32]]| Solution::count_sub_islands(to_owned!(a), to_owned!(b));
        run_test(func);
    }

    fn run_test(func: impl Fn(&[&[i32]], &[&[i32]]) -> i32) {
        assert_eq!(
            func(
                &[
                    &[1, 1, 1, 0, 0],
                    &[0, 1, 1, 1, 1],
                    &[0, 0, 0, 0, 0],
                    &[1, 0, 0, 0, 0],
                    &[1, 1, 0, 1, 1]
                ],
                &[
                    &[1, 1, 1, 0, 0],
                    &[0, 0, 1, 1, 1],
                    &[0, 1, 0, 0, 0],
                    &[1, 0, 1, 1, 0],
                    &[0, 1, 0, 1, 0]
                ]
            ),
            3
        );

        assert_eq!(
            func(
                &[
                    &[1, 0, 1, 0, 1],
                    &[1, 1, 1, 1, 1],
                    &[0, 0, 0, 0, 0],
                    &[1, 1, 1, 1, 1],
                    &[1, 0, 1, 0, 1]
                ],
                &[
                    &[0, 0, 0, 0, 0],
                    &[1, 1, 1, 1, 1],
                    &[0, 1, 0, 1, 0],
                    &[0, 1, 0, 1, 0],
                    &[1, 0, 0, 0, 1]
                ]
            ),
            2
        );

        assert_eq!(
            func(
                &[&[1, 0, 1, 0, 1], &[1, 1, 1, 1, 1],],
                &[&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1],]
            ),
            1
        );

        assert_eq!(
            func(
                &[&[1, 0, 1, 0, 1], &[1, 1, 1, 1, 1],],
                &[&[0, 0, 0, 1, 0], &[1, 1, 1, 0, 1],]
            ),
            2
        );

        assert_eq!(
            func(
                &[&[1, 0, 1, 0, 1], &[1, 1, 1, 1, 1],],
                &[&[0, 0, 0, 1, 0], &[1, 1, 1, 1, 1],]
            ),
            0
        );
    }
}
