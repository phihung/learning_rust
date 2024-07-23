/// https://leetcode.com/problems/median-of-two-sorted-arrays/description/
/// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

impl Solution {
    fn median(arr: &[i32]) -> f64 {
        let n = arr.len();
        if n % 2 == 0 {
            (arr[n / 2] + arr[n / 2 - 1]) as f64 / 2.
        } else {
            arr[n / 2] as f64
        }
    }

    // Take for example two arrays: (1, 2, 3) and (4, 5, 6, 7)
    // Need to find median => 3th element. The median should be greater than exactly 3 numbers
    // Split the arrays at i1 = 1 (number 2) and i2 = 2 (number 6)
    //        i1 + i2 = 3
    // We have: 2 < 6  (nums1[i1] < nums2[i2])
    // So: All number from i2 = 6 must be discarded, as 6 is greater than (i1 + 1) + i2 = 4 elements
    //     All number before i1 = 2 must be discarded, as they can only be greater than i1 + (i2 - 1) = 2 elements
    // New arrays: (x, 2, 3), (4, 5, x, x)
    // And the loop continues
    pub fn find_median_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
        let (n1, n2) = (nums1.len(), nums2.len());
        match (n1, n2) {
            // (1, 1) => return (nums1[0] + nums2[0]) as f64 / 2.,
            (0, _) => return Self::median(nums2),
            (_, 0) => return Self::median(nums1),
            _ => (),
        }
        if n1 == 0 {}
        let target_idx = (n1 + n2) / 2;
        let need_avg = (n1 + n2) % 2 == 0;
        let (mut a1, mut b1) = (0, nums1.len());
        let (mut a2, mut b2) = (0, nums2.len());
        let (mut i1, mut i2, mut split_ok) = (0, 0, true);
        while a1 < b1 && a2 < b2 {
            // Split: a1 <= i1 < b1, a2 <= i2 < b2
            // So that: i1 + i2 = target_idx
            // Sometime this is not possible, in this case take i1 + i2 = target_idx - 1
            i1 = (a1.max(target_idx.checked_sub(b2).unwrap_or(0)) + b1.min(target_idx - a2)) / 2;
            i2 = (target_idx - i1).min(b2 - 1);
            split_ok = i1 + i2 == target_idx;
            assert!(a1 <= i1 && i1 < b1);
            assert!(a2 <= i2 && i2 < b2);
            assert!(i1 + i2 == target_idx || i1 + i2 == target_idx - 1);

            if nums1[i1] < nums2[i2] {
                (a1, b2) = if split_ok { (i1, i2) } else { (i1 + 1, i2 + 1) };
            } else if nums1[i1] > nums2[i2] {
                (a2, b1) = if split_ok { (i2, i1) } else { (i2 + 1, i1 + 1) };
            } else {
                break;
            }
        }

        let (arr1, arr2, i1, i2) = if a1 == b1 {
            (nums2, nums1, i2, i1)
        } else {
            (nums1, nums2, i1, i2)
        };
        let v = arr1[i1];
        if need_avg {
            let mut previous = 0;
            if i1 > 0 {
                previous = previous.max(arr1[i1 - 1]);
            };
            if !split_ok {
                previous = previous.max(arr2[i2]);
            } else if i2 > 0 {
                previous = previous.max(arr2[i2 - 1]);
            };
            return (v as f64 + previous as f64) / 2.;
        } else {
            v as f64
        }
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    basic_test_cases(Solution::find_median_sorted_arrays);
}

#[cfg(test)]
fn basic_test_cases<F>(func: F)
where
    F: Fn(&Vec<i32>, &Vec<i32>) -> f64,
{
    assert_eq!(func(&vec![4], &vec![1, 2, 3, 5]), 3.);
    assert_eq!(func(&vec![4], &vec![1, 2, 3]), 2.5);
    assert_eq!(func(&vec![3], &vec![1, 2, 4]), 2.5);
    assert_eq!(func(&vec![4], &vec![1, 2, 3, 5]), 3.);
    assert_eq!(func(&vec![1], &vec![2]), 1.5);
    assert_eq!(func(&vec![1], &vec![2, 3]), 2.);
    assert_eq!(func(&vec![1], &vec![2, 3, 4]), 2.5);
    assert_eq!(func(&vec![], &vec![1, 3]), 2.);
    assert_eq!(func(&vec![1, 1], &vec![1, 2]), 1.);
    assert_eq!(func(&vec![1], &vec![1]), 1.);
    assert_eq!(func(&vec![1], &vec![2]), 1.5);
    assert_eq!(func(&vec![1, 2], &vec![3]), 2.);
    assert_eq!(func(&vec![1], &vec![1, 1]), 1.);
    assert_eq!(func(&vec![1, 2, 3, 4], &vec![5, 6, 7]), 4.);
    assert_eq!(func(&vec![1, 2, 3, 4], &vec![5, 6, 7, 8]), 4.5);
    assert_eq!(func(&vec![1, 2], &vec![3, 4]), 2.5);
    assert_eq!(func(&vec![2, 2, 4, 4], &vec![2, 2, 4, 4]), 3.);
}
