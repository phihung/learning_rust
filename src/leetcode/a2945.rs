// https://leetcode.com/problems/find-maximum-non-decreasing-array-length/

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    potential: i32,
    i_start: usize,
    length: i32,
    v_min: i64,
}

impl Solution {
    pub fn find_maximum_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let cumsums = Self::to_cumsum(&nums);

        let mut iter = 0;
        let mut best = Self::greedy(&cumsums, 0);
        let mut visited = vec![None; n];
        let mut queue = BinaryHeap::with_capacity(n * 3 / 2);
        queue.push(Node {
            potential: i32::MAX,
            i_start: 0,
            length: 0,
            v_min: 1,
        });

        let is_userfull = |x: &Node, eq_ok: bool, visited: &[_], best: i32| {
            if x.potential <= best {
                return false;
            }
            // same node => true
            if let Some((l1, v1)) = visited[x.i_start] {
                if (v1 < x.v_min && l1 >= x.length) || (v1 == x.v_min && l1 > x.length) {
                    // There is a better node in the queue
                    return false;
                }
                if v1 == x.v_min && l1 == x.length {
                    return eq_ok;
                }
            }
            return true;
        };

        // Priority queue. Process first: high potential - high i_start (potential estimation is more fiable)
        while let Some(node) = queue.pop() {
            if !is_userfull(&node, true, &visited, best) {
                continue;
            }

            iter += 1;
            if iter % 1000 == 0 {
                queue.retain(|x| is_userfull(x, true, &visited, best));
            }

            let v_prev = if node.i_start > 0 {
                cumsums[node.i_start - 1]
            } else {
                0
            };
            for (i, &x) in cumsums[node.i_start..].iter().enumerate() {
                let i = node.i_start + i;
                let v = x - v_prev;
                if v >= node.v_min {
                    if i < n - 1 {
                        let potential =
                            node.length + 1 + ((cumsums[n - 1] - cumsums[i]) / v) as i32;
                        let new_node = Node {
                            potential,
                            i_start: i + 1,
                            length: node.length + 1,
                            v_min: v,
                        };
                        if !is_userfull(&new_node, false, &visited, best) {
                            break;
                        }

                        queue.push(new_node);
                        visited[i + 1] = Some((node.length + 1, v));
                    } else if node.length + 1 > best {
                        best = node.length + 1;
                        queue.retain(|x| x.potential > best);
                    }
                }
            }
        }
        best
    }

    fn greedy(cumsums: &[i64], min: i64) -> i32 {
        let mut min = min;
        let mut count: i32 = 0;
        let mut prev = 0;
        for &s in cumsums {
            let v = s - prev;
            if v >= min {
                min = v;
                prev = s;
                count += 1;
            }
        }
        count
    }

    fn to_cumsum(nums: &[i32]) -> Vec<i64> {
        let mut sum: i64 = 0;
        let mut cumsums = Vec::with_capacity(nums.len());
        for &x in nums {
            sum += x as i64;
            cumsums.push(sum);
        }
        return cumsums;
    }
}

//  ---- Test ----

pub struct Solution {}

#[cfg(test)]
use std::error::Error;

#[cfg(test)]
use std::fs;

#[cfg(test)]
use std::path::Path;

#[test]
fn test_small() {
    assert_eq!(
        Solution::find_maximum_length(vec![417, 241, 895, 308, 259, 562]),
        3
    );
    assert_eq!(Solution::find_maximum_length(vec![2, 1, 2, 1, 2, 1, 2]), 4);
    assert_eq!(Solution::find_maximum_length(vec![5, 2, 2]), 1);
    assert_eq!(Solution::find_maximum_length(vec![1, 2, 3, 4]), 4);
    assert_eq!(Solution::find_maximum_length(vec![4, 3, 2, 6]), 3);
}

#[test]
fn test_40k() -> Result<(), Box<dyn Error>> {
    let path = Path::new("data/leetcode/2945/40k.json");
    if !path.exists() {
        return Ok(());
    }
    let v: Vec<i32> = serde_json::from_str(&fs::read_to_string(path)?)?;
    run_test(v, 412, 2.);
    Ok(())
}

#[test]
fn test_100k() -> Result<(), Box<dyn Error>> {
    let path = Path::new("data/leetcode/2945/100k.json");
    if !path.exists() {
        return Ok(());
    }
    let v: Vec<i32> = serde_json::from_str(&fs::read_to_string(path)?)?;
    run_test(v, 664, 2.);
    Ok(())
}

#[test]
fn test_550() -> Result<(), Box<dyn Error>> {
    let path = Path::new("data/leetcode/2945/550.json");
    if !path.exists() {
        return Ok(());
    }
    let v: Vec<i32> = serde_json::from_str(&fs::read_to_string(path)?)?;
    run_test(v, 624, 2.);
    Ok(())
}

#[cfg(test)]
fn run_test(input: Vec<i32>, expected: i32, max_duration: f32) {
    let now = std::time::SystemTime::now();
    let output = Solution::find_maximum_length(input);
    let duration = now.elapsed().expect("Error");

    assert_eq!(output, expected);
    assert!(
        duration.as_secs_f32() < max_duration,
        "Time limit exceeded: {duration:?}"
    );
}
