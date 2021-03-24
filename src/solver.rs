use std::collections::{BinaryHeap, HashSet, binary_heap};

use crate::board::Board;

pub fn solve(board: Board) -> bool{
    bfs(board)
}

fn bfs(board:Board) -> bool {
    let mut visited: HashSet<Board> = HashSet::<Board>::with_capacity(4000);
    visited.insert(board.clone());
    
    let mut queue: BinaryHeap<Board> = BinaryHeap::with_capacity(1000);
    queue.push(board);
    
    while !queue.is_empty() {
        let board = queue.pop().unwrap();
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                return true;
            }
            if !visited.contains(&new_board) {
                visited.insert(new_board.clone());
                queue.push(new_board);
            }
        }
    }
    
    false
}