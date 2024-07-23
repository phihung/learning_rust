/// https://leetcode.com/problems/largest-rectangle-in-histogram/description/

// Top 100%
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        compute2(&heights)
    }
}

// O(N)
fn compute2(heights: &[i32]) -> i32 {
    // Heights and first idx of given height of the bars to the left
    //   in ascending order of height
    let mut height_and_positions = vec![(0, 0)];
    let mut result = 0;
    for (i, &current_h) in heights.iter().enumerate() {
        // idx of the last dropped
        let mut prev_p1 = i;
        loop {
            let &(h1, p1) = height_and_positions.last().unwrap();
            if current_h < h1 {
                // Drop all elements higher than current bar
                result = result.max(((i - p1) as i32) * h1);
                prev_p1 = p1;
                height_and_positions.pop();
            } else if current_h > h1 {
                // h1 = First height that is lower than current bar
                // prev_p1 = idx of last dropped = idx associated with the last height higher than the current bar
                height_and_positions.push((current_h, prev_p1));
                break;
            } else {
                // h = h1
                break;
            }
        }
    }
    for (h1, p1) in height_and_positions {
        result = result.max(((heights.len() - p1) as i32) * h1);
    }
    result
}

// O(N^2)
pub fn compute1(heights: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..heights.len() {
        let mut h = i32::MAX;
        for j in (0..(i + 1)).rev() {
            h = h.min(heights[j]);
            result = result.max(h * ((i + 1 - j) as i32));
            if h * ((i + 1) as i32) < result {
                break;
            }
        }
    }
    result
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution_2() {
    run_test(compute2);
}

#[test]
fn test_solution_1() {
    run_test(compute1);
}

#[cfg(test)]
fn run_test<F>(func: F)
where
    F: Fn(&[i32]) -> i32,
{
    assert_eq!(func(&vec![2, 1, 2]), 3);
    assert_eq!(func(&vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(func(&vec![2, 4]), 4);
    assert_eq!(func(&vec![999, 999, 999, 999]), 3996);
}
