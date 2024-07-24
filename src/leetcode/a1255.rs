// https://leetcode.com/problems/maximum-score-words-formed-by-letters/description/
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let ctoi = |c: char| (c as u8 - b'a') as usize;
        let u8toi = |c: u8| (c - b'a') as usize;

        let mut counts: [u8; 26] = {
            let mut out = [0; 26];
            for c in letters {
                out[ctoi(c)] += 1;
            }
            out
        };

        let chars_n_scores: Vec<(Vec<(usize, u8)>, i32)> = {
            let mut ls = vec![];
            'outer: for w in words {
                let mut cnts = [0; 26];
                for &c in w.as_bytes() {
                    let i = u8toi(c);
                    cnts[i] += 1;
                }
                let mut cnt_list = vec![];
                let mut sc = 0;
                for (i, cnt) in cnts.into_iter().enumerate() {
                    if cnt > 0 {
                        cnt_list.push((i, cnt));
                        sc += score[i] * (cnt as i32);
                    }
                    if cnt > counts[i] {
                        continue 'outer;
                    }
                }
                ls.push((cnt_list, sc));
            }
            ls.sort_by_key(|(_, sc)| -sc);
            ls
        };

        // --- Back tracking ---

        enum Action {
            Check(usize),
            Remove(usize),
        }

        let can_form_word = |idx: usize, counts: &[u8; 26]| {
            chars_n_scores[idx]
                .0
                .iter()
                .all(|&(i, cnt)| cnt <= counts[i])
        };
        let remove_word = |idx: usize, counts: &mut [u8; 26], current_score: &mut i32| {
            let (w_counts, sc) = &chars_n_scores[idx];
            w_counts.iter().for_each(|&(i, cnt)| counts[i] += cnt);
            *current_score -= *sc;
        };
        let add_word = |idx: usize, counts: &mut [u8; 26], current_score: &mut i32| {
            let (w_counts, sc) = &chars_n_scores[idx];
            w_counts.iter().for_each(|&(i, cnt)| counts[i] -= cnt);
            *current_score += *sc;
        };

        let n = chars_n_scores.len();
        let mut max_score = 0;
        let mut current_score = 0;
        let mut stack = vec![];
        for idx in (0..n).rev() {
            if can_form_word(idx, &counts) {
                stack.push(Action::Check(idx));
            }
        }
        while let Some(action) = stack.pop() {
            match action {
                Action::Check(word_idx) => {
                    if word_idx == n {
                        max_score = max_score.max(current_score);
                        continue;
                    }
                    add_word(word_idx, &mut counts, &mut current_score);
                    stack.push(Action::Remove(word_idx));

                    let mut sc = 0;
                    for idx in ((word_idx + 1)..n).rev() {
                        if can_form_word(idx, &counts) {
                            sc += chars_n_scores[idx].1;
                            if sc + current_score > max_score {
                                stack.push(Action::Check(idx));
                            }
                        }
                    }
                    stack.push(Action::Check(n));
                }
                Action::Remove(word_idx) => remove_word(word_idx, &mut counts, &mut current_score),
            }
        }
        max_score
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let func = |w: &[&str], l: &[char], s: &[i32]| {
        Solution::max_score_words(
            w.into_iter().map(|s| s.to_string()).collect(),
            l.to_vec(),
            s.to_vec(),
        )
    };

    assert_eq!(
        func(
            &["ac", "abd", "db", "ba", "dddd", "bca"],
            &['a', 'a', 'a', 'b', 'b', 'b', 'c', 'c', 'd', 'd', 'd', 'd'],
            &[6, 4, 4, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        62
    );
    assert_eq!(
        func(
            &["azb", "ax", "awb", "ayb", "bpppp"],
            &['z', 'a', 'w', 'x', 'y', 'b', 'p', 'p', 'p'],
            &[10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 3, 2, 3, 3]
        ),
        14
    );
    assert_eq!(
        func(
            &["dog", "cat", "dad", "good"],
            &['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            &[1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        23
    );
    assert_eq!(
        func(
            &["xxxz", "ax", "bx", "cx"],
            &['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            &[4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
        ),
        27
    );
    assert_eq!(
        func(
            &["leetcode"],
            &['l', 'e', 't', 'c', 'o', 'd'],
            &[0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
        ),
        0
    );
}
