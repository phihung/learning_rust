// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/description

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        floyd_warshall(n, edges, distance_threshold)
        // all_point_dijkstra(n, edges, distance_threshold)
    }
}

pub fn floyd_warshall(n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let n = n as usize;
    let mut dist = vec![vec![threshold + 1; n]; n];
    for e in edges {
        if let &[v1, v2, d] = &e[..] {
            dist[v1 as usize][v2 as usize] = d;
            dist[v2 as usize][v1 as usize] = d;
        }
    }
    for i in 0..n {
        dist[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            if dist[i][k] > threshold {
                continue;
            }
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j])
            }
        }
    }
    let (mut imin, mut min) = (n, usize::MAX);
    for (i, row) in dist.into_iter().enumerate().rev() {
        let cnt = row.into_iter().filter(|&x| x <= threshold).count();
        if cnt < min {
            (imin, min) = (i, cnt);
        }
        if cnt == 0 {
            break;
        }
    }
    imin as i32
}

pub fn all_point_dijkstra(n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let n = n as usize;
    let graph = AdjGraph::from_edges(n, edges.into_iter().filter(|x| x[2] <= threshold));
    let (mut imin, mut min) = (n, i32::MAX);
    for i in (0..n).rev() {
        let cnt = graph.dijkstra(i, threshold);
        if cnt < min {
            (imin, min) = (i, cnt);
        }
        if cnt == 0 {
            break;
        }
    }
    imin as i32
}

pub struct AdjGraph {
    n_nodes: usize,
    adj_lists: Vec<Vec<(usize, i32)>>,
}

impl AdjGraph {
    fn from_edges(n_nodes: usize, edges: impl Iterator<Item = Vec<i32>>) -> Self {
        let mut adj_lists = vec![vec![]; n_nodes];
        for e in edges {
            if let &[v1, v2, dist] = &e[..] {
                adj_lists[v1 as usize].push((v2 as usize, dist));
                adj_lists[v2 as usize].push((v1 as usize, dist));
            }
        }
        Self { n_nodes, adj_lists }
    }

    fn dijkstra(&self, start: usize, threshold: i32) -> i32 {
        let n_nodes = self.n_nodes;
        let adj_lists = &self.adj_lists;
        let mut distances = vec![threshold + 1; n_nodes];
        let mut queue = BinaryHeap::new();
        let mut cnt = 0;
        queue.push((Reverse(0), start));
        distances[start] = 0;
        while let Some((Reverse(v_dist), v)) = queue.pop() {
            if v_dist > distances[v] {
                continue;
            }
            for &(u, d) in &adj_lists[v] {
                let vu_dist = v_dist + d;
                let v_dist = &mut distances[u];
                if vu_dist < *v_dist {
                    if *v_dist > threshold {
                        // assert!(vu_dist <= threshold);
                        cnt += 1;
                    }
                    *v_dist = vu_dist;
                    queue.push((Reverse(vu_dist), u));
                }
            }
        }
        cnt
    }
}

pub struct Solution;

#[test]
fn test_dijkstra() {
    let func = |n: i32, e: &[&[i32]], thr: i32| {
        all_point_dijkstra(n, e.iter().map(|&v| v.to_vec()).collect(), thr)
    };
    run_test(func)
}

#[test]
fn test_floyd_warshall() {
    let func = |n: i32, e: &[&[i32]], thr: i32| {
        floyd_warshall(n, e.iter().map(|&v| v.to_vec()).collect(), thr)
    };
    run_test(func)
}

#[cfg(test)]
fn run_test(func: impl Fn(i32, &[&[i32]], i32) -> i32) {
    let edges: &[&[i32]] = &[&[0, 1, 3], &[1, 2, 1], &[1, 3, 4], &[2, 3, 1]];
    assert_eq!(func(4, edges, 1), 0);
    assert_eq!(func(4, edges, 2), 0);
    assert_eq!(func(4, edges, 3), 0);
    assert_eq!(func(4, edges, 4), 3);
    assert_eq!(func(4, edges, 5), 3);
    assert_eq!(func(5, edges, 1), 4);
    assert_eq!(func(6, edges, 1), 5);

    let edges: &[&[i32]] = &[
        &[0, 1, 2],
        &[0, 4, 8],
        &[1, 2, 3],
        &[1, 4, 2],
        &[2, 3, 1],
        &[3, 4, 1],
    ];
    assert_eq!(func(5, edges, 1), 1);
    assert_eq!(func(5, edges, 2), 0);
    assert_eq!(func(5, edges, 3), 0);
    assert_eq!(func(5, edges, 4), 0);
    assert_eq!(func(5, edges, 5), 4);

    let edges: &[&[i32]] = &[
        &[0, 1, 1],
        &[0, 4, 1],
        &[0, 2, 1],
        &[1, 2, 3],
        &[1, 3, 1],
        &[2, 3, 5],
        &[3, 4, 3],
    ];
    assert_eq!(func(5, edges, 1), 4);
    assert_eq!(func(5, edges, 2), 3);
    assert_eq!(func(5, edges, 3), 4);
    assert_eq!(func(5, edges, 4), 4);
    assert_eq!(func(5, edges, 5), 4);
}
