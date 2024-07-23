// https://leetcode.com/problems/word-search-ii/description/

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

const A_U8: u8 = 'a' as u8;

// 100%, 19ms
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut words = words;
        Self::remove_impossible_words(&mut words, &board);

        let trie = Trie::new();
        for w in words {
            trie.insert(w);
        }

        let mut board = board;
        let mut solutions = Vec::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::backtracking(&mut board, &trie, &mut solutions, i, j);
            }
        }

        solutions
    }

    fn backtracking(
        board: &mut Vec<Vec<char>>,
        trie: &Rc<Trie>,
        solutions: &mut Vec<String>,
        i: usize,
        j: usize,
    ) {
        let ch = board[i][j];
        if ch == 0 as char {
            return;
        }

        let trie = match trie.get(ch) {
            None => return,
            Some(next) => next,
        };

        if let Some(word) = trie.take_word() {
            solutions.push(word);
        }

        board[i][j] = 0 as char;
        let (n_row, n_col) = (board.len(), board[0].len());
        if i > 0 {
            Self::backtracking(board, &trie, solutions, i - 1, j);
        }
        if i < n_row - 1 {
            Self::backtracking(board, &trie, solutions, i + 1, j);
        }
        if j > 0 {
            Self::backtracking(board, &trie, solutions, i, j - 1);
        }
        if j < n_col - 1 {
            Self::backtracking(board, &trie, solutions, i, j + 1);
        }
        board[i][j] = ch;
    } // fn recursive(board: &[u8], )

    // filter out the words that cannot be found doe to missing characters
    fn remove_impossible_words(words: &mut Vec<String>, board: &Vec<Vec<char>>) {
        let mut board_freq = [0; 26];
        for r in 0..board.len() {
            for c in 0..board[r].len() {
                board_freq[(board[r][c] as u8 - A_U8) as usize] += 1;
            }
        }

        words.retain(|w| {
            let mut word_freq = [0; 26];
            for ch in w.as_bytes().iter().copied().map(|ch| (ch - A_U8) as usize) {
                word_freq[ch] += 1;
            }

            board_freq
                .iter()
                .zip(word_freq.iter())
                .all(|(a, b)| *a >= *b)
        });
    }
}

#[derive(Default)]
pub struct TrieNode {
    word: Option<String>,
    count: usize,
    nexts: [Option<Rc<Trie>>; 26],
    parent: Option<Rc<Trie>>,
}

pub struct Trie(RefCell<TrieNode>);

impl Trie {
    fn new() -> Rc<Self> {
        Rc::new(Trie(RefCell::new(TrieNode::default())))
    }

    fn insert(self: &Rc<Self>, w: String) {
        let mut current = self.clone();
        current.borrow_mut().count += 1;

        for &c in w.as_bytes() {
            let index = (c - A_U8) as usize;
            let next = {
                let mut node = current.borrow_mut();
                if node.nexts[index].is_none() {
                    node.nexts[index] = Some(Trie::new());
                }
                node.nexts[index].clone().unwrap()
            };

            {
                let mut next_borrowed = next.borrow_mut();
                next_borrowed.count += 1;
                next_borrowed.parent = Some(current.clone());
            }

            current = next;
        }

        current.borrow_mut().word = Some(w);
    }

    fn get(self: &Rc<Self>, c: char) -> Option<Rc<Trie>> {
        let i = c as u8 - A_U8;
        let trie = self.borrow().nexts[i as usize].clone();
        match trie {
            Some(x) if x.borrow().count > 0 => Some(x),
            _ => None,
        }
    }

    fn take_word(self: &Rc<Self>) -> Option<String> {
        if self.borrow().word.is_none() {
            return None;
        }
        let mut t = self.clone();
        loop {
            t.borrow_mut().count -= 1;
            let tt = match t.borrow().parent.clone() {
                None => break,
                Some(t) => t,
            };
            t = tt;
        }
        self.borrow_mut().word.take()
    }
}

impl Deref for Trie {
    type Target = RefCell<TrieNode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Trie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    use std::collections::HashSet;
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
        &["oabn", "otae", "ahkr", "aflv"],
        &["oa", "oaa"],
        &["oa", "oaa"],
    );
    test(
        &["oaan", "etae", "ihkr", "iflv"],
        &["oath", "pea", "eat", "rain"],
        &["eat", "oath"],
    )
}
