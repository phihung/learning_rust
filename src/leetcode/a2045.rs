// https://leetcode.com/problems/second-minimum-time-to-reach-destination/description

use std::{cmp::Reverse, collections::BinaryHeap, i32};

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let graph = AdjGraph::from_edges(n, edges.into_iter());
        let dist = graph.find_second_shortest(0, n - 1);
        Self::compute_time(dist, time, change)
    }

    fn compute_time(dist: i32, time: i32, change: i32) -> i32 {
        let mut total = time;
        for _ in 0..(dist - 1) {
            if (total / change) % 2 == 1 {
                total += change - (total % change);
            }
            total += time;
        }
        total
    }
}

pub struct AdjGraph {
    n_nodes: usize,
    adj_lists: Vec<Vec<usize>>,
}

impl AdjGraph {
    fn from_edges(n_nodes: usize, edges: impl Iterator<Item = Vec<i32>>) -> Self {
        let mut adj_lists = vec![vec![]; n_nodes];
        for e in edges {
            if let &[v1, v2] = &e[..] {
                adj_lists[v1 as usize - 1].push(v2 as usize - 1);
                adj_lists[v2 as usize - 1].push(v1 as usize - 1);
            }
        }
        Self { n_nodes, adj_lists }
    }

    // bridge = 2 directly connected nodes of same min distance from start
    //   second_shortest = if no_bridge { shortest + 2} else { shortest + 1}
    // - If there is a bridge
    //      v-u (direct edge)
    //      dist(start, u) == dist(start, v)
    //      v in min_path(start, end)
    //   we can create a path + 1:   start => u => v => end
    // - If there is no such bridge
    //      start => x => start => ... end : length + 2
    fn find_second_shortest(&self, start: usize, end: usize) -> i32 {
        let adj_lists = &self.adj_lists;
        let mut dist = vec![self.n_nodes as i32 + 1; self.n_nodes];
        let mut queue = BinaryHeap::new();
        dist[0] = 0;
        queue.push((Reverse(0), false, start));

        while let Some((Reverse(v_dist), v_has_bridge, v)) = queue.pop() {
            if v_dist > dist[v] {
                continue;
            }
            let v_dist = dist[v];
            let has_bridge = v_has_bridge || adj_lists[v].iter().any(|&u| dist[u] == v_dist);
            if v == end {
                return if has_bridge { v_dist + 1 } else { v_dist + 2 };
            }

            let vu_dist = v_dist + 1;
            for &u in &adj_lists[v] {
                if vu_dist < dist[u] || (vu_dist == dist[u] && has_bridge && !v_has_bridge) {
                    dist[u] = vu_dist;
                    queue.push((Reverse(vu_dist), has_bridge, u));
                }
            }
        }
        unreachable!()
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let func = |n, e: &[[i32; 2]], t, c| {
        Solution::second_minimum(n, e.iter().map(|r| r.to_vec()).collect(), t, c)
    };
    assert_eq!(
        func(
            6,
            &[[1, 2], [1, 3], [1, 4], [2, 6], [3, 6], [4, 5], [5, 6]],
            3,
            2
        ),
        11
    );
    assert_eq!(func(5, &[[1, 2], [1, 3], [2, 4], [2, 5], [4, 5]], 3, 2), 11);
    assert_eq!(func(4, &[[1, 2], [2, 3], [3, 4]], 3, 2), 19);
    assert_eq!(func(5, &[[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]], 3, 5), 13);
    assert_eq!(func(2, &[[1, 2]], 3, 2), 11);
    assert_eq!(func(4, &[[1, 4], [1, 3], [1, 2], [2, 3]], 1, 5), 3);
    assert_eq!(
        func(5, &[[3, 5], [3, 4], [2, 5], [2, 1], [1, 3], [4, 1]], 1, 5),
        3
    );
}

#[test]
fn test_2() {
    let func = |n, e: &[[i32; 2]], t, c| {
        Solution::second_minimum(n, e.iter().map(|r| r.to_vec()).collect(), t, c)
    };
    #[rustfmt::skip]
    let edges = &[[1,2],[2,3],[1,4],[2,5],[2,6],[2,7],[7,8],[8,9],[7,10],[9,11],[11,12],[1,13],[3,14],[13,15],[14,16],[8,17],[4,18],[11,19],[17,11],[3,19],[19,7],[12,5],[8,1],[15,7],[19,6],[18,9],[6,8],[14,19],[13,18],[15,2],[13,12],[1,5],[16,18],[3,16],[6,1],[18,14],[12,1],[16,6],[13,11],[1,14],[16,13],[11,16],[4,15],[17,5],[5,9],[12,2],[4,10],[9,16],[17,9],[3,5],[10,2],[18,1],[15,18],[12,17],[10,6],[10,18],[19,12],[12,15],[19,13],[1,19],[9,14],[4,3],[17,13],[9,3],[17,10],[19,10],[5,4],[5,7],[14,17],[1,10],[4,11],[6,4],[5,10],[7,14],[8,14],[18,17],[15,10],[11,8],[14,11],[7,3],[5,18],[13,8],[4,12],[11,3],[5,15],[15,9],[8,10],[13,3],[17,1],[10,11],[15,11],[19,2],[1,3],[7,4],[18,11],[2,14],[9,1],[17,15],[7,13],[12,16],[12,8],[6,12],[9,6],[2,17],[15,6],[16,2],[12,7],[7,9],[8,4]];
    assert_eq!(func(19, edges, 850, 411), 1700);
}

#[test]
fn test_compute_time() {
    assert_eq!(Solution::compute_time(3, 3, 2), 11);
    assert_eq!(Solution::compute_time(11, 861, 272), 11741);
}
