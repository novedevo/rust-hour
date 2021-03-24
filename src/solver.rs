use std::collections::{BinaryHeap, HashSet};

use crate::board::Board;

pub fn solve(board: Board) -> Board{
    bfs(board)
}

fn bfs(board:Board) -> Board {
    let mut visited: HashSet<Board> = HashSet::<Board>::with_capacity(4000);
    visited.insert(board.clone());
    
    let mut queue: BinaryHeap<Board> = BinaryHeap::with_capacity(1000);
    queue.push(board);
    
    while !queue.is_empty() {
        let board = queue.pop().unwrap();
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                return new_board;
            }
            else if !visited.contains(&new_board) {
                visited.insert(new_board.clone());
                queue.push(new_board);
            }
        }
    }
    
    panic!("at the disco!");
}