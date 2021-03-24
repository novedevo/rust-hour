use std::{collections::{BTreeSet, BinaryHeap}, process::exit};

use crate::board::Board;

pub fn solve(board: Board) -> Board{
    bfs(board)
}

fn bfs(board:Board) -> Board {
    let mut visited: BTreeSet<Board> = BTreeSet::<Board>::new();
    visited.insert(board.clone());
    
    let mut queue: BinaryHeap<Board> = BinaryHeap::with_capacity(1000);
    queue.push(board.clone());
    
    while !queue.is_empty() {
        let board = queue.pop().unwrap();
        let temp = board.get_moves();
        for new_board in temp {
            println!("ASDASDASDASDASD");
            let _c = 123;
            if new_board.is_solved() {
                return new_board;
            }
            else if visited.contains(&new_board) {
                let _a = visited.get(&new_board);
                let _b = 12;
                println!("{:?}", new_board);
                exit(0)
                // visited.insert(new_board.clone());
                // queue.push(new_board.clone());
            }
        }
    }
    
    panic!("at the disco!");
}