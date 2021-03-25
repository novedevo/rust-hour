use std::collections::{HashSet, VecDeque};

use crate::board::Board;

pub fn solve(board: Board) -> (Board, usize) {
    dfs(board)
}

fn dfs(board: Board) -> (Board, usize) {
    let mut visited: HashSet<Board> = HashSet::new();
    visited.insert(board.clone());

    let mut stack: VecDeque<Board> = VecDeque::new();
    stack.push_back(board);

    while let Some(board) = stack.pop_back() {
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                // eprintln!("{}", visited.len());
                return (new_board, visited.len());
            } else if !visited.contains(&new_board) {
                visited.insert(new_board.clone());
                stack.push_back(new_board);
            }
        }
    }

    panic!("at the disco!");
}
