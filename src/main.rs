use learning::to_owned;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let (m, n) = stones
            .iter()
            .fold((0, 0), |acc, x| (acc.0.max(x[0]), acc.1.max(x[1])));
        let (m, n) = (m as usize + 1, n as usize + 1);

        let mut dsu = DSU::new(m + n);
        for x in &stones {
            let (r, c) = (x[0] as usize, m + x[1] as usize);
            dsu.make_set(r);
            dsu.make_set(c);
            dsu.union_sets(r, c);
        }
        (stones.len() - dsu.count()) as i32
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
            .map(|(i, &x)| (i == x) as usize)
            .sum::<usize>()
    }

    pub fn make_set(&mut self, v: usize) {
        if self.parent[v] == usize::MAX {
            self.parent[v] = v;
        }
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

fn main() {
    let func = |a: &[_]| Solution::remove_stones(to_owned!(a));
    assert_eq!(func(&[[0, 2], [1, 2]]), 1);
    assert_eq!(func(&[[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]), 5);
    assert_eq!(func(&[[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]]), 3);
    assert_eq!(func(&[[0, 0]]), 0);
}
