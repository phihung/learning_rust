// https://leetcode.com/problems/longest-cycle-in-a-graph/

// Each node has at most one outgoing edge
// Each node/edge can be part of at most one cycle
// We only need to visit each node ONCE
// We update edges array to negative value to avoid revisiting
impl Solution {
    pub fn longest_cycle(mut edges: Vec<i32>) -> i32 {
        let mut length = -1;
        let mut visit_order = -2;
        for mut i in 0..edges.len() {
            let chain_start = visit_order;
            loop {
                let next = edges[i];
                if next <= chain_start {
                    length = length.max(next - visit_order);
                    break;
                }
                if next < 0 {
                    break;
                }
                edges[i] = visit_order;
                visit_order -= 1;
                i = next as usize;
            }
        }
        length
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |e: &[i32]| Solution::longest_cycle(e.to_vec());
        assert_eq!(func(&[-1, 0, 1, 2, 3, 4]), -1);
        assert_eq!(func(&[1, 2, 0, 4, 5, 6, 3, 8, -1]), 4);
        assert_eq!(func(&[3, 3, 4, 2, 3]), 3);
        assert_eq!(func(&[2, -1, 3, 1]), -1);
        assert_eq!(func(&[-1, -1, -1, -1]), -1);
        assert_eq!(func(&[-1, 4, -1, 2, 0, 4]), -1);
        assert_eq!(
            func(&[
                49, 29, 24, 24, -1, -1, -1, 2, 23, -1, 44, 47, 52, 49, 9, 31, 40, 34, -1, 53, 33,
                -1, 2, -1, 18, 31, 0, 9, 47, 35, -1, -1, -1, -1, 4, 12, 14, 19, -1, 4, 4, 43, 25,
                11, 54, -1, 25, 39, 17, -1, 22, 44, -1, 44, 29, 50, -1, -1
            ]),
            -1
        );
    }
}
