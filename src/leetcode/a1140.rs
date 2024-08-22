// https://leetcode.com/problems/stone-game-ii

impl Solution {
    pub fn stone_game_ii(mut piles: Vec<i32>) -> i32 {
        let n = piles.len();
        for i in (0..(n - 1)).rev() {
            piles[i] += piles[i + 1];
        }
        fn recursive(psum: &[i32], m: usize, memory: &mut Vec<Vec<i32>>) -> i32 {
            let n = psum.len();
            if n <= 2 * m {
                return if n == 0 { 0 } else { psum[0] };
            }
            if memory[n - 1][m] != -1 {
                return memory[n - 1][m];
            }
            // minimize next player score
            let min = (1..=(2 * m).min(psum.len()))
                .map(|k| recursive(&psum[k..], m.max(k), memory))
                .min()
                .unwrap();
            memory[n - 1][m] = psum[0] - min;
            psum[0] - min
        }
        recursive(&piles, 1, &mut vec![vec![-1; 33]; piles.len()])
    }

    // dp
    pub fn stone_game_ii2(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut dp = vec![vec![piles[n - 1]; 33]];
        let mut psum = piles[n - 1];
        for i in (0..(n - 1)).rev() {
            psum += piles[i];
            let mut dp_row = vec![psum; 33];
            for m in 1..((n - i) as f32 / 2.).ceil().min(33.) as usize {
                // dp[m] = psum;
                let mut min = i32::MAX;
                for k in 1..=(2 * m).min(n - i) {
                    min = min.min(dp[n - i - k - 1][m.max(k).min(32)]);
                }
                dp_row[m] = psum - min;
            }
            dp.push(dp_row);
        }
        dp[n - 1][1]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |p: &[i32]| Solution::stone_game_ii(p.to_vec());
        run_test(func);
    }

    #[test]
    fn test_solution2() {
        let func = |p: &[i32]| Solution::stone_game_ii2(p.to_vec());
        run_test(func);
    }

    fn run_test(func: impl Fn(&[i32]) -> i32) {
        assert_eq!(func(&[9, 4, 4]), 13);
        assert_eq!(func(&[2, 7, 9, 4, 4]), 10);
        assert_eq!(func(&[1, 2, 3, 4, 5, 100]), 104);
        assert_eq!(func(&(1..100).collect::<Vec<_>>()), 2478);
        assert_eq!(
            func(&[
                3111, 4303, 2722, 2183, 6351, 5227, 8964, 7167, 9286, 6626, 2347, 1465, 5201, 7240,
                5463, 8523, 8163, 9391, 8616, 5063, 7837, 7050, 1246, 9579, 7744, 6932, 7704, 9841,
                6163, 4829, 7324, 6006, 4689, 8781, 621
            ]),
            112766
        );
    }
}
