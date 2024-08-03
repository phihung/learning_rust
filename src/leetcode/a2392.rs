// https://leetcode.com/problems/build-a-matrix-with-conditions/description
use std::collections::VecDeque;

// 100%, 100%
impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;
        let value_to_row = Self::find_permutation(k, &row_conditions);
        let value_to_col = Self::find_permutation(k, &col_conditions);
        match (value_to_row, value_to_col) {
            (Some(v2r), Some(v2c)) => {
                let mut m = vec![vec![0; k]; k];
                for (v, (i, j)) in v2r.into_iter().zip(v2c).enumerate() {
                    m[i][j] = v as i32 + 1;
                }
                m
            }
            _ => vec![],
        }
    }

    fn find_permutation(k: usize, conds: &Vec<Vec<i32>>) -> Option<Vec<usize>> {
        let adj_list = Self::build_graph(k, conds);
        Self::topological_sort(&adj_list).map(Self::invert)
    }

    fn invert(i2v: Vec<usize>) -> Vec<usize> {
        let mut v2i = vec![0; i2v.len()];
        for (pos, value) in i2v.into_iter().enumerate() {
            v2i[value] = pos;
        }
        v2i
    }

    // Kahn's algorithm
    fn topological_sort(adj_list: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
        let n_nodes = adj_list.len();
        let mut indegre: Vec<i32> = vec![0; n_nodes];
        for outs in adj_list {
            for &o in outs {
                indegre[o] += 1;
            }
        }
        let mut queue = VecDeque::with_capacity(n_nodes);
        let mut sorted = Vec::with_capacity(n_nodes);

        for (node, &cnt) in indegre.iter().enumerate() {
            if cnt == 0 {
                queue.push_back(node);
            }
        }

        while let Some(node) = queue.pop_front() {
            sorted.push(node);
            for &v in &adj_list[node] {
                indegre[v] -= 1;
                if indegre[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        if sorted.len() < n_nodes {
            None
        } else {
            Some(sorted)
        }
    }

    // fn topological_sort(adj_list: &Vec<HashSet<usize>>) -> Option<Vec<usize>> {
    //     let n_nodes = adj_list.len();
    //     let mut incomming_counts: Vec<i32> = vec![0; n_nodes];
    //     for outs in adj_list {
    //         for &o in outs {
    //             incomming_counts[o] += 1;
    //         }
    //     }
    //     let mut sorted = Vec::with_capacity(n_nodes);
    //     let mut nodes: Vec<_> = (0..n_nodes).collect();

    //     while nodes.len() > 0 {
    //         let node_with_no_incoming: HashSet<_> = nodes
    //             .iter()
    //             .filter(|&node| incomming_counts[*node] == 0)
    //             .map(|x| x.to_owned())
    //             .collect();
    //         if node_with_no_incoming.is_empty() {
    //             // circular
    //             return None;
    //         }
    //         for &node in &node_with_no_incoming {
    //             sorted.push(node);
    //             for &o in &adj_list[node] {
    //                 incomming_counts[o] -= 1;
    //             }
    //         }
    //         nodes.retain(|x| !node_with_no_incoming.contains(x));
    //     }
    //     Some(sorted)
    // }

    fn build_graph(k: usize, conds: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut adj_list = vec![Vec::new(); k];
        for arc in conds {
            let (fr, to) = (arc[0] as usize, arc[1] as usize);
            adj_list[fr - 1].push(to - 1);
        }
        adj_list
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    run_test(3, &[&[1, 2], &[3, 2]], &[&[2, 1], &[3, 2]], false);
    run_test(3, &[&[1, 2], &[2, 3], &[3, 1], &[2, 3]], &[&[2, 1]], true);
}

#[cfg(test)]
fn run_test(k: usize, rc: &[&[i32]], cc: &[&[i32]], no_solution: bool) {
    let m = Solution::build_matrix(
        k as i32,
        rc.iter().map(|x| x.to_vec()).collect(),
        cc.iter().map(|x| x.to_vec()).collect(),
    );
    if no_solution {
        assert!(m.is_empty());
        return;
    }
    assert_eq!(m.len(), k as usize);
    let mut positions = vec![None; k + 1];
    for (i, row) in m.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            assert!(v >= 0);
            if v > 0 {
                let v = v as usize;
                assert!(positions[v].is_none());
                positions[v] = Some((i, j));
            }
        }
    }

    for v in 1..=k {
        assert_ne!(positions[v], None);
    }

    for &a in rc {
        assert!(positions[a[0] as usize].unwrap().0 < positions[a[1] as usize].unwrap().0);
    }
    for &a in cc {
        assert!(positions[a[0] as usize].unwrap().1 < positions[a[1] as usize].unwrap().1);
    }
}
