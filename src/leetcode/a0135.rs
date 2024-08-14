// https://leetcode.com/problems/candy/

impl Solution {
    // O(n) speed. O(n) memory
    pub fn candy(r: Vec<i32>) -> i32 {
        let n = r.len();

        // Number of candies to give if we ignore right-side constraint
        let mut lefts = vec![1; n];
        for i in 1..n {
            lefts[i] = if r[i] > r[i - 1] { lefts[i - 1] + 1 } else { 1 };
        }

        let mut total = lefts[n - 1];

        // Number of candies to give if we ignore left-side constraint
        let mut right = 1;
        for i in (0..(n - 1)).rev() {
            right = if r[i] > r[i + 1] { right + 1 } else { 1 };
            total += right.max(lefts[i]);
        }
        total
    }

    // O(n) speed. O(1) memory
    pub fn candy2(r: Vec<i32>) -> i32 {
        // Find 1-candy children: r[i] <= min(r[i-1], r[i+1])
        // Between two consecutive 1-candy children
        //  - The rating can only follow simple pattern: increase, equal, then decrease.
        //  - Count the numbers of increase, equal, decrease
        //  - Update the total based on these numbers
        let n = r.len();
        if n == 1 {
            return 1;
        }
        let (mut dec, mut inc, mut eq) = (0, 0, 0);
        let mut total = if r[0] <= r[1] { 1 } else { 0 };
        for i in 1..=n {
            let bad = (i == n || r[i] <= r[i - 1]) && (i >= n - 1 || r[i] <= r[i + 1]);
            if bad {
                if i < n {
                    total += 1;
                    if r[i] < r[i - 1] {
                        dec += 1;
                    }
                }
                let max = dec.max(inc);
                let min = dec.min(inc) + eq;

                // 2 + ... + (max + 1)
                total += (max + 1) * (max + 2) / 2 - 1;
                if min > 0 {
                    // 2 + ... + min
                    total += min * (min + 1) / 2 - 1;
                }

                (inc, dec, eq) = (0, 0, 0)
            } else {
                if r[i] > r[i - 1] {
                    inc += 1;
                } else if r[i] < r[i - 1] {
                    dec += 1;
                } else {
                    eq += 1;
                }
            }
        }
        total as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(|nums: &[i32]| Solution::candy(nums.to_vec()));
    }

    #[test]
    fn test_solution2() {
        run_test(|nums: &[i32]| Solution::candy2(nums.to_vec()));
    }

    fn run_test(func: impl Fn(&[i32]) -> i32) {
        assert_eq!(func(&[3, 3, 2, 2, 1, 1]), 8);

        assert_eq!(func(&[2, 3, 5, 5, 4, 1]), 12);
        assert_eq!(func(&[1, 1, 1, 1, 1]), 5);
        assert_eq!(func(&[1, 2, 1, 2]), 6);
        assert_eq!(func(&[2, 2, 2, 1, 2, 2]), 8);
        assert_eq!(func(&[1, 2, 3, 3, 3, 4, 5]), 13);
        assert_eq!(func(&[5, 4, 3, 3, 3, 2, 1]), 13);
        assert_eq!(func(&[1, 1, 2, 2, 3, 3]), 8);
        assert_eq!(func(&[3, 3, 2, 2, 1, 1]), 8);
        assert_eq!(func(&[1, 2, 3, 1, 2, 4, 3, 2, 1, 2, 3]), 24);
        assert_eq!(func(&[1, 0, 2]), 5);
        assert_eq!(func(&[1, 2, 2]), 4);
        assert_eq!(func(&[2, 2, 1]), 4);
    }
}
