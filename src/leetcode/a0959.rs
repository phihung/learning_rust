// https://leetcode.com/problems/regions-cut-by-slashes/description

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n_connected_components = Self::count_connected_components(&grid) as i32;

        let n = grid.len();
        let node_count = ((n + 1) * (n + 1)) as i32;
        let edge_count = 4 * n as i32 // Borders
            + grid
                .into_iter()
                .map(|row| {
                    row.as_bytes()
                        .into_iter()
                        .map(|&x| if x != b' ' { 1 } else { 0 })
                        .sum::<i32>()
                })
                .sum::<i32>();
        edge_count - node_count + n_connected_components
    }

    fn count_connected_components(grid: &Vec<String>) -> usize {
        let n = grid.len();
        let mut dsu = DSU::new((n + 1) * (n + 1));

        for i in 0..(n + 1) * (n + 1) {
            dsu.make_set(i);
        }
        for i in 0..n {
            dsu.union_sets(i, i + 1); // Top border
            dsu.union_sets(n * (n + 1) + i, n * (n + 1) + i + 1); // Down border
            dsu.union_sets(i * (n + 1), (i + 1) * (n + 1)); // Left border
            dsu.union_sets(i * (n + 1) + n, (i + 1) * (n + 1) + n); // Right border
        }
        for r in 0..n {
            let row = grid[r].as_bytes();
            for c in 0..n {
                if row[c] == b'\\' {
                    dsu.union_sets((r + 1) * (n + 1) + c + 1, r * (n + 1) + c);
                } else if row[c] == b'/' {
                    dsu.union_sets((r + 1) * (n + 1) + c, r * (n + 1) + c + 1);
                }
            }
        }
        dsu.count()
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
    use super::*;

    #[test]
    fn test_solution() {
        let func = |grid: &[&str]| {
            Solution::regions_by_slashes(grid.iter().map(|x| x.to_string()).collect())
        };

        assert_eq!(func(&["   ", " / ", "   "]), 1);
        assert_eq!(func(&[" /", "/ "]), 2);
        assert_eq!(func(&[" /", "  "]), 1);
        assert_eq!(func(&["/\\", "\\/"]), 5);
        assert_eq!(func(&["/\\/\\", "\\///", " \\//", "////"]), 9);
    }
}
