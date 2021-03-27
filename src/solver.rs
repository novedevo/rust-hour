use std::collections::BinaryHeap;

use ahash::{AHashMap, AHashSet}; //10x faster than the default at the expense of some cryptographic defenses

use crate::board::Board;

//returns the solved board and a number indicating how many distinct states were checked
pub fn solve(board: Board) -> (Board, usize) {
    //wrapper
    astar(board)
}

//dfs is ~30% less iterations than bfs in this situation, for some reason
fn _dfs(board: Board) -> (Board, usize) {
    let mut visited: AHashSet<[[char; 6]; 6]> = AHashSet::new(); //keep track of all the nodes we have visited to avoid backtracking
    visited.insert(board.board_chars);

    let mut stack: Vec<Board> = Vec::new();
    stack.push(board);

    while let Some(board) = stack.pop() {
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                return (new_board, visited.len());
            } else if !visited.contains(&new_board.board_chars) {
                visited.insert(new_board.board_chars);
                stack.push(new_board);
            }
        }
    }

    panic!("at the disco!");
}

fn astar(board: Board) -> (Board, usize) {
    let mut open_set = BinaryHeap::<Board>::new();
    open_set.push(board.clone());

    let mut visited: AHashSet<[[char; 6]; 6]> = AHashSet::new();
    visited.insert(board.board_chars);

    // let mut g_score: AHashMap<Board, usize> = AHashMap::new();
    // g_score.insert(board.clone(), 0);

    // let mut f_score: AHashMap<Board, usize> = AHashMap::new();
    // f_score.insert(board.clone(), board.h);

    while let Some(current) = open_set.pop() {
        for neighbour in current.get_moves() {
            // let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if neighbour.is_solved() {
                return (current, 0);
            } else if !visited.contains(&neighbour.board_chars) {
                // g_score.insert(neighbour.clone(), tentative_g_score);
                // f_score.insert(
                //     neighbour.clone(),
                //     *g_score.get(&neighbour).unwrap() + neighbour.h,
                // );
                visited.insert(neighbour.board_chars);
                open_set.push(neighbour.clone())
            }
        }
    }
    panic!("at the astarsco")
}

fn _heuristic(_chars: [[char; 6]; 6]) -> u8 {
    _zero_heuristic()
}

fn _first_order_blocking_heuristic(chars: [[char; 6]; 6]) -> u8 {
    let mut retval = 1;
    for character in chars[2].iter().rev() {
        match character {
            'X' => break,
            '.' => continue,
            _ => retval += 1,
        }
    }
    retval
}

const fn _zero_heuristic() -> u8 {
    0
}
