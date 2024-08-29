// https://leetcode.com/problems/find-the-winner-of-the-circular-game/description

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        Self::dp(n, k)
    }

    pub fn dp(n: i32, k: i32) -> i32 {
        let mut pos = 0;
        for n1 in 2..=n {
            pos = (k + pos) % n1;
        }
        pos + 1
    }

    // O(k * log(n/k))
    // faster when k << n
    pub fn dp2(mut n: i32, k: i32) -> i32 {
        if k == 1 {
            return n;
        }
        let mut trajectory = vec![];
        while n > 1 {
            trajectory.push(n);
            n -= (n / k).max(1);
        }
        let mut pos = 0;
        for n1 in trajectory.into_iter().rev() {
            let n_out = (n1 / k).max(1);
            let next = k * n_out;
            if pos + next < n1 {
                pos = pos + next;
            } else {
                let y = pos + next - n1;
                pos = ((y / (k - 1)) * k + (y % (k - 1))) % n1;
            }
        }
        pos + 1
    }

    pub fn recursive(n: i32, k: i32) -> i32 {
        fn compute(n: i32, k: i32) -> i32 {
            if n == 1 {
                return 0;
            }
            let n_out = (n / k).max(1);
            let x = compute(n - n_out, k);
            let next = k * n_out;
            if x + next < n {
                return x + next;
            }
            let y = x + next - n;
            let out = ((y / (k - 1)) * k + (y % (k - 1))) % n;
            out
        }
        if k == 1 {
            n
        } else {
            compute(n, k) + 1
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(Solution::find_the_winner);
    }

    #[test]
    fn test_dp2() {
        run_test(Solution::dp2);
    }

    #[test]
    fn test_dp() {
        run_test(Solution::dp);
    }

    #[test]
    fn test_recursive() {
        run_test(Solution::recursive);
    }

    fn run_test(func: impl Fn(i32, i32) -> i32) {
        assert_eq!(func(10, 1), 10);

        assert_eq!(func(2, 2), 1);
        assert_eq!(func(3, 2), 3);
        assert_eq!(func(4, 2), 1);
        assert_eq!(func(5, 2), 3);
        assert_eq!(func(6, 2), 5);
        assert_eq!(func(7, 2), 7);

        assert_eq!(func(6, 5), 1);
        assert_eq!(func(8, 5), 3);
        assert_eq!(func(9, 5), 8);
        assert_eq!(func(10, 5), 3);
        assert_eq!(func(11, 5), 8);
        assert_eq!(func(12, 5), 1);
        assert_eq!(func(20, 5), 7);
    }
}
