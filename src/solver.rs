use crate::board::Board; //need to use our board!
use rustc_hash::FxHashSet; //10x faster than the default at the expense of some cryptographic defenses

use std::io::BufWriter; //efficient writing to files
use std::{fs::File, io::Write, path::Path}; //filesystem nonsense

//The Function(tm) that the assignment wants. Takes two paths, input file in the first and output file in the second.
//does what it says on the tin
pub fn solve(input_path: &str, output_path: &str) {
    let board = Board::from_path(input_path);

    let mut out_file = BufWriter::new(
        File::create(Path::new(output_path))
            .expect("Could not create file. Does the parent folder exist?"),
    );
    write!(out_file, "{}", dfs(board)).expect("Error writing output buffer.");
    out_file.flush().expect("Error flushing output buffer");
}

//used for benchmarking, etc
//solves the given board 100x before returning
//theoretically, rustc could optimize this out since it has no effects, but it seems that it doesn't
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
    visited.insert(board.array);

    let mut stack: Vec<Board> = vec![board]; //keep track of all the nodes we know exist, but have yet to visit
                                             //stack ensures that we are using depth-first search, which we found to be the fastest algorithm
                                             // preallocating space is actually slower here, regardless of how much we allocate

    //iterate across the entire stack
    //this is a pretty standard DFS implementation
    while let Some(mut board) = stack.pop() {
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                return new_board;
            } else if !visited.contains(&new_board.array) {
                visited.insert(new_board.array);
                stack.push(new_board);
            }
        }
    }

    unreachable!("No solution exists.");
}
