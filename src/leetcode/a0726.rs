/// https://leetcode.com/problems/number-of-atoms

const EMPTY: char = ' ';

// https://leetcode.com/problems/number-of-atoms/solutions/5480177/0ms-100-no-hashmap/
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        // [[Uppercase, Lowercase, number]]
        let mut stack: Vec<Vec<_>> = vec![vec![]];
        let formula: Vec<_> = formula.chars().collect();

        // consume a lowercase token and return the next index
        let consume_lowercase = |chars: &[char], i: usize| -> (char, usize) {
            if i < chars.len() && chars[i].is_lowercase() {
                (chars[i], i + 1)
            } else {
                (EMPTY, i)
            }
        };

        // consume digit tokens and return the next index
        let consume_number = |chars: &[char], i: usize| -> (u32, usize) {
            if i >= chars.len() {
                return (1, i);
            }
            let mut s = 0;
            for (j, &c) in chars[i..].iter().enumerate() {
                if c.is_numeric() {
                    s = s * 10 + c.to_digit(10).unwrap();
                } else {
                    return (s.max(1), i + j);
                }
            }
            return (s.max(1), chars.len());
        };

        let mut index = 0;
        while index < formula.len() {
            let c = formula[index];
            if c == '(' {
                // current = vec![];
                stack.push(vec![]);
                index += 1;
            } else if c == ')' {
                let atoms: Vec<(char, char, u32)> = stack.pop().unwrap();
                let number;
                (number, index) = consume_number(&formula, index + 1);
                for mut atom in atoms {
                    atom.2 *= number;
                    stack.last_mut().unwrap().push(atom);
                }
            } else if c.is_uppercase() {
                // upper case
                let (lowercase, number);
                (lowercase, index) = consume_lowercase(&formula, index + 1);
                (number, index) = consume_number(&formula, index);
                stack.last_mut().unwrap().push((c, lowercase, number));
            } else {
                // number or lowercase
                unreachable!();
            }
        }
        let mut atoms = stack.pop().unwrap();

        // Aggregation atoms counts and preprare output
        atoms.sort();
        let mut out = vec![];
        let (mut prev_up, mut prev_lo, mut count) = (EMPTY, EMPTY, 0);
        let mut add_output = |up: char, lo: char, cnt: u32| {
            out.push(up.to_string());
            if lo != EMPTY {
                out.push(lo.to_string());
            }
            if cnt > 1 {
                out.push(cnt.to_string());
            }
        };
        for &(up, lo, num) in &atoms {
            if (up, lo) == (prev_up, prev_lo) {
                count += num;
            } else {
                if count > 0 {
                    add_output(prev_up, prev_lo, count);
                }
                count = num;
                (prev_up, prev_lo) = (up, lo);
            }
        }
        add_output(prev_up, prev_lo, count);
        out.join("")
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |s: &str| Solution::count_of_atoms(s.to_string());
    assert_eq!(func("H2O"), "H2O");
    assert_eq!(func("Mg(OH)2"), "H2MgO2");
    assert_eq!(func("K4(ON(SO3)2)2"), "K4N2O14S4");
}
