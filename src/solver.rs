use ahash::AHashSet; //10x faster than the default at the expense of some cryptographic defenses

use crate::board::Board;

use std::io::BufWriter;
use std::{fs::File, io::Write, path::Path};

pub fn solve(input_path: &str, output_path: &str) {
    let board = Board::from_str(input_path);

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
    let mut visited: AHashSet<[[char; 6]; 6]> = AHashSet::with_capacity(10000); //keep track of all the nodes we have visited to avoid backtracking
    visited.insert(board.board_chars);

    let mut stack: Vec<Board> = vec![]; //keep track of all the nodes we know exist, but have yet to visit
                                        //stack ensures that we are using depth-first search, which we found to be the fastest algorithm
                                        // preallocating space is actually slower here, regardless of how much we allocate
    stack.push(board);

    while let Some(mut board) = stack.pop() {
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                return new_board;
            } else if !visited.contains(&new_board.board_chars) {
                visited.insert(new_board.board_chars);
                stack.push(new_board);
            }
        }
    }

    panic!("No solution exists.");
}
