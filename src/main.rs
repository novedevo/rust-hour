mod board;
mod solver;

use board::Board;
use std::thread;

fn main() {
    let mut threads = vec![];
    for entry in std::fs::read_dir("puzzles").unwrap() {
        let new_thread = thread::spawn(|| {
            let entry = entry.unwrap();
            let a = Board::from_str(entry.path().to_str().unwrap());
            solver::stress_solve(a.clone());
        });
        threads.push(new_thread);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}
