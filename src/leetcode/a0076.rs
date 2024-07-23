/// https://leetcode.com/problems/minimum-window-substring/
/// Given two strings s and t of lengths m and n respectively, return the minimum window substring
// of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (start, end) = min_window(s.as_bytes(), t.as_bytes());
        s[start..end].to_owned()
    }
}

fn min_window(s: &[u8], t: &[u8]) -> (usize, usize) {
    let (len_s, len_t) = (s.len(), t.len());
    if len_s < len_t {
        return (0, 0);
    }
    let mut char_counts: Vec<i32> = vec![0; u8::MAX as usize];
    for &c in t {
        char_counts[c as usize] -= 1;
    }
    let mut cnt: usize = char_counts.iter().map(|&x| if x < 0 { 1 } else { 0 }).sum();

    // Find s[0..(option_end + 1)] the first substring containing t
    let option_end = s.iter().position(|&c| {
        let x = &mut char_counts[c as usize];
        *x += 1;
        if *x == 0 {
            cnt -= 1;
        }
        if cnt == 0 {
            return true;
        }
        return false;
    });
    if option_end.is_none() {
        return (0, 0);
    }

    let mut end = option_end.unwrap() + 1;
    let mut start = 0;
    let mut best = (0, end);
    while start < end {
        let c_start = s[start];

        let c_count = char_counts[c_start as usize];
        if c_count > 0 {
            // There are more c_start in the current substring than necessary
            // No need to update `end` pointer
            char_counts[c_start as usize] = c_count - 1;
        } else {
            // There just enough c_start in the current substring
            // Need to avance `end` pointer before advancing `start` pointer
            match s[end..].iter().position(|&c| c == c_start) {
                None => {
                    // No more c_start character. End the loop
                    break;
                }
                Some(j) => {
                    // Move `end` pointer and update the counts
                    for &c in &s[end..(end + j)] {
                        char_counts[c as usize] += 1;
                    }
                    end = end + j + 1;
                }
            }
        }

        // Advance `start` pointer
        start += 1;
        if best.1 - best.0 > end - start {
            best = (start, end);
            if end - start == len_t {
                return best;
            }
        }
    }
    best
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |s: &str, t: &str| Solution::min_window(s.to_string(), t.to_string());
    assert_eq!(func("cabwefgewcwaefgcf", "cae"), "cwae");
    assert_eq!(func("ADOBECODEBANC", "ABC"), "BANC");
    assert_eq!(func("ab", "b"), "b");
    assert_eq!(func("aabcc", "cab"), "abc");
    assert_eq!(func("a", "a"), "a");
    assert_eq!(func("a", "aa"), "");
}
