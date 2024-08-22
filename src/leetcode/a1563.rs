// https://leetcode.com/problems/stone-game-v/
impl Solution {
    pub fn stone_game_v(mut values: Vec<i32>) -> i32 {
        let n = values.len();
        for i in 1..n {
            values[i] += values[i - 1];
        }

        fn recursive(psum: &[i32], from: usize, to: usize, memory: &mut Vec<Vec<i32>>) -> i32 {
            let (mid, sum) = if from == 0 {
                (psum[to] as f32 / 2., psum[to])
            } else {
                (
                    (psum[to] + psum[from - 1]) as f32 / 2.,
                    (psum[to] - psum[from - 1]),
                )
            };
            if to == from {
                return sum;
            }
            if memory[from][to] != -1 {
                return memory[from][to];
            }
            let mut next = 0;
            for j in from..to {
                if psum[j] as f32 <= mid {
                    next = next.max(recursive(psum, from, j, memory));
                }
                if psum[j] as f32 >= mid {
                    next = next.max(recursive(psum, j + 1, to, memory));
                }
            }
            memory[from][to] = sum + next;
            sum + next
        }
        recursive(&values, 0, values.len() - 1, &mut vec![vec![-1; n]; n]) - values[n - 1]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |v: &[i32]| Solution::stone_game_v(v.to_vec());
        assert_eq!(func(&[44, 51, 96]), 95 + 44);
        assert_eq!(func(&[6, 12, 2, 44, 51, 96]), 96);
        assert_eq!(func(&[98, 77, 24, 49, 6, 12, 2, 44, 51, 96]), 330);
        assert_eq!(func(&[1, 2, 3, 9]), 10);
        assert_eq!(func(&[1, 2, 3, 11]), 10);
        assert_eq!(func(&[10, 3, 2, 2, 2, 2]), 14);
        assert_eq!(func(&[10, 2, 2, 2, 2, 2]), 16);
        assert_eq!(func(&[6, 2, 3, 4, 5, 5]), 18);
        assert_eq!(func(&[7, 7, 7, 7, 7, 7, 7]), 28);
        assert_eq!(func(&[4]), 0);
        assert_eq!(func(&vec![1_000_000; 500]), 494000000);
    }
}
