/// https://leetcode.com/problems/max-points-on-a-line/
/// return the maximum number of points that lie on the same straight line

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n < 3 {
            return n as i32;
        }

        let mut coefs = vec![0.; n];
        let mut result = 0;
        for (i1, p1) in points.iter().enumerate() {
            let (x1, y1) = (p1[0], p1[1]);
            let mut num = 0;
            for p2 in &points[(i1 + 1)..] {
                let (x2, y2) = (p2[0], p2[1]);
                let slope = if x1 == x2 {
                    f32::MAX
                } else {
                    (y2 - y1) as f32 / (x2 - x1) as f32
                };
                coefs[num] = slope;
                num += 1;
            }
            coefs[..num].sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut count = 0;
            let mut prev = f32::MIN;
            for &v in &coefs[..num] {
                if v == prev {
                    count += 1;
                } else {
                    result = result.max(count);
                    prev = v;
                    count = 2;
                }
            }
            result = result.max(count);
        }
        result
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func =
        |points: &[[i32; 2]]| Solution::max_points(points.iter().map(|x| x.to_vec()).collect());
    assert_eq!(
        func(&[[0, 0], [4, 5], [7, 8], [8, 9], [5, 6], [3, 4], [1, 1]]),
        5
    );
    assert_eq!(
        func(&[[4, 5], [0, 0], [7, 8], [8, 9], [5, 6], [3, 4], [1, 1]]),
        5
    );
    assert_eq!(func(&[[2, 3], [3, 3], [-5, 3]]), 3);
    assert_eq!(func(&[[1, 2]]), 1);
    assert_eq!(func(&[[1, 2], [10, -9]]), 2);
    assert_eq!(func(&[[1, 1], [2, 2], [3, 3]]), 3);
    assert_eq!(func(&[[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]), 4);
}
