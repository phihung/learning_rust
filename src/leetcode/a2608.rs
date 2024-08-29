// https://leetcode.com/problems/shortest-cycle-in-a-graph/

use std::collections::VecDeque;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_list = vec![vec![]; n];
        for e in edges {
            adj_list[e[0] as usize].push(e[1] as usize);
            adj_list[e[1] as usize].push(e[0] as usize);
        }
        let mut length = i32::MAX;
        let mut dist = vec![-1; n];
        for root in 0..n {
            let mut queue = VecDeque::new();
            queue.push_back(root);
            dist[root] = (root * n) as i32;
            'a: while let Some(node) = queue.pop_front() {
                if (dist[node] - dist[root]) * 2 + 1 >= length {
                    break;
                }
                for &next in &adj_list[node] {
                    if dist[next] == dist[node] {
                        length = dist[node] + dist[next] - 2 * dist[root] + 1;
                        break 'a;
                    }
                    if dist[next] > dist[node] {
                        length = dist[node] + dist[next] - 2 * dist[root] + 1;
                        break;
                    }
                    if root < next && dist[next] < dist[root] {
                        dist[next] = dist[node] + 1;
                        queue.push_back(next);
                    }
                }
            }
        }
        if length == i32::MAX {
            -1
        } else {
            length
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use rand::prelude::*;

    #[test]
    fn test_solution() {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut test = |n: i32, e: &[[i32; 2]], expect: i32| {
            let mut edges: Vec<Vec<i32>> = e.iter().map(|x| x.to_vec()).collect();
            assert_eq!(Solution::find_shortest_cycle(n, edges.clone()), expect);
            for _ in 0..5 {
                shuffle(&mut rng, n, &mut edges);
                assert_eq!(Solution::find_shortest_cycle(n, edges.clone()), expect);
            }
        };

        test(5, &[[0, 1], [0, 2], [0, 3], [1, 4], [2, 4], [2, 3]], 3);

        test(
            9,
            &[
                [0, 1],
                [0, 2],
                [1, 3],
                [1, 4],
                [2, 5],
                [2, 6],
                [3, 7],
                [4, 7],
                [5, 7],
                [6, 8],
                [7, 8],
            ],
            4,
        );

        test(5, &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 3], [3, 4]], 3);

        test(5, &[[0, 1], [0, 2], [1, 3], [2, 3], [2, 4], [3, 4]], 3);
        test(4, &[[0, 1], [0, 2], [1, 2], [1, 3], [2, 3]], 3);
        test(4, &[[0, 1], [1, 2], [2, 3], [3, 0]], 4);
        test(3, &[[0, 1], [1, 2], [2, 0]], 3);
        test(2, &[[0, 1]], -1);
        test(3, &[[0, 1], [0, 2]], -1);
        test(4, &[[0, 1], [1, 2], [2, 3], [3, 1]], 3);
        test(4, &[[0, 2], [2, 3], [3, 0], [0, 1]], 3);

        test(
            7,
            &[[0, 1], [1, 2], [2, 0], [3, 4], [4, 5], [5, 6], [6, 3]],
            3,
        );
    }

    fn shuffle(rng: &mut ThreadRng, n: i32, edges: &mut Vec<Vec<i32>>) {
        let mut m: Vec<_> = (0..n).collect();
        m.shuffle(rng);
        edges.shuffle(rng);
        for e in edges {
            e[0] = m[e[0] as usize];
            e[1] = m[e[1] as usize];
        }
    }
}
