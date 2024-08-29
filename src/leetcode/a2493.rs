// https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/

use std::collections::VecDeque;

// 1. The solution exists iff there is no cycle of odd length
// 2. For connected graph: magnificent_sets = 1 + max(distance between 2 nodes)
// 3. For non-connected graph: Sum of result for each connected component
impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_list = vec![vec![]; n];
        for e in edges {
            let (e0, e1) = ((e[0] - 1) as usize, (e[1] - 1) as usize);
            adj_list[e0].push(e1);
            adj_list[e1].push(e0);
        }

        // Pointer to the first node of connected component
        let mut group = vec![0; n];
        // Max distance between two nodes of the same connected components
        let mut max_distances = vec![];
        let mut dist = vec![-1; n];
        for root in 0..n {
            if dist[root] >= 0 {
                // The node belongs to an existing connected component
                group[root] = group[dist[root] as usize / n];
            } else {
                // New connected component
                max_distances.push(0);
                group[root] = max_distances.len() - 1;
            };
            let distance = &mut max_distances[group[root]];

            // BFS
            let mut queue = VecDeque::new();
            queue.push_back(root);
            dist[root] = (root * n) as i32;
            while let Some(node) = queue.pop_front() {
                for &next in &adj_list[node] {
                    if dist[next] == dist[node] {
                        // Cycle of length impair
                        return -1;
                    }
                    if dist[next] < dist[root] {
                        dist[next] = dist[node] + 1;
                        // Update max distance between 2 nodes
                        *distance = (*distance).max(dist[next] - dist[root]);
                        queue.push_back(next);
                    }
                }
            }
        }
        max_distances.iter().sum::<i32>() + max_distances.len() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::to_owned;

    use super::Solution;
    use rand::prelude::*;

    #[test]
    fn test_solution() {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut test = |n: i32, e: &[[i32; 2]], expect: i32| {
            let mut edges: Vec<Vec<i32>> = to_owned!(e);
            assert_eq!(Solution::magnificent_sets(n, edges.clone()), expect);
            for _ in 0..5 {
                shuffle(&mut rng, n, &mut edges);
                assert_eq!(Solution::magnificent_sets(n, edges.clone()), expect);
            }
        };
        test(8, &[[1, 2], [3, 4]], 8);
        test(3, &[[1, 2], [2, 3], [3, 1]], -1);
        test(6, &[[1, 2], [1, 4], [1, 5], [2, 6], [2, 3], [4, 6]], 4);
        test(10, &[[1, 2], [1, 4], [1, 5], [2, 6], [2, 3], [4, 6]], 8);
        test(15, &[[1, 5], [3, 5], [8, 5]], 14);

        test(
            12,
            &[
                [1, 2],
                [2, 3],
                [3, 4],
                [4, 5],
                [5, 6],
                [6, 1],
                [1, 7],
                [1, 11],
                [2, 8],
                [2, 12],
                [3, 7],
                [3, 9],
                [4, 8],
                [4, 10],
                [5, 9],
                [5, 11],
                [6, 10],
                [6, 12],
            ],
            4,
        );
    }

    fn shuffle(rng: &mut ThreadRng, n: i32, edges: &mut Vec<Vec<i32>>) {
        let mut m: Vec<_> = (1..=n).collect();
        m.shuffle(rng);
        edges.shuffle(rng);
        for e in edges {
            e[0] = m[(e[0] - 1) as usize];
            e[1] = m[(e[1] - 1) as usize];
        }
    }
}
