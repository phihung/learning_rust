// https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/description/
use std::collections::BinaryHeap;

impl Solution {
    // Kruskal's algorithm
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let (n, n_edges) = (n as usize, edges.len());
        let (mut union_count, mut edge_count) = (0, 0);

        let mut add = |dsu: &mut DSU, (u, v), cnt: usize| {
            if dsu.union_sets(u, v) {
                union_count += cnt;
                edge_count += 1;
                if union_count == 2 * n - 2 {
                    return true; // stop
                }
            }
            false
        };

        let [edges_1, edges_2, edges_3] = {
            let mut edges_by_type = [vec![], vec![], vec![]];
            for e in edges {
                let (typ, u, v) = (e[0], e[1] as usize - 1, e[2] as usize - 1);
                edges_by_type[typ as usize - 1].push((u, v));
            }
            edges_by_type
        };

        let mut alice = DSU::new(n);
        (0..n).for_each(|i| alice.make_set(i));

        // Add type 3 edges. Each edge is counted 2
        for e in edges_3 {
            if add(&mut alice, e, 2) {
                return n_edges as i32 - edge_count;
            }
        }
        let mut bob = alice.clone();

        // Add type 1 edges. Each edge is counted 1
        for e in edges_1 {
            if add(&mut alice, e, 1) {
                return n_edges as i32 - edge_count;
            }
        }

        // Add type 2 edges. Each edge is counted 1
        for e in edges_2 {
            if add(&mut bob, e, 1) {
                return n_edges as i32 - edge_count;
            }
        }
        -1
    }

    // Kruskal's algorithm
    pub fn max_num_edges_to_remove2(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        let (n, n_edges) = (n as usize, edges.len());
        let mut a = DSU::new(n);
        let mut b = DSU::new(n);
        let (mut union_count, mut edge_count) = (0, 0);

        (0..n).for_each(|i| {
            a.make_set(i);
            b.make_set(i);
        });

        edges.sort_unstable_by_key(|x| if x[0] == 3 { 0 } else { 1 });
        for e in edges {
            let (typ, u, v) = (e[0], e[1] as usize - 1, e[2] as usize - 1);
            let mut used = false;
            if typ != 2 && a.union_sets(u, v) {
                used = true;
                union_count += 1;
            }
            if typ != 1 && b.union_sets(u, v) {
                used = true;
                union_count += 1;
            }
            if used {
                edge_count += 1;
                if union_count == 2 * n - 2 {
                    return n_edges as i32 - edge_count;
                }
            }
        }
        -1
    }

    // Prim's algorithm
    pub fn max_num_edges_to_remove3(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let n_edges = edges.len() as i32;
        let mut adj = vec![vec![]; n];
        for e in edges {
            let (typ, u, v) = (e[0], e[1] - 1, e[2] - 1);
            adj[u as usize].push((typ as u8, v as usize));
            adj[v as usize].push((typ as u8, u as usize));
        }

        let mut node_count = 2;
        let mut edge_count = 0;
        let mut visited: Vec<u8> = vec![0; n];
        visited[0] = 3;

        let mut queue = BinaryHeap::new();
        for &x in &adj[0] {
            queue.push(x);
        }

        while let Some((typ, node)) = queue.pop() {
            let v = &mut visited[node];
            if *v == 3 || *v == typ {
                continue;
            }
            if typ == 3 {
                if *v == 0 {
                    node_count += 1;
                }
                *v = 3;
            } else {
                *v += typ;
            }
            node_count += 1;
            edge_count += 1;
            if node_count == 2 * n {
                return n_edges - edge_count;
            }

            for &(next_typ, next) in &adj[node] {
                if visited[next] != 3 || visited[next] != next_typ {
                    // TODO: avoid re-adding useless nodes to reduce pression on the queue
                    queue.push((next_typ, next));
                }
            }
        }
        -1
    }
}

// https://cp-algorithms.com/data_structures/disjoint_set_union.html
#[derive(Clone, Debug)]
pub struct DSU {
    parent: Vec<usize>,
    // size: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![0; n],
            // size: vec![0; n],
        }
    }

    pub fn make_set(&mut self, v: usize) {
        self.parent[v] = v;
        // self.size[v] = 1;
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
            // if self.size[a] < self.size[b] {
            //     (a, b) = (b, a);
            // }
            self.parent[b] = a;
            // self.size[a] += self.size[b];
            true
        } else {
            false
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dsu() {
        let mut a = DSU::new(10);
        a.make_set(1);
        a.make_set(2);
        a.make_set(3);
        a.union_sets(3, 1);
        assert_eq!(a.find_set(1), a.find_set(3));
        assert_ne!(a.find_set(1), a.find_set(2));
        a.union_sets(2, 3);
        assert_eq!(a.find_set(1), a.find_set(2));
    }

    #[test]
    fn test_solution() {
        run_test(|n, es| {
            Solution::max_num_edges_to_remove(n, es.iter().map(|x| x.to_vec()).collect())
        });
    }

    #[test]
    fn test_solution2() {
        run_test(|n, es| {
            Solution::max_num_edges_to_remove2(n, es.iter().map(|x| x.to_vec()).collect())
        });
    }

    #[test]
    fn test_solution3() {
        run_test(|n, es| {
            Solution::max_num_edges_to_remove3(n, es.iter().map(|x| x.to_vec()).collect())
        });
    }

    fn run_test(func: impl Fn(i32, &[[i32; 3]]) -> i32) {
        assert_eq!(func(4, &[[3, 1, 2], [3, 3, 4], [1, 1, 3], [2, 2, 4]]), 0);
        assert_eq!(
            func(
                4,
                &[
                    [3, 1, 2],
                    [3, 2, 3],
                    [1, 1, 3],
                    [1, 2, 4],
                    [1, 1, 2],
                    [2, 3, 4]
                ]
            ),
            2
        );
        assert_eq!(func(4, &[[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]]), 0);
        assert_eq!(func(4, &[[3, 2, 3], [1, 1, 2], [2, 3, 4]]), -1);
        assert_eq!(
            func(
                4,
                &[
                    [1, 1, 2],
                    [2, 1, 4],
                    [2, 2, 3],
                    [1, 3, 4],
                    [1, 2, 4],
                    [2, 2, 4]
                ]
            ),
            0
        );
    }
}
