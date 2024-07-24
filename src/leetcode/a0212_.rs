// https://leetcode.com/problems/word-search-ii/description/

use std::collections::{HashMap, HashSet};

const A_U8: u8 = 'a' as u8;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let shape = (board.len(), board[0].len());
        let words_int: HashSet<_> = words.iter().map(|s| Self::str_to_int(s)).collect();
        let mut prefixes = HashMap::new();
        for w in words {
            for prefix in Self::get_prefixes(&w, usize::MAX) {
                if let Some(cnt) = prefixes.get_mut(&prefix) {
                    *cnt += 1;
                } else {
                    prefixes.insert(prefix, 1);
                }
            }
        }
        // let prefixes: HashSet<_> = words
        //     .iter()
        //     .flat_map(|s| Self::get_prefixes(s, usize::MAX))
        //     .collect();
        let board: Vec<i64> = board
            .into_iter()
            .flat_map(|row| row.into_iter().map(|c| ((c as u8) - A_U8 + 1) as i64))
            .collect();

        let mut solution = HashSet::new();
        let mut used = vec![false; board.len()];
        for pos in 0..board.len() {
            assert!(!used.iter().any(|x| *x));
            Self::backtracking(
                &board,
                shape,
                pos,
                0,
                &mut used,
                &mut prefixes,
                &words_int,
                &mut solution,
            );
        }

        solution.into_iter().map(Self::int_to_str).collect()
    }

    fn backtracking(
        board: &Vec<i64>,
        shape: (usize, usize),
        pos: usize,
        current: i64,
        used: &mut [bool],
        prefixes: &mut HashMap<i64, usize>,
        dict: &HashSet<i64>,
        solutions: &mut HashSet<i64>,
    ) {
        let current = (current << 5) | board[pos];
        if dict.contains(&current) && !solutions.contains(&current) {
            let mut v = current;
            loop {
                v = v >> 5;
                if v <= 0 {
                    break;
                }
                *prefixes.get_mut(&v).unwrap() -= 1;
            }
            solutions.insert(current);
        }
        if prefixes.get(&current).unwrap_or(&0) == &0 {
            return;
        }

        used[pos] = true;
        let (n_row, n_col) = shape;
        let (i, j) = (pos / n_col, pos % n_col);
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
            if new_i < 0 || new_i >= n_row as i32 || new_j < 0 || new_j >= n_col as i32 {
                continue;
            }

            let next_pos = (new_i as usize) * n_col + new_j as usize;
            let next_pos = next_pos as usize;
            if !used[next_pos] {
                Self::backtracking(
                    board, shape, next_pos, current, used, prefixes, dict, solutions,
                );
            }
        }
        used[pos] = false;
    }

    fn str_to_int(s: &str) -> i64 {
        let mut out = 0;
        for &c in s.as_bytes() {
            out = (out << 5) | ((c - A_U8 + 1) as i64);
        }
        out
    }

    fn int_to_str(v: i64) -> String {
        let mut v = v;
        let mut chars = Vec::with_capacity(10);
        while v > 0 {
            chars.push((v % 32) as u8 - 1 + A_U8);
            v >>= 5;
        }
        chars.reverse();
        String::from_utf8(chars).unwrap()
    }

    fn get_prefixes(s: &str, max_len: usize) -> Vec<i64> {
        let n = max_len.min(s.len() - 1);
        let mut out = Vec::with_capacity(n);
        let mut v = 0;
        for &c in s[..n].as_bytes() {
            v = (v << 5) | ((c - A_U8 + 1) as i64);
            out.push(v);
        }
        out
    } // fn recursive(board: &[u8], )
}

pub struct Solution;

#[test]
fn test_solution() {
    let test = |b: &[&str], words: &[&str], expect: &[&str]| {
        let actual = Solution::find_words(
            b.iter().map(|x| x.chars().collect()).collect(),
            words.iter().map(|x| x.to_string()).collect(),
        );
        assert_eq!(
            actual.into_iter().collect::<HashSet<_>>(),
            HashSet::from_iter(expect.iter().map(|x| x.to_string()))
        );
    };
    test(
        &["oaan", "etae", "ihkr", "iflv"],
        &["oath", "pea", "eat", "rain"],
        &["eat", "oath"],
    )
}
