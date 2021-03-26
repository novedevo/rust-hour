// use std::collections::{HashSet, VecDeque};
use ahash::AHashSet;

use crate::board::Board;

pub fn solve(board: Board) -> (Board, usize) {
    dfs(board)
}

fn dfs(board: Board) -> (Board, usize) {
    let mut visited: AHashSet<Board> = AHashSet::new();
    visited.insert(board.clone());

    let mut stack: Vec<Board> = Vec::new();
    stack.push(board);

    while let Some(board) = stack.pop() {
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                // eprintln!("{}", visited.len());
                return (new_board, visited.len());
            } else if !visited.contains(&new_board) {
                visited.insert(new_board.clone());
                stack.push(new_board);
            }
        }
    }

    panic!("at the disco!");
}
