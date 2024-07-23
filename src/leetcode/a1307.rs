/// https://leetcode.com/problems/verbal-arithmetic-puzzle/

// https://leetcode.com/problems/verbal-arithmetic-puzzle/solutions/5477715/simple-recursive-backtracking-100-0-ms
impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let (coefs, not_zeros) = Self::to_equation(&words, &result);
        Self::is_solvable_recursive(&coefs, &not_zeros, &mut [false; 10], &mut [0; 10], 0, 0)
    }

    fn is_solvable_recursive(
        coefs: &[i32],
        not_zeros: &[bool],
        digit_used: &mut [bool],
        value: &mut [i32],
        sum: i32,
        i_var: usize,
    ) -> bool {
        let n_vars = coefs.len();
        if i_var >= n_vars {
            return sum == 0;
        }

        // Replace all variables with positive coefs with 9, negative coefs with 0
        let max: i32 = coefs[i_var..]
            .iter()
            .map(|&c| if c > 0 { 9 * c } else { 0 })
            .sum();

        // Replace all variables with positive coefs with 0, negative coefs with 9
        let min: i32 = coefs[i_var..]
            .iter()
            .map(|&c| if c > 0 { 0 } else { 9 * c })
            .sum();

        if (sum + max < 0) || (sum + min > 0) {
            return false;
        }

        let start = if not_zeros[i_var] { 1 } else { 0 };
        let old_value = value[i_var];
        for digit in start..10 {
            if !digit_used[digit] {
                value[i_var] = digit as i32;
                digit_used[digit] = true;
                let new_sum = sum + coefs[i_var] * (digit as i32);
                if Self::is_solvable_recursive(
                    coefs,
                    not_zeros,
                    digit_used,
                    value,
                    new_sum,
                    i_var + 1,
                ) {
                    return true;
                }
                digit_used[digit] = false;
            }
        }
        value[i_var] = old_value;
        false
    }

    fn to_equation(words: &[String], result: &str) -> (Vec<i32>, Vec<bool>) {
        let mut coefs = [(0, false); 26];

        let mut parse = |w: &str, unit: i32| {
            let mut unit = unit;
            for (i, &c) in w.as_bytes().iter().rev().enumerate() {
                let i_var = (c - ('A' as u8)) as usize;
                coefs[i_var].0 += unit;
                if i == w.len() - 1 && w.len() > 1 {
                    coefs[i_var].1 = true;
                }
                unit *= 10;
            }
        };

        for w in words {
            parse(w, 1);
        }
        parse(&result, -1);

        let mut coefs: Vec<_> = coefs.iter().filter(|x| x.0 != 0).collect();
        coefs.sort_by(|a, b| b.0.abs().cmp(&a.0.abs()));
        (
            coefs.iter().map(|x| x.0).collect(),
            coefs.iter().map(|x| x.1).collect(),
        )
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |words: &[&str], result: &str| {
        Solution::is_solvable(
            words.iter().map(|x| x.to_string()).collect(),
            result.to_string(),
        )
    };
    assert!(func(&["SEND", "MORE"], "MONEY"));
    assert!(func(&["SIX", "SEVEN", "SEVEN"], "TWENTY"));
    assert!(!func(&["LEET", "CODE"], "POINT"));
    assert!(func(&["A", "B"], "A"));
}
