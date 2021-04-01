// use ahash::AHashSet; //10x faster than the default at the expense of some cryptographic defenses
use crate::board::Board;
use rustc_hash::FxHashSet;

use std::io::BufWriter;
use std::{fs::File, io::Write, path::Path};

pub fn solve(input_path: &str, output_path: &str) {
    let board = Board::from_path(input_path);

    let output_path = Path::new(output_path);
    let out_file =
        File::create(output_path).expect("Could not create file. Does the parent folder exist?");
    let mut out_file = BufWriter::new(out_file);
    write!(out_file, "{}", dfs(board)).expect("Error writing output buffer.");
    out_file.flush().expect("Error flushing output buffer");
}

pub fn stress_solve(board: Board) {
    for _ in 0..100 {
        dfs(board.clone());
    }
}

//dfs is twice as fast as bfs.
//A* is in between.
fn dfs(board: Board) -> Board {
    let mut visited: FxHashSet<[[u8; 6]; 6]> = FxHashSet::default(); //keep track of all the nodes we have visited to avoid backtracking
    visited.reserve(5000);
    visited.insert(board.board_u8s);

    let mut stack: Vec<Board> = vec![board]; //keep track of all the nodes we know exist, but have yet to visit
                                             //stack ensures that we are using depth-first search, which we found to be the fastest algorithm
                                             // preallocating space is actually slower here, regardless of how much we allocate

    while let Some(mut board) = stack.pop() {
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                return new_board;
            } else if !visited.contains(&new_board.board_u8s) {
                visited.insert(new_board.board_u8s);
                stack.push(new_board);
            }
        }
    }

    unreachable!("No solution exists.");
}
