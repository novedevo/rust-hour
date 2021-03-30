// use std::collections::{BinaryHeap, VecDeque};

use ahash::AHashSet; //10x faster than the default at the expense of some cryptographic defenses

use crate::board::Board;

//returns the solved board and a number indicating how many distinct states were checked
// pub fn solve(board: Board) -> (Board, usize) {
//     //wrapper
//     _dfs(&board)
// }

pub fn stress_solve(board: Board) {
    //wrapper
    for _ in 0..100 {
        _dfs(&board);
    }
}

//dfs is twice as fast as bfs.
//A* is in between.
fn _dfs(board: &Board) -> (Board, usize) {
    let mut visited: AHashSet<[[char; 6]; 6]> = AHashSet::new(); //keep track of all the nodes we have visited to avoid backtracking
    visited.insert(board.board_chars);

    let mut stack: Vec<Board> = vec![];
    stack.push(board.clone());

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

// fn _astar(board: &Board) -> (Board, usize) {
//     let mut open_set = BinaryHeap::<Board>::new();
//     open_set.push(board.clone());

//     let mut visited: AHashSet<[[char; 6]; 6]> = AHashSet::new();
//     visited.insert(board.board_chars);

//     while let Some(current) = open_set.pop() {
//         for neighbour in current.get_moves() {
            
//             if !visited.contains(&neighbour.board_chars) {
//                 if neighbour.is_solved() {
//                     return (neighbour, visited.len());
//                 }

//                 visited.insert(neighbour.board_chars);
//                 open_set.push(neighbour.clone())
//             }
//         }
//     }
//     panic!("at the astarsco")
// }
