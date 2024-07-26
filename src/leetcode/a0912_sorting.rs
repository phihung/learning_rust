// https://leetcode.com/problems/sort-an-array/

use std::ptr;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        // QuickSort::run(&mut nums);
        // MergeSort::run(&mut nums)
        CountingSort::run(&mut nums)
    }
}

pub struct CountingSort;

impl CountingSort {
    pub fn run(arr: &mut [i32]) -> Vec<i32> {
        let (min, max) = arr
            .iter()
            .fold((i32::MAX, i32::MIN), |acc, &x| (acc.0.min(x), acc.1.max(x)));
        if min == max {
            return arr.to_owned();
        }

        let k = (max - min + 1) as usize;
        let mut count = vec![0; k + 1];
        let mut out = vec![0; arr.len()];

        arr.iter().for_each(|&v| count[Self::key(v, min)] += 1);
        (1..k).for_each(|v| count[v] += count[v - 1]);

        for &v in arr.iter().rev() {
            let j = Self::key(v, min);
            count[j] -= 1;
            out[count[j]] = v
        }
        out
    }

    #[inline]
    fn key(v: i32, min: i32) -> usize {
        (v - min) as usize
    }
}
pub struct MergeSort;

impl MergeSort {
    pub fn run(arr: &mut [i32]) -> Vec<i32> {
        let mut b = vec![0; arr.len()];
        Self::merge_sort(arr, &mut b).to_owned()
    }

    fn merge_sort<'a>(mut a: &'a mut [i32], mut b: &'a mut [i32]) -> &'a mut [i32] {
        let n = a.len();
        let mut width = 1;
        while width < n {
            // Array A is full of runs of length width.
            for i in (0..n).step_by(2 * width) {
                // println!("{i} {width}");
                Self::merge(a, i, n.min(i + width), n.min(i + 2 * width), b);
            }
            (a, b) = (b, a);
            width *= 2;
        }
        a
    }

    fn merge(a: &mut [i32], i_left: usize, i_right: usize, i_end: usize, b: &mut [i32]) {
        let (mut i, mut j) = (i_left, i_right);
        // While there are elements in the left or right runs...
        for k in i_left..i_end {
            if i < i_right && (j >= i_end || a[i] <= a[j]) {
                b[k] = a[i];
                i = i + 1;
            } else {
                b[k] = a[j];
                j = j + 1;
            }
        }
    }
}

pub struct QuickSort;

impl QuickSort {
    pub fn run(nums: &mut [i32]) {
        let n = nums.len();
        Self::sort(nums, 0, n);
    }

    fn sort(arr: &mut [i32], lo: usize, hi: usize) {
        if lo + 1 >= hi {
            return;
        }
        let p = Self::partition(arr, lo, hi);
        Self::sort(arr, lo, p + 1);
        Self::sort(arr, p + 1, hi);
    }

    fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
        let ptr = arr.as_mut_ptr();
        let pivot = Self::mo3(arr, lo, hi - 1);
        let (mut i, mut j) = (lo, hi - 1);
        loop {
            while arr[i] < pivot {
                i += 1;
            }
            while arr[j] > pivot {
                j -= 1;
            }
            if i >= j {
                return j;
            };
            Self::swap(ptr, i, j);
            i += 1;
            j -= 1;
        }
    }

    #[inline]
    fn mo3(arr: &mut [i32], lo: usize, hi: usize) -> i32 {
        let ptr = arr.as_mut_ptr();
        let mid = (lo + hi) / 2;
        if arr[mid] < arr[lo] {
            Self::swap(ptr, lo, mid);
        }
        if arr[hi] < arr[lo] {
            Self::swap(ptr, lo, hi);
        }
        if arr[mid] < arr[hi] {
            Self::swap(ptr, hi, mid);
        }
        arr[hi]
    }

    #[inline]
    fn swap(ptr: *mut i32, i: usize, j: usize) {
        unsafe { ptr::swap(ptr.add(i), ptr.add(j)) };
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let func = |a: &[i32]| Solution::sort_array(a.to_vec());
    run_test(func)
}

#[test]
fn test_quick_sort() {
    let func = |a: &[i32]| {
        let mut a = a.to_owned();
        QuickSort::run(&mut a);
        a
    };
    run_test(func)
}

#[test]
fn test_merge_sort() {
    let func = |a: &[i32]| {
        let mut a = a.to_owned();
        MergeSort::run(&mut a)
    };
    run_test(func)
}

#[test]
fn test_counting_sort() {
    let func = |a: &[i32]| {
        let mut a = a.to_owned();
        CountingSort::run(&mut a)
    };
    run_test(func)
}

#[cfg(test)]
fn run_test<F>(func: F)
where
    F: Fn(&[i32]) -> Vec<i32>,
{
    assert_eq!(func(&[1, 3, 2, 4, 0]), [0, 1, 2, 3, 4]);

    let n_max = 50_000;
    assert_eq!(func(&vec![2; n_max]), vec![2; n_max]);
    assert_eq!(
        func(&(1..(n_max as i32)).collect::<Vec<i32>>()),
        (1..(n_max as i32)).collect::<Vec<i32>>()
    );
    assert_eq!(
        func(&(1..(n_max as i32)).rev().collect::<Vec<i32>>()),
        (1..(n_max as i32)).collect::<Vec<i32>>()
    );

    assert_eq!(func(&[-2, 3, -5]), [-5, -2, 3]);
    assert_eq!(func(&[1]), [1]);
    assert_eq!(func(&[1, 1]), [1, 1]);
    assert_eq!(func(&[1; 10]), &[1; 10]);
    assert_eq!(func(&[5, 2, 3, 1]), [1, 2, 3, 5]);
    assert_eq!(func(&[5, 1, 1, 2, 0, 0]), [0, 0, 1, 1, 2, 5]);
}

#[bench]
fn benchmark_mergesort(b: &mut test::Bencher) {
    let data = get_data();

    b.iter(|| {
        data.iter().for_each(|nums| {
            MergeSort::run(&mut nums.clone());
        });
    });
}

#[bench]
fn benchmark_quicksort(b: &mut test::Bencher) {
    let data = get_data();

    b.iter(|| {
        data.iter().for_each(|nums| {
            QuickSort::run(&mut nums.clone());
        });
    });
}

#[bench]
fn benchmark_counting_sort(b: &mut test::Bencher) {
    let data = get_data();

    b.iter(|| {
        data.iter().for_each(|nums| {
            CountingSort::run(&mut nums.clone());
        });
    });
}

#[cfg(test)]
fn get_data() -> [Vec<i32>; 3] {
    let n_max = 50_000;
    [
        (1..(n_max as i32)).collect::<Vec<i32>>(),
        (1..(n_max as i32)).rev().collect::<Vec<i32>>(),
        vec![0; n_max],
    ]
}
