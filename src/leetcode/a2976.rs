// https://leetcode.com/problems/minimum-cost-to-convert-string-i/description

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let dist = Self::floyd_warshall(&original, &changed, &cost);
        let mut total: i64 = 0;
        for (&c1, &c2) in source
            .as_bytes()
            .into_iter()
            .zip(target.as_bytes().into_iter())
        {
            let d = dist[(c1 - b'a') as usize][(c2 - b'a') as usize];
            if d == i32::MAX {
                return -1;
            }
            total += d as i64;
        }
        total
    }

    pub fn floyd_warshall(original: &[char], changed: &[char], cost: &[i32]) -> [[i32; 26]; 26] {
        let c2i = |c| (c as u8 - b'a') as usize;
        let n = 26;
        let max = i32::MAX;
        let mut dist = [[max; 26]; 26];
        for ((&fr, &to), &d) in original.iter().zip(changed).zip(cost) {
            let v = &mut dist[c2i(fr)][c2i(to)];
            *v = d.min(*v);
        }
        for i in 0..n {
            dist[i][i] = 0;
        }

        for k in 0..n {
            for i in 0..n {
                if dist[i][k] >= max {
                    continue;
                }
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
                }
            }
        }
        dist
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let func = |src: &str, tgt: &str, org: &str, chg: &str, cost: &[i32]| {
        Solution::minimum_cost(
            src.to_string(),
            tgt.to_string(),
            org.chars().collect(),
            chg.chars().collect(),
            cost.to_vec(),
        )
    };
    assert_eq!(
        func("abcd", "acbe", "abcced", "bcbebe", &[2, 5, 5, 1, 2, 20]),
        28
    );
    assert_eq!(
        func("abcd", "acde", "abcced", "bcbebe", &[2, 5, 5, 1, 2, 20]),
        -1
    );
    assert_eq!(func("aaaa", "bbbb", "ac", "cb", &[1, 2]), 12);
}
