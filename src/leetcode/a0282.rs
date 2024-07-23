// https://leetcode.com/problems/expression-add-operators

// 100%, 100%
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let zero = '0' as u8;
        let digits: Vec<i64> = num.as_bytes().iter().map(|x| (x - zero) as i64).collect();

        let mut out = vec![];
        let mut save_solution = |ops: &Vec<u8>| {
            let mut chars = Vec::with_capacity(digits.len() * 2);
            chars.push(digits[0] as u8 + zero);
            for (&d, &op) in digits.iter().skip(1).zip(ops.iter()) {
                if op != '@' as u8 {
                    chars.push(op as u8);
                }
                chars.push(d as u8 + zero);
            }
            out.push(String::from_utf8(chars).unwrap());
        };

        Self::recursive(
            &digits[1..],
            target as i64,
            digits[0],
            1,
            &mut vec![],
            &mut save_solution,
        );
        out
    }

    // nums is in reversed order
    fn recursive<F>(
        nums: &[i64],
        target: i64,
        prev: i64,
        mul: i64,
        current: &mut Vec<u8>,
        save_solution: &mut F,
    ) where
        F: FnMut(&Vec<u8>),
    {
        let n = nums.len();
        if n == 0 {
            if target == prev * mul {
                save_solution(current)
            }
            return;
        }

        let d = nums[0];

        current.push('+' as u8);
        Self::recursive(
            &nums[1..],
            target - prev * mul,
            d,
            1,
            current,
            save_solution,
        );

        *current.last_mut().unwrap() = '*' as u8;
        Self::recursive(&nums[1..], target, d, prev * mul, current, save_solution);

        // ("293", 32) => ([3, 9, 2], 32) => ([3, -9], 30)
        *current.last_mut().unwrap() = '-' as u8;
        Self::recursive(
            &nums[1..],
            target - prev * mul,
            -d,
            1,
            current,
            save_solution,
        );

        if prev != 0 {
            let x = if prev > 0 {
                prev * 10 + d
            } else {
                prev * 10 - d
            };
            *current.last_mut().unwrap() = '@' as u8;
            Self::recursive(&nums[1..], target, x, mul, current, save_solution);
        }

        current.pop();
    }
}

const PLUS: i32 = 0;
const MINUS: i32 = 1;
const MUL: i32 = 2;
const CONCAT: i32 = 3;

impl Solution2 {
    fn evaluate(digits: &[i64], ops: &[i32]) -> Option<i64> {
        let mut result = 0;
        let mut current = digits[0];
        let mut mul = 1;
        // 2 + 3 * 4
        for (&d, &op) in digits[1..].iter().zip(ops) {
            (result, current, mul) = match op {
                PLUS => (result + current * mul, d, 1),   // +
                MINUS => (result + current * mul, -d, 1), // -
                MUL => (result, d, mul * current),        // *
                CONCAT => {
                    if current != 0 {
                        if current > 0 {
                            (result, current * 10 + d, mul)
                        } else {
                            (result, current * 10 - d, mul)
                        }
                    } else {
                        return None;
                    }
                } // concat
                _ => unreachable!(),
            };
        }
        Some(result + current * mul)
    }

    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let zero = '0' as u8;
        let target = target as i64;

        let digits: Vec<i64> = num.as_bytes().iter().map(|x| (x - zero) as i64).collect();
        if digits.len() == 1 {
            return if digits[0] == target {
                vec![num]
            } else {
                vec![]
            };
        }
        let mut out = vec![];
        Self::carterian(4, digits.len() - 1, &mut |ops, _j| {
            let v = Self::evaluate(&digits, ops);
            if v == Some(target) {
                // println!("{:?}", ops);
                let mut chars = Vec::with_capacity(digits.len() * 2);
                chars.push(digits[0] as u8 + zero);
                for (&d, &op) in digits[1..].iter().zip(ops) {
                    if op != CONCAT {
                        chars.push(['+', '-', '*'][op as usize] as u8);
                    }
                    chars.push(d as u8 + zero);
                }
                out.push(String::from_utf8(chars).unwrap());
            }
            true
        });
        out
    }

    fn carterian<F>(n: i32, k: usize, func: &mut F)
    where
        F: FnMut(&Vec<i32>, usize) -> bool,
    {
        let mut value = vec![0; k];
        value[k - 1] = -1;
        for _ in 0..n.pow(k as u32) {
            let mut j = k - 1;
            loop {
                value[j] += 1;
                if value[j] < n {
                    break;
                }
                value[j] = 0;
                j -= 1;
            }
            if !func(&value, j) {
                break;
            }
        }
    }
}

pub struct Solution {}
pub struct Solution2 {}

#[test]
fn test_solution() {
    run_test(Solution::add_operators);
}

#[test]
fn test_solution2() {
    run_test(Solution2::add_operators);
}

#[cfg(test)]
fn run_test<F>(func: F)
where
    F: Fn(String, i32) -> Vec<String>,
{
    use std::collections::HashSet;
    let test = |num: &str, target: i32, expect: &[&str]| {
        assert_eq!(
            HashSet::from_iter(func(num.to_string(), target)),
            expect.iter().map(|x| x.to_string()).collect::<HashSet<_>>()
        );
    };
    test("293", 32, &["29+3"]);
    test(
        "010",
        0,
        &["0*1*0", "0*1+0", "0*1-0", "0*10", "0+1*0", "0-1*0"],
    );
    test("3456237490", 9191, &[]);
    test("2147483648", -2147483648, &[]);

    test("1009", 9, &["1*0+0+9", "1*0*0+9", "1*0-0+9", "10*0+9"]);
    test(
        "10009",
        9,
        &[
            "1*0-0+0+9",
            "1*0-0-0+9",
            "1*0*0+0+9",
            "1*0*0-0+9",
            "10*0+0+9",
            "10*0*0+9",
            "1*0+0*0+9",
            "10*0-0+9",
            "1*0+0-0+9",
            "1*0*0*0+9",
            "1*0-0*0+9",
            "100*0+9",
            "1*0+0+0+9",
        ],
    );
    test(
        "000",
        0,
        &[
            "0+0+0", "0*0-0", "0+0*0", "0+0-0", "0-0+0", "0-0*0", "0-0-0", "0*0+0", "0*0*0",
        ],
    );
    test("203", 6, &[]);

    test("110", 110, &["110"]);
    test("1", 1, &["1"]);
    test("123", 123, &["123"]);
    test("123", 15, &["12+3"]);
    test("123", 6, &["1*2*3", "1+2+3"]);
    // println!("{:?}", Solution::add_operators("1000000009".to_string(), 9));
    assert_eq!(
        Solution::add_operators("1000000009".to_string(), 9).len(),
        3280
    );
}

#[test]
fn test_evaluate() {
    let func = |a, b| Solution2::evaluate(a, b).unwrap_or(i64::MIN);
    assert_eq!(func(&[0, 1, 0], &[MUL, CONCAT]), 0);
    assert_eq!(
        func(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[CONCAT, CONCAT, PLUS, MINUS, MINUS, MINUS, CONCAT, MINUS]
        ),
        29
    );
    assert_eq!(
        func(
            &[3, 4, 5, 6, 2, 3, 7, 4, 9, 0],
            &[PLUS, CONCAT, CONCAT, MUL, CONCAT, MINUS, MUL, PLUS, CONCAT]
        ),
        10553
    );
    assert_eq!(func(&[1, 0, 0, 9], &[MUL, CONCAT, PLUS]), i64::MIN);
    assert_eq!(func(&[1, 2, 3], &[MUL, PLUS]), 5);
    assert_eq!(func(&[1, 2, 3], &[PLUS, PLUS]), 6);
    assert_eq!(func(&[1, 2, 3], &[PLUS, MUL]), 7);
    assert_eq!(func(&[1, 2, 3], &[MINUS, PLUS]), 2);
    assert_eq!(func(&[1, 2, 3], &[PLUS, MINUS]), 0);
    assert_eq!(func(&[1, 2, 3], &[PLUS, CONCAT]), 24);
    assert_eq!(func(&[1, 2, 3], &[CONCAT, CONCAT]), 123);
    assert_eq!(func(&[1, 0, 0, 2], &[CONCAT, PLUS, MUL]), 10);
}
