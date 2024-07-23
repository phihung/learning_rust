/// https://leetcode.com/problems/reducing-dishes/

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;

        satisfaction.sort();

        let mut gain = 0;
        let mut value = 0;
        for v in satisfaction.iter().rev() {
            gain += v;
            if gain < 0 {
                break;
            }
            value += gain;
        }

        value
    }
}

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |x: &[i32]| Solution::max_satisfaction(x.to_vec());
    assert_eq!(func(&[-1, -8, 0, 5, -9]), 14);
    assert_eq!(func(&[4, 3, 2]), 20);
    assert_eq!(func(&[-1, -4, -5]), 0);
}
