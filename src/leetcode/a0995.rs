impl Solution {
    // O(N)
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        const FLIP: i32 = 2;
        let (n, k) = (nums.len(), k as usize);

        let mut flip = 0;
        let mut count = 0;
        for i in 0..n {
            if nums[i] == flip {
                // x flip x times = 0
                if i < n + 1 - k {
                    nums[i] = FLIP;
                    count += 1;
                    flip ^= 1;
                } else {
                    return -1;
                }
            }
            if i >= k - 1 && nums[i + 1 - k] == FLIP {
                flip ^= 1;
            }
        }
        count
    }

    // O(NlogN)
    pub fn min_k_bit_flips2(nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), k as usize);

        // bit.sum(i+1) == true => nums[i] is flipped
        // bit.flip(i) => flip all the sum from i to n
        let mut bit = BIT::new(n);

        let mut cnt = 0;
        for i in 0..n {
            if (nums[i] == 1) == bit.sum(i + 1) {
                if i <= n - k {
                    bit.flip(i);
                    bit.flip(i + k);
                    cnt += 1;
                } else {
                    return -1;
                }
            }
        }
        cnt
    }
}

#[derive(Debug)]
pub struct BIT {
    bits: Vec<bool>,
}

// Binary Indexed Tree: https://cp-algorithms.com/data_structures/fenwick.html
impl BIT {
    fn new(n: usize) -> Self {
        Self {
            bits: vec![false; n],
        }
    }

    // Sum the number of values < idx
    // O(log(N))
    fn sum(&self, mut idx: usize) -> bool {
        let mut ret = false;
        while idx > 0 {
            ret ^= self.bits[idx - 1];
            idx &= idx - 1;
        }
        ret
    }

    // O(log(N))
    fn flip(&mut self, mut idx: usize) {
        while idx < self.bits.len() {
            self.bits[idx] = !self.bits[idx];
            idx |= idx + 1;
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, fs, path::Path};

    #[test]
    fn test_solution() {
        run_test(|nums, k| Solution::min_k_bit_flips(nums.to_vec(), k));
    }

    #[test]
    fn test_solution2() {
        run_test(|nums, k| Solution::min_k_bit_flips2(nums.to_vec(), k));
    }

    fn run_test(func: impl Fn(&[i32], i32) -> i32) {
        assert_eq!(func(&[0, 1, 1], 2), -1);
        assert_eq!(func(&[0, 1, 0], 1), 2);
        assert_eq!(func(&[1, 1, 0], 2), -1);
        assert_eq!(func(&[0, 0, 0, 1, 0, 1, 1, 0], 3), 3);
    }

    #[bench]
    fn test_96(b: &mut test::Bencher) -> Result<(), Box<dyn Error>> {
        let path = Path::new("data/leetcode/0995/96.json");
        if !path.exists() {
            return Ok(());
        }
        let (nums, k, exp): (Vec<i32>, i32, i32) =
            serde_json::from_str(&fs::read_to_string(path)?)?;

        b.iter(|| assert_eq!(Solution::min_k_bit_flips(nums.clone(), k), exp));
        Ok(())
    }

    #[test]
    fn test_bit() {
        let mut bit = BIT::new(7);
        for (i, &v) in [1, 0, 1, 0, 1, 0, 1].iter().enumerate() {
            if v == 1 {
                bit.flip(i);
                bit.flip(i + 1);
            }
        }
        let v: Vec<_> = (0..7).map(|i| bit.sum(i + 1) as u8).collect();
        assert_eq!(v, vec![1, 0, 1, 0, 1, 0, 1]);
        bit.flip(1);
        bit.flip(4);
        let v: Vec<_> = (0..7).map(|i| bit.sum(i + 1) as u8).collect();
        assert_eq!(v, vec![1, 1, 0, 1, 1, 0, 1]);
    }
}
