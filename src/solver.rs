use std::collections::{HashSet};

use crate::board::Board;

pub fn solve(board: Board) -> Board {
    bfs(board)
}

fn bfs(board: Board) -> Board {
    let mut visited: HashSet<Board> = HashSet::<Board>::new();
    visited.insert(board.clone());

    let mut queue: Vec<Board> = Vec::new();
    queue.push(board);

    while !queue.is_empty() {
        let board = queue.pop().unwrap();
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                return new_board;
            } else if !visited.contains(&new_board) {
                visited.insert(new_board.clone());
                queue.push(new_board);
            }
        }
    }

    panic!("at the disco!");
}
