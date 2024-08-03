// https://leetcode.com/problems/robot-collisions/description/
impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut i2id: Vec<_> = (0..positions.len()).collect();
        i2id.sort_by_key(|&id| positions[id]);
        let mut output = vec![];
        let mut stack = vec![];
        for id in i2id {
            let (mut h, dir) = (healths[id], directions.as_bytes()[id]);
            if dir == b'R' {
                stack.push((id, h));
            } else {
                while let Some((id1, h1)) = stack.pop() {
                    if h > h1 {
                        h -= 1;
                        continue;
                    }
                    if h < h1 {
                        stack.push((id1, h1 - 1));
                    }
                    h = 0;
                    break;
                }
                if h > 0 {
                    output.push((id, h))
                }
            }
        }
        output.append(&mut stack);
        output.sort_unstable();
        output.into_iter().map(|(_, h)| h).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        run_test(|p: &[i32], h: &[i32], d: &str| {
            Solution::survived_robots_healths(p.to_vec(), h.to_vec(), d.to_string())
        });
    }

    fn run_test(func: impl Fn(&[i32], &[i32], &str) -> Vec<i32>) {
        assert_eq!(
            func(&[5, 4, 3, 2, 1], &[2, 17, 9, 15, 10], "RRRRR"),
            vec![2, 17, 9, 15, 10]
        );
        assert_eq!(func(&[3, 5, 2, 6], &[10, 10, 15, 12], "RLRL"), [14]);
        assert_eq!(
            func(&[1, 2, 5, 6], &[10, 10, 11, 11], "RLRL"),
            Vec::<i32>::new()
        );
    }
}
