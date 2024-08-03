// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/description
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let chars = Self::reverse(s.as_bytes());
        unsafe { String::from_utf8_unchecked(chars) }
    }

    pub fn reverse(s: &[u8]) -> Vec<u8> {
        let mut stack: Vec<(usize, Vec<(&[u8], usize, bool)>)> = Vec::new();
        let (mut i, mut pos, mut rev) = (0, 0, false);
        stack.push((0, vec![]));
        while i < s.len() {
            (i, pos, rev) = match s[i] {
                b'(' => {
                    stack.push((pos, vec![]));
                    (i + 1, pos, !rev)
                }
                b')' => {
                    let ((start, mut content), end) = (stack.pop().unwrap(), pos);
                    //Reverse child content
                    content
                        .iter_mut()
                        .for_each(|(substr, pos, _)| *pos = start + end - *pos - substr.len());

                    // Add to parent content
                    stack.last_mut().unwrap().1.append(&mut content);
                    (i + 1, pos, !rev)
                }
                _ => {
                    let mut end = i + 1;
                    while end < s.len() && s[end] != b'(' && s[end] != b')' {
                        end += 1;
                    }
                    stack.last_mut().unwrap().1.push((&s[i..end], pos, rev));
                    (end, pos + end - i, rev)
                }
            }
        }
        let mut out = vec![0; pos];
        for &(ss, pos, rev) in &stack[0].1 {
            for i in 0..ss.len() {
                let index = if rev { pos + ss.len() - i - 1 } else { pos + i };
                out[index] = ss[i];
            }
        }
        out
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = |s: &str| Solution::reverse_parentheses(s.to_string());
        assert_eq!(func("(ef(ab(123)))"), "ab321fe");

        assert_eq!(func("(((a)b))"), "ab");
        assert_eq!(func("((a(b)))"), "ab");
        assert_eq!(func("((ab))"), "ab");
        assert_eq!(func("(((ab)))"), "ba");
        assert_eq!(func("ab(123)"), "ab321");
        assert_eq!(func("(ab(123))"), "123ba");
        assert_eq!(func("((ab(123)))"), "ab321");
        assert_eq!(func("ef(ab(cd))"), "efcdba");
        assert_eq!(func("(ef(ab(123)))"), "ab321fe");

        assert_eq!(func("a"), "a");
        assert_eq!(func("abc"), "abc");
        assert_eq!(func("(abcd)"), "dcba");
        assert_eq!(func("(u(love)i)"), "iloveu");
        assert_eq!(func("(ed(et(oc))el)"), "leetcode");
    }
}
