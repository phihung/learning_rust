/// https://leetcode.com/problems/sudoku-solver/description/
/// Sudoku Solver

// Approach: backtracking. Top 100% solution
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        solve(board);
    }
}

const EMPTY: char = '.';

#[inline]
fn set_bit(mask: &mut u16, v: u8) {
    *mask |= 1 << v
}

#[inline]
fn unset_bit(mask: &mut u16, v: u8) {
    *mask ^= 1 << v
}

#[inline]
fn get_bit(mask: u16, v: u8) -> bool {
    (mask & (1 << v)) != 0
}

fn solve(board: &mut Vec<Vec<char>>) -> bool {
    let mut masks_i = [1_u16; 9];
    let mut masks_j = [1_u16; 9];
    let mut masks_b = [1_u16; 9];
    for i in 0..9 {
        for j in 0..9 {
            let b = (i / 3) * 3 + (j / 3);
            match board[i][j] {
                EMPTY => (),
                v => {
                    let v = v.to_digit(10).unwrap() as u8;
                    set_bit(&mut masks_i[i], v);
                    set_bit(&mut masks_j[j], v);
                    set_bit(&mut masks_b[b], v);
                }
            }
        }
    }
    _solve(board, 0, &mut masks_i, &mut masks_j, &mut masks_b)
}

fn _solve(
    board: &mut Vec<Vec<char>>,
    n: usize,
    masks_i: &mut [u16; 9],
    masks_j: &mut [u16; 9],
    masks_b: &mut [u16; 9],
) -> bool {
    if n == 81 {
        return true;
    }
    let (i, j) = (n / 9, n % 9);
    let b = (i / 3) * 3 + (j / 3);
    if board[i][j] != EMPTY {
        return _solve(board, n + 1, masks_i, masks_j, masks_b);
    }
    for v in 1..10 {
        let mask = masks_i[i] | masks_j[j] | masks_b[b];
        if !get_bit(mask, v) {
            set_bit(&mut masks_i[i], v);
            set_bit(&mut masks_j[j], v);
            set_bit(&mut masks_b[b], v);
            board[i][j] = char::from_digit(v as u32, 10).unwrap();
            if _solve(board, n + 1, masks_i, masks_j, masks_b) {
                return true;
            }
            unset_bit(&mut masks_i[i], v);
            unset_bit(&mut masks_j[j], v);
            unset_bit(&mut masks_b[b], v);
        }
    }
    board[i][j] = EMPTY;
    return false;
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    let board = [
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let solution = [
        ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ];
    let mut board = board.map(|x| x.to_vec()).to_vec();
    let solution = solution.map(|x| x.to_vec()).to_vec();
    Solution::solve_sudoku(&mut board);
    assert_eq!(board, solution);
}
