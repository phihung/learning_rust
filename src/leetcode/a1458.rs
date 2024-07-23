/// https://leetcode.com/problems/max-dot-product-of-two-subsequences/description/

// 0ms, 2.1mb
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (_, n2) = (nums1.len(), nums2.len());

        // beta[j] : Solution for nums1[..i], nums2[..j]
        // alpha[j]: Solution for nums1[..i], nums2[..j] relaxing the contraint of non-empty product
        let mut beta = vec![-1e9 as i32; n2 + 1];
        let mut alpha = vec![0; n2 + 1];

        for v_i in nums1 {
            let mut old_alpha_j_minus_1 = alpha[0];
            for j in 1..=n2 {
                let alpha_j = alpha[j]
                    .max(old_alpha_j_minus_1 + v_i * nums2[j - 1])
                    .max(alpha[j - 1]);

                let beta_j = beta[j]
                    .max(old_alpha_j_minus_1 + v_i * nums2[j - 1])
                    .max(beta[j - 1]);

                old_alpha_j_minus_1 = alpha[j];
                alpha[j] = alpha_j;
                beta[j] = beta_j;
            }
        }
        *beta.last().unwrap()
    }
}

pub struct Solution {}

#[test]
fn test_solution() {
    let func =
        |nums1: &[i32], nums2: &[i32]| Solution::max_dot_product(nums1.to_vec(), nums2.to_vec());
    assert_eq!(func(&[2, 1, -2, 5], &[3, 0, -6]), 18);
    assert_eq!(func(&[3, -2], &[2, -6, 7]), 21);
    assert_eq!(func(&[-1, -1], &[1, 1]), -1);
    assert_eq!(func(&[1], &[-2]), -2);
    assert_eq!(func(&[1], &[-2, 3]), 3);
}
