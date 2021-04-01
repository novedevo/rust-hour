mod board;
mod solver;

use board::Board;
use std::thread;

fn main() {
    let mut threads = vec![];
    for entry in std::fs::read_dir("puzzles").unwrap() {
        let new_thread = thread::spawn(|| {
            let entry = entry.unwrap();
            let a = Board::from_path(entry.path().to_str().unwrap());
            solver::stress_solve(a);
        });
        threads.push(new_thread);
    }
    
    
    solver::solve("puzzles/B19.txt", "solutions/B19.txt");

    for handle in threads {
        handle.join().unwrap();
    }
}
