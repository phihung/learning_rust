/// https://leetcode.com/problems/basic-calculator/description/
/// Basic Calculator supporting + - ( )

// Top 100%
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.replace(" ", "");
        let tokens = tokenize(&s);
        evaluate(&tokens)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperatorToken {
    Plus,
    Minus,
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Num(i32),
    Operator(OperatorToken),
}

use OperatorToken::*;
use Token::*;

// -1-(-2+3) => [0,-,1,-,(,0,-,2,+,3,)]
fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut prev = None;
    let mut cnt = 0;
    let str_to_token = |c| match c {
        '+' => Plus,
        '-' => Minus,
        '(' => Open,
        ')' => Close,
        _ => unreachable!(),
    };

    for (i, c) in s.chars().enumerate() {
        // assert!(!c.is_whitespace());
        if c == '+' || c == '-' || c == '(' || c == ')' {
            if cnt > 0 {
                let num = s[(i - cnt)..i].parse::<i32>().unwrap();
                tokens.push(Num(num));
                cnt = 0;
            } else if c == '-' && prev != Some(')') {
                // '-' not preceded by a number
                tokens.push(Num(0));
            }
            tokens.push(Operator(str_to_token(c)));
        } else {
            cnt += 1;
        }
        prev = Some(c);
    }
    if cnt > 0 {
        let num = s[(s.len() - cnt)..].parse::<i32>().unwrap();
        tokens.push(Num(num));
    }
    tokens
}

fn evaluate(tokens: &[Token]) -> i32 {
    let mut stack: Vec<(Option<i32>, Option<OperatorToken>)> = vec![(None, None)];
    let reduce = |operator: Option<OperatorToken>, left: Option<i32>, value: i32| match operator {
        None => value,
        Some(Plus) => left.unwrap() + value,
        Some(Minus) => left.unwrap() - value,
        _ => unreachable!(),
    };

    for &tk in tokens {
        match tk {
            Operator(operator) => match operator {
                Plus | Minus => stack.last_mut().unwrap().1 = Some(operator),
                Open => stack.push((None, None)),
                Close => {
                    let value = stack.pop().unwrap().0.unwrap();
                    let (left, operator) = stack.pop().unwrap();
                    let result = reduce(operator, left, value);
                    stack.push((Some(result), None));
                }
            },
            Num(value) => {
                let (left, operator) = stack.pop().unwrap();
                let result = reduce(operator, left, value);
                stack.push((Some(result), None));
            }
        };
    }
    stack.pop().unwrap().0.unwrap()
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let func = |x: &str| Solution::calculate(x.to_string());
    assert_eq!(func("1 + 2"), 3);
    assert_eq!(func("1 - 2+3"), 2);
    assert_eq!(func("1 - (2+3)"), -4);
    assert_eq!(func("-(2+3)"), -5);
    assert_eq!(func("-1"), -1);
    assert_eq!(func("-1-2"), -3);
    assert_eq!(func("(1+(4+5+2)-3)+(6+8)"), 23);
}
