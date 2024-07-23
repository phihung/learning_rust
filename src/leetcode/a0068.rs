/// https://leetcode.com/problems/text-justification/description/

// Top 100%
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut out: Vec<String> = vec![];
        let (mut line, mut length) = (vec![], 0);
        let spaces = String::from_utf8(vec![b' '; max_width]).unwrap();
        for word in &words {
            let w_len = word.chars().count();
            if length + w_len + line.len() > max_width {
                out.push(Self::concat(line, max_width - length, false, &spaces));
                (line, length) = (vec![], 0);
            }
            line.push(word);
            length += w_len;
        }
        out.push(Self::concat(line, max_width - length, true, &spaces));
        out
    }

    fn concat<'a>(words: Vec<&'a str>, n_spaces: usize, is_last: bool, spaces: &'a str) -> String {
        let n_words = words.len();
        if is_last || words.len() == 1 {
            if n_spaces < n_words {
                words.join(" ")
            } else {
                let mut words = words;
                words.push(&spaces[..n_spaces - n_words]);
                words.join(" ")
            }
        } else {
            let q = n_spaces / (n_words - 1) as usize;
            let r = n_spaces % (n_words - 1);
            let mut ls = Vec::with_capacity(2 * n_words - 1);
            ls.push(words[0]);
            for (i, &w) in words[1..].iter().enumerate() {
                if i < r {
                    ls.push(&spaces[..(q + 1)]);
                } else {
                    ls.push(&spaces[..q]);
                }
                ls.push(w);
            }
            ls.join("")
        }
    }
}

// ---- test ----

pub struct Solution {}

#[test]
#[rustfmt::skip]
fn test_solution() {
    let func = |words: &[&str], max_width| {
        Solution::full_justify(words.iter().map(|&x| x.to_string()).collect(), max_width)
    };

    let words = ["This", "is", "an", "example", "of", "text", "justification."];
    let expected = ["This    is    an", "example  of text", "justification.  "];
    assert_eq!(func(&words, 16), expected);

    let words = ["What","must","be","acknowledgment","shall","be"];
    let expected = ["What   must   be", "acknowledgment  ", "shall be        "];
    assert_eq!(func(&words, 16), expected);

    let words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"];
    let expected = [
        "Science  is  what we",
        "understand      well",
        "enough to explain to",
        "a  computer.  Art is",
        "everything  else  we",
        "do                  "
    ];
    assert_eq!(func(&words, 20), expected);

    let words = ["ask   not   what", "your country can", "do  for  you ask", "what  you can do", "for your country"];
    let expected = ["ask   not   what", "your country can", "do  for  you ask", "what  you can do", "for your country"];
    assert_eq!(func(&words, 16), expected);
}
