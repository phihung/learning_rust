/// https://leetcode.com/problems/distinct-subsequences/
/// Given two strings s and t, return the number of distinct subsequences of s which equals t.
use std::collections::HashMap;

// Top 98%
// https://leetcode.com/problems/distinct-subsequences/solutions/5443174/simple-recursive-97-5/
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut memory: HashMap<usize, i32> = HashMap::new();
        _recursive_memorized_solution(s.as_bytes(), t.as_bytes(), &mut memory)
    }
}

fn _recursive_memorized_solution(s: &[u8], t: &[u8], memory: &mut HashMap<usize, i32>) -> i32 {
    let (len_s, len_t) = (s.len(), t.len());
    if len_t == 0 {
        return 1;
    }
    if (len_s < len_t) || (len_s == len_t && s != t) {
        return 0;
    }
    match s.iter().position(|c| *c == t[0]) {
        None => 0,
        Some(i) => {
            let key = (len_t << 10) + len_s - i;
            if let Some(&ans) = memory.get(&key) {
                return ans;
            }
            // s[i] in substring     => match s[(i+1)..] and t[1..]
            // s[i] NOT in substring => match s[(i+1)..] and t
            let n_solutions_with_i = _recursive_memorized_solution(&s[(i + 1)..], &t[1..], memory);
            let n_solutions_without_i = _recursive_memorized_solution(&s[(i + 1)..], &t, memory);
            let out = n_solutions_with_i + n_solutions_without_i;
            memory.insert(key, out);
            return out;
        }
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |s: &str, t: &str| Solution::num_distinct(s.to_string(), t.to_string());
    assert_eq!(func("rabbbit", "rabbit"), 3);
    assert_eq!(func("aabb", "ab"), 4);
    assert_eq!(func("babgbag", "bag"), 5);
}
