use std::collections::HashSet;
impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 1 {
            return true;
        }
        if n < 200 {
            Self::brut_force(nums)
        } else {
            Self::sieve(nums)
        }
    }
    pub fn sieve(nums: Vec<i32>) -> bool {
        let m = *nums.iter().max().unwrap() as usize;
        let mut mask = vec![0; m + 1];
        let mut prime: Vec<bool> = vec![true; m + 1];
        for &x in &nums {
            mask[x as usize] = 1;
        }
        if mask[1] == 1 {
            return false;
        }
        let mut arcs = vec![];
        let mut group = 2;
        for i in 2..(m + 1) {
            if prime[i] {
                let mut cnt = 0;
                let mut j = i;
                let mut merged_indices = HashSet::new();
                loop {
                    let v = &mut mask[j];
                    if *v == 1 {
                        *v = group;
                        cnt += 1;
                    } else if *v > 1 {
                        merged_indices.insert(*v);
                    }
                    j += i;
                    if j >= m + 1 {
                        break;
                    }
                    prime[j] = false;
                }
                if cnt > 0 || merged_indices.len() > 1 {
                    for v in merged_indices {
                        arcs.push((v, group));
                    }
                    group += 1;
                }
            }
        }
        // println!("{:?}", mask);
        // println!("{:?}", merged);
        Self::check_connectivity(&mut arcs, group as usize - 2)
    }

    fn check_connectivity(arcs: &mut [(i32, i32)], n_nodes: usize) -> bool {
        if n_nodes == 1 {
            return true;
        }
        if arcs.is_empty() && n_nodes > 1 {
            return false;
        }
        let mut connected = HashSet::new();
        connected.insert(arcs[0].0);
        loop {
            let mut changed = false;
            for (a, b) in arcs.iter_mut() {
                if *a == 0 {
                    continue;
                }
                if connected.contains(a) {
                    connected.insert(*b);
                    *a = 0;
                    changed = true;
                } else if connected.contains(b) {
                    connected.insert(*a);
                    *a = 0;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        return connected.len() == n_nodes;
    }

    pub fn brut_force(nums: Vec<i32>) -> bool {
        // let mut nums = nums;
        // nums.sort();
        let n = nums.len();
        let mut not_connected: HashSet<_> = (1..n).collect();
        let mut connected = vec![0];
        while let Some(idx) = connected.pop() {
            if not_connected.is_empty() {
                return true;
            }
            let mut remove = vec![];
            for &other_idx in &not_connected {
                if Self::gcd(nums[idx], nums[other_idx]) > 1 {
                    remove.push(other_idx);
                    connected.push(other_idx);
                }
            }

            for x in &remove {
                not_connected.remove(x);
            }
        }
        not_connected.is_empty()
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |x: &[i32]| Solution::can_traverse_all_pairs(x.to_vec());
    run_test(func);

    let func = |x: &[i32]| Solution::sieve(x.to_vec());
    run_test(func);

    let func = |x: &[i32]| Solution::brut_force(x.to_vec());
    run_test(func);

    assert!(!Solution::can_traverse_all_pairs(
        (2..100_000).rev().collect()
    ));
}

#[cfg(test)]
fn run_test<F>(func: F)
where
    F: Fn(&[i32]) -> bool,
{
    assert!(!func(&[2, 99991]));
    assert!(func(&vec![99991; 1000]));
    assert!(func(&[20, 6]));
    assert!(func(&[40, 22, 15]));
    assert!(func(&[35, 42, 42, 10, 11, 42, 45, 30, 33]));
    assert!(func(&[6, 3, 15, 5, 35, 7, 77, 11, 143]));
    assert!(func(&[6, 3, 11, 143, 15, 5, 35, 7, 77]));
    assert!(func(&[2]));
    assert!(!func(&[2, 3]));
    assert!(func(&[9, 12]));
    assert!(func(&[2, 3, 6]));
    assert!(!func(&[3, 9, 5]));
    assert!(func(&[4, 3, 12, 8]));
    assert!(func(&[4, 7, 3, 12, 8, 14]));
    assert!(!func(&[2, 3, 2, 3, 2, 3]));
}
