/// https://leetcode.com/problems/minimum-cost-to-equalize-array/description/

const MODULO: i64 = 1_000_000_007;

// Step 1: Use 2-blocks to transform the problem to "special blocks"
// Step 2: Do the computation for special blocks
impl Solution {
    // Block of form: [M - k, M, M,..., M] (n-1 times M)
    fn min_cost_special_block(n: i64, k: i64, c1: i64, c2: i64) -> i64 {
        let mut out = i64::MAX;
        for one_block in 0..(k + 1) {
            let k1 = k - one_block;
            if n % 2 == 0 && k1 % 2 == 1 {
                continue;
            }
            let mut x = (k1 as f32 / (n as f32 - 2.)).ceil() as i64;
            if (x * n + k1) % 2 == 1 {
                x += 1;
            }
            let v = one_block * c1 + (n * x + k1) / 2 * c2;
            if v < out {
                out = v;
            } else {
                break;
            }
        }
        out = out.min(k * c1);
        return out;
    }

    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        let n = nums.len() as i32;
        let cost1 = cost1 as i64;
        let cost2 = cost2 as i64;

        let max = *nums.iter().max().unwrap() as i64;
        let missing: i64 = nums.iter().map(|&x| max - (x as i64)).sum();

        if cost2 >= 2 * cost1 || n <= 2 {
            return (missing * cost1 % MODULO) as i32;
        }
        let min = max - (*nums.iter().min().unwrap() as i64);
        let k = 2 * min - missing;
        let out = if k > 0 {
            cost2 * (missing - min) + Self::min_cost_special_block(n as i64, k as i64, cost1, cost2)
        } else {
            if missing % 2 == 0 {
                cost2 * (missing / 2)
            } else {
                cost2 * (missing / 2) + Self::min_cost_special_block(n as i64, 1, cost1, cost2)
            }
        };
        (out % MODULO) as i32
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func =
        |x: &[i32], cost1, cost2| Solution::min_cost_to_equalize_array(x.to_vec(), cost1, cost2);
    assert_eq!(func(&[1, 9, 9, 9, 9], 10, 4), 52);
    assert_eq!(func(&[2, 9, 9, 9, 9], 10, 4), 42);
    assert_eq!(func(&[2, 11, 11, 11, 12], 10, 4), 54);
    assert_eq!(func(&[1, 3, 3, 3, 3], 60, 2), 12);
    assert_eq!(func(&[1, 3, 3, 3, 3], 4, 2), 8);
    assert_eq!(func(&[1, 3, 3, 3, 3], 5, 2), 10);
    assert_eq!(func(&[1, 1000000], 1000000, 1), 998993007);
    assert_eq!(func(&[60, 19, 53, 31, 57], 60, 2), 90);
    assert_eq!(func(&[2, 3, 3, 3, 5], 2, 1), 6);
    assert_eq!(func(&[3, 5, 3], 1, 3), 4);
    assert_eq!(func(&[4, 1], 5, 2), 15);
    assert_eq!(func(&[4, 3], 2, 6), 2);
    assert_eq!(func(&[1, 14, 14, 15], 2, 1), 20);
}
